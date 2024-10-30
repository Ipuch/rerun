// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/depth_image.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::external::arrow2;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: A depth image, i.e. as captured by a depth camera.
///
/// Each pixel corresponds to a depth value in units specified by [`components::DepthMeter`][crate::components::DepthMeter].
///
/// ## Example
///
/// ### Depth to 3D example
/// ```ignore
/// use ndarray::{s, Array, ShapeBuilder};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_depth_image_3d").spawn()?;
///
///     let width = 300;
///     let height = 200;
///     let mut image = Array::<u16, _>::from_elem((height, width).f(), 65535);
///     image.slice_mut(s![50..150, 50..150]).fill(20000);
///     image.slice_mut(s![130..180, 100..280]).fill(45000);
///
///     let depth_image = rerun::DepthImage::try_from(image)?
///         .with_meter(10000.0)
///         .with_colormap(rerun::components::Colormap::Viridis);
///
///     // If we log a pinhole camera model, the depth gets automatically back-projected to 3D
///     rec.log(
///         "world/camera",
///         &rerun::Pinhole::from_focal_length_and_resolution(
///             [200.0, 200.0],
///             [width as f32, height as f32],
///         ),
///     )?;
///
///     rec.log("world/camera/depth", &depth_image)?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/depth_image_3d/924e9d4d6a39d63d4fdece82582855fdaa62d15e/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/depth_image_3d/924e9d4d6a39d63d4fdece82582855fdaa62d15e/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/depth_image_3d/924e9d4d6a39d63d4fdece82582855fdaa62d15e/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/depth_image_3d/924e9d4d6a39d63d4fdece82582855fdaa62d15e/1200w.png">
///   <img src="https://static.rerun.io/depth_image_3d/924e9d4d6a39d63d4fdece82582855fdaa62d15e/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq)]
pub struct DepthImage {
    /// The raw depth image data.
    pub buffer: crate::components::ImageBuffer,

    /// The format of the image.
    pub format: crate::components::ImageFormat,

    /// An optional floating point value that specifies how long a meter is in the native depth units.
    ///
    /// For instance: with uint16, perhaps meter=1000 which would mean you have millimeter precision
    /// and a range of up to ~65 meters (2^16 / 1000).
    ///
    /// Note that the only effect on 2D views is the physical depth values shown when hovering the image.
    /// In 3D views on the other hand, this affects where the points of the point cloud are placed.
    pub meter: Option<crate::components::DepthMeter>,

    /// Colormap to use for rendering the depth image.
    ///
    /// If not set, the depth image will be rendered using the Turbo colormap.
    pub colormap: Option<crate::components::Colormap>,

    /// The expected range of depth values.
    ///
    /// This is typically the expected range of valid values.
    /// Everything outside of the range is clamped to the range for the purpose of colormpaping.
    /// Note that point clouds generated from this image will still display all points, regardless of this range.
    ///
    /// If not specified, the range will be automatically estimated from the data.
    /// Note that the Viewer may try to guess a wider range than the minimum/maximum of values
    /// in the contents of the depth image.
    /// E.g. if all values are positive, some bigger than 1.0 and all smaller than 255.0,
    /// the Viewer will guess that the data likely came from an 8bit image, thus assuming a range of 0-255.
    pub depth_range: Option<crate::components::ValueRange>,

    /// Scale the radii of the points in the point cloud generated from this image.
    ///
    /// A fill ratio of 1.0 (the default) means that each point is as big as to touch the center of its neighbor
    /// if it is at the same depth, leaving no gaps.
    /// A fill ratio of 0.5 means that each point touches the edge of its neighbor if it has the same depth.
    ///
    /// TODO(#6744): This applies only to 3D views!
    pub point_fill_ratio: Option<crate::components::FillRatio>,

