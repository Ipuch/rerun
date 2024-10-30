// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/datatypes/pixel_format.fbs".

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
#![allow(non_camel_case_types)]

use ::re_types_core::external::arrow2;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Datatype**: Specifieds a particular format of an [`archetypes::Image`][crate::archetypes::Image].
///
/// Most images can be described by a [`datatypes::ColorModel`][crate::datatypes::ColorModel] and a [`datatypes::ChannelDatatype`][crate::datatypes::ChannelDatatype],
/// e.g. `RGB` and `U8` respectively.
///
/// However, some image formats has chroma downsampling and/or
/// use differing number of bits per channel, and that is what this [`datatypes::PixelFormat`][crate::datatypes::PixelFormat] is for.
///
/// All these formats support random access.
///
/// For more compressed image formats, see [`archetypes::EncodedImage`][crate::archetypes::EncodedImage].
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum PixelFormat {
    /// `Y_U_V12` is a YUV 4:2:0 fully planar YUV format without chroma downsampling, also known as `I420`.
    ///
    /// This uses limited range YUV, i.e. Y is expected to be within [16, 235] and U/V within [16, 240].
    ///
    /// First comes entire image in Y in one plane, followed by the U and V planes, which each only have half
    /// the resolution of the Y plane.
    #[allow(clippy::upper_case_acronyms)]
    Y_U_V12_LimitedRange = 20,

    /// `NV12` (aka `Y_UV12`) is a YUV 4:2:0 chroma downsampled form at with 12 bits per pixel and 8 bits per channel.
    ///
    /// This uses limited range YUV, i.e. Y is expected to be within [16, 235] and U/V within [16, 240].
    ///
    /// First comes entire image in Y in one plane,
    /// followed by a plane with interleaved lines ordered as U0, V0, U1, V1, etc.
    #[default]
    #[allow(clippy::upper_case_acronyms)]
    NV12 = 26,

    /// `YUY2` (aka `YUYV`, `YUYV16` or `NV21`), is a YUV 4:2:2 chroma downsampled format with 16 bits per pixel and 8 bits per channel.
    ///
    /// This uses limited range YUV, i.e. Y is expected to be within [16, 235] and U/V within [16, 240].
    ///
    /// The order of the channels is Y0, U0, Y1, V0, all in the same plane.
    #[allow(clippy::upper_case_acronyms)]
    YUY2 = 27,

    /// Monochrome Y plane only, essentially a YUV 4:0:0 planar format.
    ///
    /// Also known as just "gray". This is virtually identical to a 8bit luminance/grayscale (see [`datatypes::ColorModel`][crate::datatypes::ColorModel]).
    ///
    /// This uses entire range YUV, i.e. Y is expected to be within [0, 255].
    /// (as opposed to "limited range" YUV as used e.g. in NV12).
    #[allow(clippy::upper_case_acronyms)]
    Y8_FullRange = 30,

    /// `Y_U_V24` is a YUV 4:4:4 fully planar YUV format without chroma downsampling, also known as `I444`.
    ///
    /// This uses limited range YUV, i.e. Y is expected to be within [16, 235] and U/V within [16, 240].
    ///
    /// First comes entire image in Y in one plane, followed by the U and V planes.
    #[allow(clippy::upper_case_acronyms)]
    Y_U_V24_LimitedRange = 39,

    /// `Y_U_V24` is a YUV 4:4:4 fully planar YUV format without chroma downsampling, also known as `I444`.
    ///
    /// This uses full range YUV with all components ranging from 0 to 255
    /// (as opposed to "limited range" YUV as used e.g. in NV12).
    ///
    /// First comes entire image in Y in one plane, followed by the U and V planes.
    #[allow(clippy::upper_case_acronyms)]
    Y_U_V24_FullRange = 40,

    /// Monochrome Y plane only, essentially a YUV 4:0:0 planar format.
    ///
    /// Also known as just "gray".
    ///
    /// This uses limited range YUV, i.e. Y is expected to be within [16, 235].
    /// If not for this range limitation/remapping, this is almost identical to 8bit luminace/grayscale (see [`datatypes::ColorModel`][crate::datatypes::ColorModel]).
    #[allow(clippy::upper_case_acronyms)]
    Y8_LimitedRange = 41,

    /// `Y_U_V12` is a YUV 4:2:0 fully planar YUV format without chroma downsampling, also known as `I420`.
    ///
    /// This uses full range YUV with all components ranging from 0 to 255
    /// (as opposed to "limited range" YUV as used e.g. in NV12).
    ///
    /// First comes entire image in Y in one plane, followed by the U and V planes, which each only have half
    /// the resolution of the Y plane.
    #[allow(clippy::upper_case_acronyms)]
    Y_U_V12_FullRange = 44,

    /// `Y_U_V16` is a YUV 4:2:2 fully planar YUV format without chroma downsampling, also known as `I422`.
    ///
    /// This uses limited range YUV, i.e. Y is expected to be within [16, 235] and U/V within [16, 240].
    ///
    /// First comes entire image in Y in one plane, followed by the U and V planes, which each only have half
    /// the horizontal resolution of the Y plane.
    #[allow(clippy::upper_case_acronyms)]
    Y_U_V16_LimitedRange = 49,

    /// `Y_U_V16` is a YUV 4:2:2 fully planar YUV format without chroma downsampling, also known as `I422`.
    ///
    /// This uses full range YUV with all components ranging from 0 to 255
    /// (as opposed to "limited range" YUV as used e.g. in NV12).
    ///
    /// First comes entire image in Y in one plane, followed by the U and V planes, which each only have half
    /// the horizontal resolution of the Y plane.
    #[allow(clippy::upper_case_acronyms)]
    Y_U_V16_FullRange = 50,
}

