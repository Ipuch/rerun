use std::sync::{
    atomic::{AtomicU64, Ordering},
    OnceLock,
};

use ahash::HashMap;
use arrow2::{
    array::Array as ArrowArray, chunk::Chunk as ArrowChunk, datatypes::Schema as ArrowSchema,
};
use itertools::Itertools as _;

use nohash_hasher::IntMap;
use re_chunk::{Chunk, ComponentName, RangeQuery, RowId, TimeInt, Timeline};
use re_chunk_store::{
    ColumnDescriptor, ColumnSelector, ComponentColumnDescriptor, ComponentColumnSelector,
    ControlColumnDescriptor, ControlColumnSelector, JoinEncoding, QueryExpression2,
    TimeColumnDescriptor, TimeColumnSelector,
};
use re_log_types::ResolvedTimeRange;

use crate::{QueryEngine, RecordBatch};

// ---

// TODO(cmc): (no specific order) (should we make issues for these?)
// * [x] basic thing working
// * [x] custom selection
// * [x] support for overlaps (slow)
// * [x] pagination (any solution, even a slow one)
// * [ ] overlaps (less dumb)
// * [ ] selector-based `filtered_index`
// * [ ] clears
// * [ ] latestat sparse-filling
// * [ ] pagination (fast)
// * [ ] pov support
// * [ ] sampling support
// * [ ] configurable cache bypass
// * [ ] allocate null arrays once
// * [ ] take kernel duplicates all memory
// * [ ] dedupe-latest without allocs/copies

/// A handle to a dataframe query, ready to be executed.
///
/// Cheaply created via [`QueryEngine::query`].
///
/// See [`QueryHandle::next_row`] or [`QueryHandle::into_iter`].
pub struct QueryHandle<'a> {
    /// Handle to the [`QueryEngine`].
    pub(crate) engine: &'a QueryEngine<'a>,

    /// The original query expression used to instantiate this handle.
    pub(crate) query: QueryExpression2,

    /// Internal private state. Lazily computed.
    ///
    /// It is important that handles stay cheap to create.
    state: OnceLock<QueryHandleState>,
}

/// Internal private state. Lazily computed.
struct QueryHandleState {
    /// Describes the columns that make up this view.
    ///
    /// See [`QueryExpression2::view_contents`].
    view_contents: Vec<ColumnDescriptor>,

    /// Describes the columns specifically selected to be returned from this view.
    ///
    /// All returned rows will have an Arrow schema that matches this selection.
    ///
    /// Columns that do not yield any data will still be present in the results, filled with null values.
    ///
    /// See also [`QueryHandleState::arrow_schema`].
    selected_contents: Vec<ColumnDescriptor>,

    /// The Arrow schema that corresponds to the `selected_contents`.
    ///
    /// All returned rows will have this schema.
    arrow_schema: ArrowSchema,

    /// All the [`Chunk`]s included in the view contents.
    ///
    /// These are already sorted, densified, vertically sliced, and [latest-deduped] according
    /// to the query.
    ///
    /// The atomic counter is used as a cursor which keeps track of our current position within
    /// each individual chunk.
    /// Because chunks are allowed to overlap, we might need to rebound between two or more chunks
    /// during our iteration.
    ///
    /// [latest-deduped]: [`Chunk::deduped_latest_on_index`]
    chunks: HashMap<ComponentName, Vec<(AtomicU64, Chunk)>>,

    /// Tracks the current row index: the position of the iterator. For [`QueryHandle::next_row`].
    ///
    /// This represents the number of rows that the caller has iterated on: it is completely
    /// unrelated to the cursors used to track the current position in each individual chunk.
    cur_row: AtomicU64,
}

impl<'a> QueryHandle<'a> {
    pub(crate) fn new(engine: &'a QueryEngine<'a>, query: QueryExpression2) -> Self {
        Self {
            engine,
            query,
            state: Default::default(),
        }
    }
}