    /// An optional floating point value that specifies the 2D drawing order, used only if the depth image is shown as a 2D image.
    ///
    /// Objects with higher values are drawn on top of those with lower values.
    pub draw_order: Option<crate::components::DrawOrder>,
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.DepthImage".into()),
                component_name: "rerun.components.ImageBuffer".into(),
                archetype_field_name: Some("buffer".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.DepthImage".into()),
                component_name: "rerun.components.ImageFormat".into(),
                archetype_field_name: Some("format".into()),
            },
        ]
    });

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| {
        [ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.DepthImage".into()),
            component_name: "DepthImageIndicator".into(),
            archetype_field_name: None,
        }]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 5usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.DepthImage".into()),
                component_name: "rerun.components.DepthMeter".into(),
                archetype_field_name: Some("meter".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.DepthImage".into()),
                component_name: "rerun.components.Colormap".into(),
                archetype_field_name: Some("colormap".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.DepthImage".into()),
                component_name: "rerun.components.ValueRange".into(),
                archetype_field_name: Some("depth_range".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.DepthImage".into()),
                component_name: "rerun.components.FillRatio".into(),
                archetype_field_name: Some("point_fill_ratio".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.DepthImage".into()),
                component_name: "rerun.components.DrawOrder".into(),
                archetype_field_name: Some("draw_order".into()),
            },
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 8usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.DepthImage".into()),
                component_name: "rerun.components.ImageBuffer".into(),
                archetype_field_name: Some("buffer".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.DepthImage".into()),
                component_name: "rerun.components.ImageFormat".into(),
                archetype_field_name: Some("format".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.DepthImage".into()),
                component_name: "DepthImageIndicator".into(),
                archetype_field_name: None,
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.DepthImage".into()),
                component_name: "rerun.components.DepthMeter".into(),
                archetype_field_name: Some("meter".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.DepthImage".into()),
                component_name: "rerun.components.Colormap".into(),
                archetype_field_name: Some("colormap".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.DepthImage".into()),
                component_name: "rerun.components.ValueRange".into(),
                archetype_field_name: Some("depth_range".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.DepthImage".into()),
                component_name: "rerun.components.FillRatio".into(),
                archetype_field_name: Some("point_fill_ratio".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.DepthImage".into()),
                component_name: "rerun.components.DrawOrder".into(),
                archetype_field_name: Some("draw_order".into()),
            },
        ]
    });

impl DepthImage {
    /// The total number of components in the archetype: 2 required, 1 recommended, 5 optional
    pub const NUM_COMPONENTS: usize = 8usize;
}

/// Indicator component for the [`DepthImage`] [`::re_types_core::Archetype`]
pub type DepthImageIndicator = ::re_types_core::GenericIndicatorComponent<DepthImage>;

impl ::re_types_core::Archetype for DepthImage {
    type Indicator = DepthImageIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.DepthImage".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Depth image"
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: DepthImageIndicator = DepthImageIndicator::DEFAULT;
        MaybeOwnedComponentBatch::new(&INDICATOR as &dyn ::re_types_core::ComponentBatch)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let buffer = {
            let array = arrays_by_name
                .get("rerun.components.ImageBuffer")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.DepthImage#buffer")?;
            <crate::components::ImageBuffer>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.DepthImage#buffer")?
                .into_iter()
                .next()
                .flatten()
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.DepthImage#buffer")?
        };
        let format = {
            let array = arrays_by_name
                .get("rerun.components.ImageFormat")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.DepthImage#format")?;
            <crate::components::ImageFormat>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.DepthImage#format")?
                .into_iter()
                .next()
                .flatten()
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.DepthImage#format")?
        };
        let meter = if let Some(array) = arrays_by_name.get("rerun.components.DepthMeter") {
            <crate::components::DepthMeter>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.DepthImage#meter")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let colormap = if let Some(array) = arrays_by_name.get("rerun.components.Colormap") {
            <crate::components::Colormap>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.DepthImage#colormap")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let depth_range = if let Some(array) = arrays_by_name.get("rerun.components.ValueRange") {
            <crate::components::ValueRange>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.DepthImage#depth_range")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let point_fill_ratio = if let Some(array) = arrays_by_name.get("rerun.components.FillRatio")
        {
            <crate::components::FillRatio>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.DepthImage#point_fill_ratio")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let draw_order = if let Some(array) = arrays_by_name.get("rerun.components.DrawOrder") {
            <crate::components::DrawOrder>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.DepthImage#draw_order")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        Ok(Self {
            buffer,
            format,
            meter,
            colormap,
            depth_range,
            point_fill_ratio,
            draw_order,
        })
    }
}

impl ::re_types_core::AsComponents for DepthImage {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            (Some(&self.buffer as &dyn ComponentBatch)).map(|batch| {
                ::re_types_core::MaybeOwnedComponentBatch {
                    batch: batch.into(),
                    descriptor_override: Some(ComponentDescriptor {
                        archetype_name: Some("rerun.archetypes.DepthImage".into()),
                        archetype_field_name: Some(("buffer").into()),
                        component_name: ("rerun.components.ImageBuffer").into(),
                    }),
                }
            }),
            (Some(&self.format as &dyn ComponentBatch)).map(|batch| {
                ::re_types_core::MaybeOwnedComponentBatch {
                    batch: batch.into(),
                    descriptor_override: Some(ComponentDescriptor {
                        archetype_name: Some("rerun.archetypes.DepthImage".into()),
                        archetype_field_name: Some(("format").into()),
                        component_name: ("rerun.components.ImageFormat").into(),
                    }),
                }
            }),
            (self
                .meter
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::MaybeOwnedComponentBatch {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.archetypes.DepthImage".into()),
                    archetype_field_name: Some(("meter").into()),
                    component_name: ("rerun.components.DepthMeter").into(),
                }),
            }),
            (self
                .colormap
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::MaybeOwnedComponentBatch {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.archetypes.DepthImage".into()),
                    archetype_field_name: Some(("colormap").into()),
                    component_name: ("rerun.components.Colormap").into(),
                }),
            }),
            (self
                .depth_range
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::MaybeOwnedComponentBatch {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.archetypes.DepthImage".into()),
                    archetype_field_name: Some(("depth_range").into()),
                    component_name: ("rerun.components.ValueRange").into(),
                }),
            }),
            (self
                .point_fill_ratio
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::MaybeOwnedComponentBatch {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.archetypes.DepthImage".into()),
                    archetype_field_name: Some(("point_fill_ratio").into()),
                    component_name: ("rerun.components.FillRatio").into(),
                }),
            }),
            (self
                .draw_order
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::MaybeOwnedComponentBatch {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.archetypes.DepthImage".into()),
                    archetype_field_name: Some(("draw_order").into()),
                    component_name: ("rerun.components.DrawOrder").into(),
                }),
            }),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for DepthImage {}

