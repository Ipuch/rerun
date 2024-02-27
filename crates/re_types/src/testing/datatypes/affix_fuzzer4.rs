// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/testing/datatypes/fuzzy.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

#[derive(Clone, Debug, PartialEq)]
pub enum AffixFuzzer4 {
    SingleRequired(crate::testing::datatypes::AffixFuzzer3),
    ManyRequired(Vec<crate::testing::datatypes::AffixFuzzer3>),
    ManyOptional(Option<Vec<crate::testing::datatypes::AffixFuzzer3>>),
}

impl ::re_types_core::SizeBytes for AffixFuzzer4 {
    #[allow(clippy::match_same_arms)]
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        match self {
            Self::SingleRequired(v) => v.heap_size_bytes(),
            Self::ManyRequired(v) => v.heap_size_bytes(),
            Self::ManyOptional(v) => v.heap_size_bytes(),
        }
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::testing::datatypes::AffixFuzzer3>::is_pod()
            && <Vec<crate::testing::datatypes::AffixFuzzer3>>::is_pod()
            && <Option<Vec<crate::testing::datatypes::AffixFuzzer3>>>::is_pod()
    }
}

::re_types_core::macros::impl_into_cow!(AffixFuzzer4);

impl ::re_types_core::Loggable for AffixFuzzer4 {
    type Name = ::re_types_core::DatatypeName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.testing.datatypes.AffixFuzzer4".into()
    }

    #[allow(clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use arrow2::datatypes::*;
        DataType::Union(
            std::sync::Arc::new(vec![
                Field {
                    name: "_null_markers".to_owned(),
                    data_type: DataType::Null,
                    is_nullable: true,
                    metadata: [].into(),
                },
                Field {
                    name: "single_required".to_owned(),
                    data_type: <crate::testing::datatypes::AffixFuzzer3>::arrow_datatype(),
                    is_nullable: false,
                    metadata: [].into(),
                },
                Field {
                    name: "many_required".to_owned(),
                    data_type: DataType::List(std::sync::Arc::new(Field {
                        name: "item".to_owned(),
                        data_type: <crate::testing::datatypes::AffixFuzzer3>::arrow_datatype(),
                        is_nullable: false,
                        metadata: [].into(),
                    })),
                    is_nullable: false,
                    metadata: [].into(),
                },
                Field {
                    name: "many_optional".to_owned(),
                    data_type: DataType::List(std::sync::Arc::new(Field {
                        name: "item".to_owned(),
                        data_type: <crate::testing::datatypes::AffixFuzzer3>::arrow_datatype(),
                        is_nullable: false,
                        metadata: [].into(),
                    })),
                    is_nullable: false,
                    metadata: [].into(),
                },
            ]),
            Some(std::sync::Arc::new(vec![0i32, 1i32, 2i32, 3i32])),
            UnionMode::Dense,
        )
    }

    #[allow(clippy::wildcard_imports)]
    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, datatypes::*};
        Ok({
            let data: Vec<_> = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    datum
                })
                .collect();
            let types = data
                .iter()
                .map(|a| match a.as_deref() {
                    None => 0,
                    Some(AffixFuzzer4::SingleRequired(_)) => 1i8,
                    Some(AffixFuzzer4::ManyRequired(_)) => 2i8,
                    Some(AffixFuzzer4::ManyOptional(_)) => 3i8,
                })
                .collect();
            let fields = vec![
                NullArray::new(DataType::Null, data.iter().filter(|v| v.is_none()).count()).boxed(),
                {
                    let (somes, single_required): (Vec<_>, Vec<_>) = data
                        .iter()
                        .filter(|datum| {
                            matches!(datum.as_deref(), Some(AffixFuzzer4::SingleRequired(_)))
                        })
                        .map(|datum| {
                            let datum = match datum.as_deref() {
                                Some(AffixFuzzer4::SingleRequired(v)) => Some(v.clone()),
                                _ => None,
                            };
                            (datum.is_some(), datum)
                        })
                        .unzip();
                    let single_required_bitmap: Option<arrow2::bitmap::Bitmap> = {
                        let any_nones = somes.iter().any(|some| !*some);
                        any_nones.then(|| somes.into())
                    };
                    {
                        _ = single_required_bitmap;
                        crate::testing::datatypes::AffixFuzzer3::to_arrow_opt(single_required)?
                    }
                },
                {
                    let (somes, many_required): (Vec<_>, Vec<_>) = data
                        .iter()
                        .filter(|datum| {
                            matches!(datum.as_deref(), Some(AffixFuzzer4::ManyRequired(_)))
                        })
                        .map(|datum| {
                            let datum = match datum.as_deref() {
                                Some(AffixFuzzer4::ManyRequired(v)) => Some(v.clone()),
                                _ => None,
                            };
                            (datum.is_some(), datum)
                        })
                        .unzip();
                    let many_required_bitmap: Option<arrow2::bitmap::Bitmap> = {
                        let any_nones = somes.iter().any(|some| !*some);
                        any_nones.then(|| somes.into())
                    };
                    {
                        use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                        let many_required_inner_data: Vec<_> = many_required
                            .iter()
                            .flatten()
                            .flatten()
                            .cloned()
                            .map(Some)
                            .collect();
                        let many_required_inner_bitmap: Option<arrow2::bitmap::Bitmap> = None;
                        let offsets = arrow2::offset::Offsets::<i32>::try_from_lengths(
                            many_required.iter().map(|opt| {
                                opt.as_ref().map(|datum| datum.len()).unwrap_or_default()
                            }),
                        )
                        .unwrap()
                        .into();
                        ListArray::new(
                            DataType::List(std::sync::Arc::new(Field {
                                name: "item".to_owned(),
                                data_type:
                                    <crate::testing::datatypes::AffixFuzzer3>::arrow_datatype(),
                                is_nullable: false,
                                metadata: [].into(),
                            })),
                            offsets,
                            {
                                _ = many_required_inner_bitmap;
                                crate::testing::datatypes::AffixFuzzer3::to_arrow_opt(
                                    many_required_inner_data,
                                )?
                            },
                            many_required_bitmap,
                        )
                        .boxed()
                    }
                },
                {
                    let (somes, many_optional): (Vec<_>, Vec<_>) = data
                        .iter()
                        .filter(|datum| {
                            matches!(datum.as_deref(), Some(AffixFuzzer4::ManyOptional(_)))
                        })
                        .map(|datum| {
                            let datum = match datum.as_deref() {
                                Some(AffixFuzzer4::ManyOptional(v)) => Some(v.clone()),
                                _ => None,
                            }
                            .flatten();
                            (datum.is_some(), datum)
                        })
                        .unzip();
                    let many_optional_bitmap: Option<arrow2::bitmap::Bitmap> = {
                        let any_nones = somes.iter().any(|some| !*some);
                        any_nones.then(|| somes.into())
                    };
                    {
                        use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                        let many_optional_inner_data: Vec<_> = many_optional
                            .iter()
                            .flatten()
                            .flatten()
                            .cloned()
                            .map(Some)
                            .collect();
                        let many_optional_inner_bitmap: Option<arrow2::bitmap::Bitmap> = None;
                        let offsets = arrow2::offset::Offsets::<i32>::try_from_lengths(
                            many_optional.iter().map(|opt| {
                                opt.as_ref().map(|datum| datum.len()).unwrap_or_default()
                            }),
                        )
                        .unwrap()
                        .into();
                        ListArray::new(
                            DataType::List(std::sync::Arc::new(Field {
                                name: "item".to_owned(),
                                data_type:
                                    <crate::testing::datatypes::AffixFuzzer3>::arrow_datatype(),
                                is_nullable: false,
                                metadata: [].into(),
                            })),
                            offsets,
                            {
                                _ = many_optional_inner_bitmap;
                                crate::testing::datatypes::AffixFuzzer3::to_arrow_opt(
                                    many_optional_inner_data,
                                )?
                            },
                            many_optional_bitmap,
                        )
                        .boxed()
                    }
                },
            ];
            let offsets = Some({
                let mut single_required_offset = 0;
                let mut many_required_offset = 0;
                let mut many_optional_offset = 0;
                let mut nulls_offset = 0;
                data.iter()
                    .map(|v| match v.as_deref() {
                        None => {
                            let offset = nulls_offset;
                            nulls_offset += 1;
                            offset
                        }
                        Some(AffixFuzzer4::SingleRequired(_)) => {
                            let offset = single_required_offset;
                            single_required_offset += 1;
                            offset
                        }
                        Some(AffixFuzzer4::ManyRequired(_)) => {
                            let offset = many_required_offset;
                            many_required_offset += 1;
                            offset
                        }
                        Some(AffixFuzzer4::ManyOptional(_)) => {
                            let offset = many_optional_offset;
                            many_optional_offset += 1;
                            offset
                        }
                    })
                    .collect()
            });
            UnionArray::new(
                <crate::testing::datatypes::AffixFuzzer4>::arrow_datatype(),
                types,
                fields,
                offsets,
            )
            .boxed()
        })
    }

    #[allow(clippy::wildcard_imports)]
    fn from_arrow_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, buffer::*, datatypes::*};
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<arrow2::array::UnionArray>()
                .ok_or_else(|| {
                    let expected = Self::arrow_datatype();
                    let actual = arrow_data.data_type().clone();
                    DeserializationError::datatype_mismatch(expected, actual)
                })
                .with_context("rerun.testing.datatypes.AffixFuzzer4")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let (arrow_data_types, arrow_data_arrays) =
                    (arrow_data.types(), arrow_data.fields());
                let arrow_data_offsets = arrow_data
                    .offsets()
                    .ok_or_else(|| {
                        let expected = Self::arrow_datatype();
                        let actual = arrow_data.data_type().clone();
                        DeserializationError::datatype_mismatch(expected, actual)
                    })
                    .with_context("rerun.testing.datatypes.AffixFuzzer4")?;
                if arrow_data_types.len() != arrow_data_offsets.len() {
                    return Err(DeserializationError::offset_slice_oob(
                        (0, arrow_data_types.len()),
                        arrow_data_offsets.len(),
                    ))
                    .with_context("rerun.testing.datatypes.AffixFuzzer4");
                }
                let single_required = {
                    if 1usize >= arrow_data_arrays.len() {
                        return Ok(Vec::new());
                    }
                    let arrow_data = &*arrow_data_arrays[1usize];
                    crate::testing::datatypes::AffixFuzzer3::from_arrow_opt(arrow_data)
                        .with_context("rerun.testing.datatypes.AffixFuzzer4#single_required")?
                        .into_iter()
                        .collect::<Vec<_>>()
                };
                let many_required = {
                    if 2usize >= arrow_data_arrays.len() {
                        return Ok(Vec::new());
                    }
                    let arrow_data = &*arrow_data_arrays[2usize];
                    {
                        let arrow_data = arrow_data
                            .as_any()
                            .downcast_ref::<arrow2::array::ListArray<i32>>()
                            .ok_or_else(|| {
                                let expected = DataType::List(std::sync::Arc::new(Field {
                                    name: "item".to_owned(),
                                    data_type:
                                        <crate::testing::datatypes::AffixFuzzer3>::arrow_datatype(),
                                    is_nullable: false,
                                    metadata: [].into(),
                                }));
                                let actual = arrow_data.data_type().clone();
                                DeserializationError::datatype_mismatch(expected, actual)
                            })
                            .with_context("rerun.testing.datatypes.AffixFuzzer4#many_required")?;
                        if arrow_data.is_empty() {
                            Vec::new()
                        } else {
                            let arrow_data_inner = {
                                let arrow_data_inner = &**arrow_data.values();
                                crate::testing::datatypes::AffixFuzzer3::from_arrow_opt(
                                    arrow_data_inner,
                                )
                                .with_context("rerun.testing.datatypes.AffixFuzzer4#many_required")?
                                .into_iter()
                                .collect::<Vec<_>>()
                            };
                            let offsets = arrow_data.offsets();
                            arrow2::bitmap::utils::ZipValidity::new_with_validity(
                                offsets.iter().zip(offsets.lengths()),
                                arrow_data.validity(),
                            )
                            .map(|elem| {
                                elem.map(|(start, len)| {
                                    let start = *start as usize;
                                    let end = start + len;
                                    if end as usize > arrow_data_inner.len() {
                                        return Err(DeserializationError::offset_slice_oob(
                                            (start, end),
                                            arrow_data_inner.len(),
                                        ));
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    let data = unsafe {
                                        arrow_data_inner.get_unchecked(start as usize..end as usize)
                                    };
                                    let data = data
                                        .iter()
                                        .cloned()
                                        .map(Option::unwrap_or_default)
                                        .collect();
                                    Ok(data)
                                })
                                .transpose()
                            })
                            .collect::<DeserializationResult<Vec<Option<_>>>>()?
                        }
                        .into_iter()
                    }
                    .collect::<Vec<_>>()
                };
                let many_optional = {
                    if 3usize >= arrow_data_arrays.len() {
                        return Ok(Vec::new());
                    }
                    let arrow_data = &*arrow_data_arrays[3usize];
                    {
                        let arrow_data = arrow_data
                            .as_any()
                            .downcast_ref::<arrow2::array::ListArray<i32>>()
                            .ok_or_else(|| {
                                let expected = DataType::List(std::sync::Arc::new(Field {
                                    name: "item".to_owned(),
                                    data_type:
                                        <crate::testing::datatypes::AffixFuzzer3>::arrow_datatype(),
                                    is_nullable: false,
                                    metadata: [].into(),
                                }));
                                let actual = arrow_data.data_type().clone();
                                DeserializationError::datatype_mismatch(expected, actual)
                            })
                            .with_context("rerun.testing.datatypes.AffixFuzzer4#many_optional")?;
                        if arrow_data.is_empty() {
                            Vec::new()
                        } else {
                            let arrow_data_inner = {
                                let arrow_data_inner = &**arrow_data.values();
                                crate::testing::datatypes::AffixFuzzer3::from_arrow_opt(
                                    arrow_data_inner,
                                )
                                .with_context("rerun.testing.datatypes.AffixFuzzer4#many_optional")?
                                .into_iter()
                                .collect::<Vec<_>>()
                            };
                            let offsets = arrow_data.offsets();
                            arrow2::bitmap::utils::ZipValidity::new_with_validity(
                                offsets.iter().zip(offsets.lengths()),
                                arrow_data.validity(),
                            )
                            .map(|elem| {
                                elem.map(|(start, len)| {
                                    let start = *start as usize;
                                    let end = start + len;
                                    if end as usize > arrow_data_inner.len() {
                                        return Err(DeserializationError::offset_slice_oob(
                                            (start, end),
                                            arrow_data_inner.len(),
                                        ));
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    let data = unsafe {
                                        arrow_data_inner.get_unchecked(start as usize..end as usize)
                                    };
                                    let data = data
                                        .iter()
                                        .cloned()
                                        .map(Option::unwrap_or_default)
                                        .collect();
                                    Ok(data)
                                })
                                .transpose()
                            })
                            .collect::<DeserializationResult<Vec<Option<_>>>>()?
                        }
                        .into_iter()
                    }
                    .collect::<Vec<_>>()
                };
                arrow_data_types
                    .iter()
                    .enumerate()
                    .map(|(i, typ)| {
                        let offset = arrow_data_offsets[i];
                        if *typ == 0 {
                            Ok(None)
                        } else {
                            Ok(Some(match typ {
                                1i8 => AffixFuzzer4::SingleRequired({
                                    if offset as usize >= single_required.len() {
                                        return Err(DeserializationError::offset_oob(
                                            offset as _,
                                            single_required.len(),
                                        ))
                                        .with_context(
                                            "rerun.testing.datatypes.AffixFuzzer4#single_required",
                                        );
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    unsafe { single_required.get_unchecked(offset as usize) }
                                        .clone()
                                        .ok_or_else(DeserializationError::missing_data)
                                        .with_context(
                                            "rerun.testing.datatypes.AffixFuzzer4#single_required",
                                        )?
                                }),
                                2i8 => AffixFuzzer4::ManyRequired({
                                    if offset as usize >= many_required.len() {
                                        return Err(DeserializationError::offset_oob(
                                            offset as _,
                                            many_required.len(),
                                        ))
                                        .with_context(
                                            "rerun.testing.datatypes.AffixFuzzer4#many_required",
                                        );
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    unsafe { many_required.get_unchecked(offset as usize) }
                                        .clone()
                                        .ok_or_else(DeserializationError::missing_data)
                                        .with_context(
                                            "rerun.testing.datatypes.AffixFuzzer4#many_required",
                                        )?
                                }),
                                3i8 => AffixFuzzer4::ManyOptional({
                                    if offset as usize >= many_optional.len() {
                                        return Err(DeserializationError::offset_oob(
                                            offset as _,
                                            many_optional.len(),
                                        ))
                                        .with_context(
                                            "rerun.testing.datatypes.AffixFuzzer4#many_optional",
                                        );
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    unsafe { many_optional.get_unchecked(offset as usize) }.clone()
                                }),
                                _ => {
                                    return Err(DeserializationError::missing_union_arm(
                                        Self::arrow_datatype(),
                                        "<invalid>",
                                        *typ as _,
                                    ));
                                }
                            }))
                        }
                    })
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.testing.datatypes.AffixFuzzer4")?
            }
        })
    }
}