::re_types_core::macros::impl_into_cow!(PixelFormat);

impl ::re_types_core::Loggable for PixelFormat {
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow2::datatypes::*;
        DataType::UInt8
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        #![allow(clippy::wildcard_imports)]
        #![allow(clippy::manual_is_variant_and)]
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data0): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| *datum as u8);
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_bitmap: Option<arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            PrimitiveArray::new(
                Self::arrow_datatype(),
                data0.into_iter().map(|v| v.unwrap_or_default()).collect(),
                data0_bitmap,
            )
            .boxed()
        })
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        #![allow(clippy::wildcard_imports)]
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, buffer::*, datatypes::*};
        Ok(arrow_data
            .as_any()
            .downcast_ref::<UInt8Array>()
            .ok_or_else(|| {
                let expected = Self::arrow_datatype();
                let actual = arrow_data.data_type().clone();
                DeserializationError::datatype_mismatch(expected, actual)
            })
            .with_context("rerun.datatypes.PixelFormat#enum")?
            .into_iter()
            .map(|opt| opt.copied())
            .map(|typ| match typ {
                Some(20) => Ok(Some(Self::Y_U_V12_LimitedRange)),
                Some(26) => Ok(Some(Self::NV12)),
                Some(27) => Ok(Some(Self::YUY2)),
                Some(30) => Ok(Some(Self::Y8_FullRange)),
                Some(39) => Ok(Some(Self::Y_U_V24_LimitedRange)),
                Some(40) => Ok(Some(Self::Y_U_V24_FullRange)),
                Some(41) => Ok(Some(Self::Y8_LimitedRange)),
                Some(44) => Ok(Some(Self::Y_U_V12_FullRange)),
                Some(49) => Ok(Some(Self::Y_U_V16_LimitedRange)),
                Some(50) => Ok(Some(Self::Y_U_V16_FullRange)),
                None => Ok(None),
                Some(invalid) => Err(DeserializationError::missing_union_arm(
                    Self::arrow_datatype(),
                    "<invalid>",
                    invalid as _,
                )),
            })
            .collect::<DeserializationResult<Vec<Option<_>>>>()
            .with_context("rerun.datatypes.PixelFormat")?)
    }
}

impl std::fmt::Display for PixelFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Y_U_V12_LimitedRange => write!(f, "Y_U_V12_LimitedRange"),
            Self::NV12 => write!(f, "NV12"),
            Self::YUY2 => write!(f, "YUY2"),
            Self::Y8_FullRange => write!(f, "Y8_FullRange"),
            Self::Y_U_V24_LimitedRange => write!(f, "Y_U_V24_LimitedRange"),
            Self::Y_U_V24_FullRange => write!(f, "Y_U_V24_FullRange"),
            Self::Y8_LimitedRange => write!(f, "Y8_LimitedRange"),
            Self::Y_U_V12_FullRange => write!(f, "Y_U_V12_FullRange"),
            Self::Y_U_V16_LimitedRange => write!(f, "Y_U_V16_LimitedRange"),
            Self::Y_U_V16_FullRange => write!(f, "Y_U_V16_FullRange"),
        }
    }
}