impl DepthImage {
    /// Create a new `DepthImage`.
    #[inline]
    pub fn new(
        buffer: impl Into<crate::components::ImageBuffer>,
        format: impl Into<crate::components::ImageFormat>,
    ) -> Self {
        Self {
            buffer: buffer.into(),
            format: format.into(),
            meter: None,
            colormap: None,
            depth_range: None,
            point_fill_ratio: None,
            draw_order: None,
        }
    }

    /// An optional floating point value that specifies how long a meter is in the native depth units.
    ///
    /// For instance: with uint16, perhaps meter=1000 which would mean you have millimeter precision
    /// and a range of up to ~65 meters (2^16 / 1000).
    ///
    /// Note that the only effect on 2D views is the physical depth values shown when hovering the image.
    /// In 3D views on the other hand, this affects where the points of the point cloud are placed.
    #[inline]
    pub fn with_meter(mut self, meter: impl Into<crate::components::DepthMeter>) -> Self {
        self.meter = Some(meter.into());
        self
    }

    /// Colormap to use for rendering the depth image.
    ///
    /// If not set, the depth image will be rendered using the Turbo colormap.
    #[inline]
    pub fn with_colormap(mut self, colormap: impl Into<crate::components::Colormap>) -> Self {
        self.colormap = Some(colormap.into());
        self
    }

    /// The expected range of depth values.
    ///
    /// This is typically the expected range of valid values.
    /// Everything outside of the range is clamped to the range for the purpose of colormpaping.
    /// Note that point clouds generated from this image will still display all points, regardless of this range.
    ///
    /// If not specified, the range will be automatically estimated from the data.
    /// Note that the Viewer may try to guess a wider range than the minimum/maximum of values
    /// in the contents of the depth image.
    /// E.g. if all values are positive, some bigger than 1.0 and all smaller than 255.0,
    /// the Viewer will guess that the data likely came from an 8bit image, thus assuming a range of 0-255.
    #[inline]
    pub fn with_depth_range(
        mut self,
        depth_range: impl Into<crate::components::ValueRange>,
    ) -> Self {
        self.depth_range = Some(depth_range.into());
        self
    }

    /// Scale the radii of the points in the point cloud generated from this image.
    ///
    /// A fill ratio of 1.0 (the default) means that each point is as big as to touch the center of its neighbor
    /// if it is at the same depth, leaving no gaps.
    /// A fill ratio of 0.5 means that each point touches the edge of its neighbor if it has the same depth.
    ///
    /// TODO(#6744): This applies only to 3D views!
    #[inline]
    pub fn with_point_fill_ratio(
        mut self,
        point_fill_ratio: impl Into<crate::components::FillRatio>,
    ) -> Self {
        self.point_fill_ratio = Some(point_fill_ratio.into());
        self
    }

    /// An optional floating point value that specifies the 2D drawing order, used only if the depth image is shown as a 2D image.
    ///
    /// Objects with higher values are drawn on top of those with lower values.
    #[inline]
    pub fn with_draw_order(mut self, draw_order: impl Into<crate::components::DrawOrder>) -> Self {
        self.draw_order = Some(draw_order.into());
        self
    }
}

impl ::re_types_core::SizeBytes for DepthImage {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.buffer.heap_size_bytes()
            + self.format.heap_size_bytes()
            + self.meter.heap_size_bytes()
            + self.colormap.heap_size_bytes()
            + self.depth_range.heap_size_bytes()
            + self.point_fill_ratio.heap_size_bytes()
            + self.draw_order.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::components::ImageBuffer>::is_pod()
            && <crate::components::ImageFormat>::is_pod()
            && <Option<crate::components::DepthMeter>>::is_pod()
            && <Option<crate::components::Colormap>>::is_pod()
            && <Option<crate::components::ValueRange>>::is_pod()
            && <Option<crate::components::FillRatio>>::is_pod()
            && <Option<crate::components::DrawOrder>>::is_pod()
    }
}