impl QueryHandle<'_> {
    /// Lazily initialize internal private state.
    ///
    /// It is important that query handles stay cheap to create.
    fn init(&self) -> &QueryHandleState {
        self.state.get_or_init(|| {
            re_tracing::profile_scope!("init");

            // 1. Compute the schema of the view contents.
            let view_contents = if let Some(view_contents) = self.query.view_contents.as_ref() {
                self.engine
                    .store
                    .schema()
                    .into_iter()
                    .filter(|column| match column {
                        ColumnDescriptor::Control(_) | ColumnDescriptor::Time(_) => true,
                        ColumnDescriptor::Component(column) => view_contents
                            .get(&column.entity_path)
                            .map_or(false, |components| {
                                components.as_ref().map_or(true, |components| {
                                    components.contains(&column.component_name)
                                })
                            }),
                    })
                    .collect_vec()
            } else {
                self.engine.store.schema()
            };

            // 2. Compute the schema of the selected contents.
            //
            // The caller might have selected columns that do not exist in the view: they should
            // still appear in the results.
            let selected_contents = if let Some(selection) = self.query.selection.as_ref() {
                 selection
                    .iter()
                    .map(|column| {
                        match column {
                            ColumnSelector::Control(selected_column) => {
                                let ControlColumnSelector { component: selected_component_name } = selected_column;

                                view_contents
                                    .iter()
                                    .filter_map(|view_column| match view_column {
                                        ColumnDescriptor::Control(view_descr) => Some(view_descr),
                                        _ => None,
                                    })
                                    .find(|view_descr| view_descr.component_name == *selected_component_name)
                                    .cloned()
                                    .map_or_else(
                                        || ColumnDescriptor::Control(ControlColumnDescriptor {
                                            component_name: *selected_component_name,
                                            datatype: arrow2::datatypes::DataType::Null,
                                        }),
                                        ColumnDescriptor::Control,
                                    )
                            },

                            ColumnSelector::Time(selected_column) => {
                                let TimeColumnSelector { timeline: selected_timeline } = selected_column;

                                view_contents
                                    .iter()
                                    .filter_map(|view_column| match view_column {
                                        ColumnDescriptor::Time(view_descr) => Some(view_descr),
                                        _ => None,
                                    })
                                    .find(|view_descr| *view_descr.timeline.name() == *selected_timeline)
                                    .cloned()
                                    .map_or_else(
                                        || ColumnDescriptor::Time(TimeColumnDescriptor {
                                            // TODO(cmc): I picked a sequence here because I have to pick something.
                                            // It doesn't matter, only the name will remain in the Arrow schema anyhow.
                                            timeline: Timeline::new_sequence(*selected_timeline),
                                            datatype: arrow2::datatypes::DataType::Null,
                                        }),
                                        ColumnDescriptor::Time,
                                    )
                            },

                            ColumnSelector::Component(selected_column) => {
                                let ComponentColumnSelector {
                                    entity_path: selected_entity_path,
                                    component: selected_component_name,
                                    join_encoding: _
                                } = selected_column;

                                view_contents
                                    .iter()
                                    .filter_map(|view_column| match view_column {
                                        ColumnDescriptor::Component(view_descr) => Some(view_descr),
                                        _ => None,
                                    })
                                    .find(|view_descr| {
                                        view_descr.entity_path == *selected_entity_path
                                        && view_descr.component_name == *selected_component_name
                                    })
                                    .cloned()
                                    .map_or_else(
                                        || ColumnDescriptor::Component(ComponentColumnDescriptor {
                                            entity_path: selected_entity_path.clone(),
                                            archetype_name: None,
                                            archetype_field_name: None,
                                            component_name: *selected_component_name,
                                            store_datatype: arrow2::datatypes::DataType::Null,
                                            join_encoding: JoinEncoding::default(),
                                            is_static: false,
                                        }),
                                        ColumnDescriptor::Component,
                                    )
                            },
                        }
                    }).collect_vec()
            } else {
                view_contents.clone()
            };

            // 3. Compute the Arrow schema of the selected components.
            //
            // Every result returned using this `QueryHandle` will match this schema exactly.
            let arrow_schema =
                ArrowSchema {
                    fields: selected_contents
                        .iter()
                        .map(|descr| descr.to_arrow_field())
                        .collect_vec(),
                    metadata: Default::default(),
                };

            // 4. Perform the query and keep track of all the relevant chunks.
            let chunks = {
                let index_range = self
                    .query
                    .filtered_index_range
                    .unwrap_or(ResolvedTimeRange::EVERYTHING);

                let query = RangeQuery::new(self.query.filtered_index, index_range)
                    .keep_extra_timelines(true) // we want all the timelines we can get!
                    .keep_extra_components(false);

                selected_contents
                    .iter()
                    .filter_map(|column| match column {
                        ColumnDescriptor::Control(_) | ColumnDescriptor::Time(_) => None,
                        ColumnDescriptor::Component(column) => Some(column),
                    })
                    .filter_map(|column| {
                        // NOTE: Keep in mind that the range APIs natively make sure that we will
                        // either get a bunch of relevant _static_ chunks, or a bunch of relevant
                        // _temporal_ chunks, but never both.
                        //
                        // TODO(cmc): Going through the cache is very useful in a Viewer context, but
                        // not so much in an SDK context. Make it configurable.
                        let results = self.engine.cache.range(
                            self.engine.store,
                            &query,
                            &column.entity_path,
                            [column.component_name],
                        );

                        debug_assert!(
                            results.components.len() <= 1,
                            "cannot possibly get more than one component with this query"
                        );

                        results
                            .components
                            .into_iter()
                            .next()
                            .map(|(component_name, chunks)| {
                                let chunks = chunks
                                    .into_iter()
                                    .map(|chunk| {
                                        // NOTE: Keep in mind that the range APIs would have already taken care
                                        // of A) sorting the chunk on the `filtered_index` (and row-id) and
                                        // B) densifying it according to the current `component_name`.
                                        // Both of these are mandatory requirements for the deduplication logic to
                                        // do what we want: keep the latest known value for `component_name` at all
                                        // remaining unique index values all while taking row-id ordering semantics
                                        // into account.
                                        debug_assert!(
                                            chunk.is_sorted(),
                                            "the query cache should have already taken care of sorting (and densifying!) the chunk",
                                        );

                                        let chunk = chunk.deduped_latest_on_index(&self.query.filtered_index);


                                        (AtomicU64::default(), chunk)
                                    })
                                    .collect_vec();
                                (component_name, chunks)
                            })
                    })
                    .collect()
            };

            QueryHandleState {
                view_contents,
                selected_contents,
                arrow_schema,
                chunks,
                cur_row: AtomicU64::new(0),
            }
        })
    }

    /// The query used to instantiate this handle.
    pub fn query(&self) -> &QueryExpression2 {
        &self.query
    }

    /// Describes the columns that make up this view.
    ///
    /// See [`QueryExpression2::view_contents`].
    pub fn view_contents(&self) -> &[ColumnDescriptor] {
        &self.init().view_contents
    }

    /// The Arrow schema that corresponds to the `selected_contents`.
    ///
    /// All returned rows will have this schema.
    pub fn selected_contents(&self) -> &[ColumnDescriptor] {
        &self.init().selected_contents
    }

    /// All results returned by this handle will strictly follow this Arrow schema.
    ///
    /// Columns that do not yield any data will still be present in the results, filled with null values.
    pub fn schema(&self) -> &ArrowSchema {
        &self.init().arrow_schema
    }

    /// Returns the next row's worth of data.
    ///
    /// The returned vector of Arrow arrays strictly follows the schema specified by [`Self::schema`].
    /// Columns that do not yield any data will still be present in the results, filled with null values.
    ///
    /// Each cell in the result corresponds to the latest _locally_ known value at that particular point in
    /// the index, for each respective `ColumnDescriptor`.
    /// See [`QueryExpression2::sparse_fill_strategy`] to go beyond local resolution.
    ///
    /// Example:
    /// ```ignore
    /// while let Some(row) = query_handle.next_row() {
    ///     // …
    /// }
    /// ```
    ///
    /// ## Pagination
    ///
    /// This does not offer any kind of native pagination yet.
    ///
    /// To emulate pagination from user-space, use the `Iterator` API:
    /// ```ignore
    /// for row in query_handle.into_iter().skip(offset).take(len) {
    ///     // …
    /// }
    /// ```
    //
    // TODO(cmc): better/actual pagination
    pub fn next_row(&mut self) -> Option<Vec<Box<dyn ArrowArray>>> {
        re_tracing::profile_function!();

        /// Temporary state used to resolve the streaming join for the current iteration.
        struct StreamingJoinState<'a> {
            /// Which `Chunk` is this?
            chunk: &'a Chunk,

            /// How far are we into this `Chunk`?
            cursor: &'a AtomicU64,

            /// What's the index value at the current cursor?
            index_value: TimeInt,

            /// What's the `RowId` at the current cursor?
            row_id: RowId,
        }

        let state = self.init();

        let _cur_row = state.cur_row.fetch_add(1, Ordering::Relaxed);

        // First, we need to find, among all the chunks available in the current view contents,
        // what is their index value for the current row?
        let mut streaming_state_per_component: IntMap<ComponentName, StreamingJoinState<'_>> =
            IntMap::default();
        for (component_name, chunks) in &state.chunks {
            for (cur_cursor, cur_chunk) in chunks {
                // NOTE: Too soon to increment the cursor, we cannot know yet which chunks will or
                // will not be part of the current row.
                let cursor_value = cur_cursor.load(Ordering::Relaxed) as usize;

                // TODO(cmc): This can easily be optimized by looking ahead and breaking as soon as chunks
                // stop overlapping.

                let Some(cur_row_id) = cur_chunk.row_ids().nth(cursor_value) else {
                    continue;
                };

                let Some(cur_index_value) = cur_chunk
                    .timelines()
                    .get(&self.query.filtered_index)
                    .map_or(Some(TimeInt::STATIC), |time_column| {
                        time_column
                            .times_raw()
                            .get(cursor_value)
                            .copied()
                            .map(TimeInt::new_temporal)
                    })
                else {
                    continue;
                };

                streaming_state_per_component
                    .entry(*component_name)
                    .and_modify(|streaming_state| {
                        let StreamingJoinState {
                            chunk,
                            cursor,
                            index_value,
                            row_id,
                        } = streaming_state;

                        let cur_chunk_has_smaller_index_value = cur_index_value < *index_value;
                        // If these two chunks overlap and share the index value of the current
                        // iteration, we shall pick the row with the most recent row-id.
                        let cur_chunk_has_equal_index_but_higher_rowid =
                            cur_index_value == *index_value && cur_row_id > *row_id;

                        if cur_chunk_has_smaller_index_value
                            || cur_chunk_has_equal_index_but_higher_rowid
                        {
                            *chunk = chunk;
                            *cursor = cursor;
                            *index_value = cur_index_value;
                            *row_id = cur_row_id;
                        }
                    })
                    .or_insert_with(|| StreamingJoinState {
                        chunk: cur_chunk,
                        cursor: cur_cursor,
                        index_value: cur_index_value,
                        row_id: cur_row_id,
                    });
            }
        }

        // What's the index value we're looking for at the current iteration?
        let cur_index_value = streaming_state_per_component
            .values()
            // NOTE: We're purposefully ignoring RowId-related semantics here: we just want to know
            // the value we're looking for on the "main" index (dedupe semantics).
            .min_by_key(|streaming_state| streaming_state.index_value)
            .map(|streaming_state| streaming_state.index_value)?;

        streaming_state_per_component.retain(|_component_name, streaming_state| {
            streaming_state.index_value == cur_index_value
        });

        // The most recent chunk in the current iteration, according to RowId semantics.
        let cur_most_recent_chunk = streaming_state_per_component
            .values()
            .max_by_key(|streaming_state| streaming_state.row_id)?;

        // We are stitching a bunch of unrelated cells together in order to create the final row
        // that is being returned.
        //
        // For this reason, we can only guarantee that the index being explicitly queried for
        // (`QueryExpression2::filtered_index`) will match for all these cells.
        //
        // When it comes to other indices that the caller might have asked for, it is possible that
        // these different cells won't share the same values (e.g. two cells were found at
        // `log_time=100`, but one of them has `frame=3` and the other `frame=5`, for whatever
        // reason).
        // In order to deal with this, we keep track of the maximum value for every possible index
        // within the returned set of cells, and return that.
        //
        // TODO(cmc): In the future, it would be nice to make that either configurable, e.g.:
        // * return the minimum value instead of the max
        // * return the exact value for each component (that would be a _lot_ of columns!)
        // * etc
        let mut max_value_per_index = IntMap::default();
        {
            // Unless we are currently iterating over a static row, then we know for sure that the
            // timeline being used as `filtered_index` is A) present and B) has for value `cur_index_value`.
            if cur_index_value != TimeInt::STATIC {
                let slice = cur_most_recent_chunk
                    .chunk
                    .timelines()
                    .get(&self.query.filtered_index)
                    .map(|time_column| {
                        time_column.times_array().sliced(
                            cur_most_recent_chunk.cursor.load(Ordering::Relaxed) as usize,
                            1,
                        )
                    });

                debug_assert!(
                    slice.is_some(),
                    "Timeline must exist, otherwise the query engine would have never returned that chunk in the first place",
                );

                // NOTE: Cannot fail, just want to stay away from unwraps.
                if let Some(slice) = slice {
                    max_value_per_index.insert(self.query.filtered_index, (cur_index_value, slice));
                }
            }

            streaming_state_per_component
                .values()
                .flat_map(|streaming_state| {
                    streaming_state
                        .chunk
                        .timelines()
                        .values()
                        // NOTE: Already took care of that one above.
                        .filter(|time_column| *time_column.timeline() != self.query.filtered_index)
                        // NOTE: Cannot fail, just want to stay away from unwraps.
                        .filter_map(|time_column| {
                            let cursor = streaming_state.cursor.load(Ordering::Relaxed) as usize;
                            time_column
                                .times_raw()
                                .get(cursor)
                                .copied()
                                .map(TimeInt::new_temporal)
                                .map(|time| {
                                    (
                                        *time_column.timeline(),
                                        (time, time_column.times_array().sliced(cursor, 1)),
                                    )
                                })
                        })
                })
                .for_each(|(timeline, (time, time_sliced))| {
                    max_value_per_index
                        .entry(timeline)
                        .and_modify(|(max_time, max_time_sliced)| {
                            if time > *max_time {
                                *max_time = time;
                                *max_time_sliced = time_sliced.clone();
                            }
                        })
                        .or_insert((time, time_sliced));
                });
        }

        let sliced_arrays: IntMap<_, _> = streaming_state_per_component
            .iter()
            .filter_map(|(component_name, streaming_state)| {
                let cursor = streaming_state.cursor.fetch_add(1, Ordering::Relaxed);

                let list_array = streaming_state
                    .chunk
                    .components()
                    .get(component_name)
                    .map(|list_array| list_array.sliced(cursor as usize, 1));

                debug_assert!(
                    list_array.is_some(),
                    "This must exist or the chunk wouldn't have been sliced to start with."
                );

                // NOTE: This cannot possibly return None, see assert above.
                list_array.map(|list_array| (*component_name, list_array))
            })
            .collect();

        // TODO(cmc): It would likely be worth it to allocate all these possible
        // null-arrays ahead of time, and just return a pointer to those in the failure
        // case here.
        let arrays = state
            .view_contents
            .iter()
            .map(|column| match column {
                ColumnDescriptor::Control(_) => cur_most_recent_chunk.chunk.row_ids_array().sliced(
                    cur_most_recent_chunk
                        .cursor
                        .load(Ordering::Relaxed)
                        // NOTE: We did the cursor increments while computing the final sliced arrays,
                        // so we need to go back one tick for this.
                        .saturating_sub(1) as usize,
                    1,
                ),

                ColumnDescriptor::Time(descr) => {
                    max_value_per_index.remove(&descr.timeline).map_or_else(
                        || arrow2::array::new_null_array(column.datatype(), 1),
                        |(_time, time_sliced)| time_sliced,
                    )
                }

                ColumnDescriptor::Component(descr) => {
                    sliced_arrays.get(&descr.component_name).map_or_else(
                        || arrow2::array::new_null_array(column.datatype(), 1),
                        |list_array| list_array.clone(),
                    )
                }
            })
            .collect_vec();

        debug_assert_eq!(state.arrow_schema.fields.len(), arrays.len());

        Some(arrays)
    }

    /// Calls [`Self::next_row`] and wraps the result in a [`RecordBatch`].
    ///
    /// Only use this if you absolutely need a [`RecordBatch`] as this adds a lot of allocation
    /// overhead.
    ///
    /// See [`Self::next_row`] for more information.
    #[inline]
    pub fn next_row_batch(&mut self) -> Option<RecordBatch> {
        Some(RecordBatch {
            schema: self.schema().clone(),
            data: ArrowChunk::new(self.next_row()?),
        })
    }
}

impl<'a> QueryHandle<'a> {
    /// Returns an iterator backed by [`Self::next_row`].
    #[allow(clippy::should_implement_trait)] // we need an anonymous closure, this won't work
    pub fn into_iter(mut self) -> impl Iterator<Item = Vec<Box<dyn ArrowArray>>> + 'a {
        std::iter::from_fn(move || self.next_row())
    }

    /// Returns an iterator backed by [`Self::next_row_batch`].
    #[allow(clippy::should_implement_trait)] // we need an anonymous closure, this won't work
    pub fn into_batch_iter(mut self) -> impl Iterator<Item = RecordBatch> + 'a {
        std::iter::from_fn(move || self.next_row_batch())
    }
}