impl ::re_types_core::reflection::Enum for PixelFormat {
    #[inline]
    fn variants() -> &'static [Self] {
        &[
            Self::Y_U_V12_LimitedRange,
            Self::NV12,
            Self::YUY2,
            Self::Y8_FullRange,
            Self::Y_U_V24_LimitedRange,
            Self::Y_U_V24_FullRange,
            Self::Y8_LimitedRange,
            Self::Y_U_V12_FullRange,
            Self::Y_U_V16_LimitedRange,
            Self::Y_U_V16_FullRange,
        ]
    }

    #[inline]
    fn docstring_md(self) -> &'static str {
        match self {
            Self::Y_U_V12_LimitedRange => {
                "`Y_U_V12` is a YUV 4:2:0 fully planar YUV format without chroma downsampling, also known as `I420`.\n\nThis uses limited range YUV, i.e. Y is expected to be within [16, 235] and U/V within [16, 240].\n\nFirst comes entire image in Y in one plane, followed by the U and V planes, which each only have half\nthe resolution of the Y plane."
            }
            Self::NV12 => {
                "`NV12` (aka `Y_UV12`) is a YUV 4:2:0 chroma downsampled form at with 12 bits per pixel and 8 bits per channel.\n\nThis uses limited range YUV, i.e. Y is expected to be within [16, 235] and U/V within [16, 240].\n\nFirst comes entire image in Y in one plane,\nfollowed by a plane with interleaved lines ordered as U0, V0, U1, V1, etc."
            }
            Self::YUY2 => {
                "`YUY2` (aka `YUYV`, `YUYV16` or `NV21`), is a YUV 4:2:2 chroma downsampled format with 16 bits per pixel and 8 bits per channel.\n\nThis uses limited range YUV, i.e. Y is expected to be within [16, 235] and U/V within [16, 240].\n\nThe order of the channels is Y0, U0, Y1, V0, all in the same plane."
            }
            Self::Y8_FullRange => {
                "Monochrome Y plane only, essentially a YUV 4:0:0 planar format.\n\nAlso known as just \"gray\". This is virtually identical to a 8bit luminance/grayscale (see [`datatypes::ColorModel`][crate::datatypes::ColorModel]).\n\nThis uses entire range YUV, i.e. Y is expected to be within [0, 255].\n(as opposed to \"limited range\" YUV as used e.g. in NV12)."
            }
            Self::Y_U_V24_LimitedRange => {
                "`Y_U_V24` is a YUV 4:4:4 fully planar YUV format without chroma downsampling, also known as `I444`.\n\nThis uses limited range YUV, i.e. Y is expected to be within [16, 235] and U/V within [16, 240].\n\nFirst comes entire image in Y in one plane, followed by the U and V planes."
            }
            Self::Y_U_V24_FullRange => {
                "`Y_U_V24` is a YUV 4:4:4 fully planar YUV format without chroma downsampling, also known as `I444`.\n\nThis uses full range YUV with all components ranging from 0 to 255\n(as opposed to \"limited range\" YUV as used e.g. in NV12).\n\nFirst comes entire image in Y in one plane, followed by the U and V planes."
            }
            Self::Y8_LimitedRange => {
                "Monochrome Y plane only, essentially a YUV 4:0:0 planar format.\n\nAlso known as just \"gray\".\n\nThis uses limited range YUV, i.e. Y is expected to be within [16, 235].\nIf not for this range limitation/remapping, this is almost identical to 8bit luminace/grayscale (see [`datatypes::ColorModel`][crate::datatypes::ColorModel])."
            }
            Self::Y_U_V12_FullRange => {
                "`Y_U_V12` is a YUV 4:2:0 fully planar YUV format without chroma downsampling, also known as `I420`.\n\nThis uses full range YUV with all components ranging from 0 to 255\n(as opposed to \"limited range\" YUV as used e.g. in NV12).\n\nFirst comes entire image in Y in one plane, followed by the U and V planes, which each only have half\nthe resolution of the Y plane."
            }
            Self::Y_U_V16_LimitedRange => {
                "`Y_U_V16` is a YUV 4:2:2 fully planar YUV format without chroma downsampling, also known as `I422`.\n\nThis uses limited range YUV, i.e. Y is expected to be within [16, 235] and U/V within [16, 240].\n\nFirst comes entire image in Y in one plane, followed by the U and V planes, which each only have half\nthe horizontal resolution of the Y plane."
            }
            Self::Y_U_V16_FullRange => {
                "`Y_U_V16` is a YUV 4:2:2 fully planar YUV format without chroma downsampling, also known as `I422`.\n\nThis uses full range YUV with all components ranging from 0 to 255\n(as opposed to \"limited range\" YUV as used e.g. in NV12).\n\nFirst comes entire image in Y in one plane, followed by the U and V planes, which each only have half\nthe horizontal resolution of the Y plane."
            }
        }
    }
}

impl ::re_types_core::SizeBytes for PixelFormat {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        0
    }

    #[inline]
    fn is_pod() -> bool {
        true
    }
}
