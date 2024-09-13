// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/datatypes/video_timestamp.fbs".

#pragma once

#include "../result.hpp"
#include "video_time_mode.hpp"

#include <cstdint>
#include <memory>

namespace arrow {
    class Array;
    class DataType;
    class StructBuilder;
} // namespace arrow

namespace rerun::datatypes {
    /// **Datatype**: Timestamp inside a `archetypes::AssetVideo`.
    ///
    /// ⚠ **This is an experimental API! It is not fully supported, and is likely to change significantly in future versions.**
    struct VideoTimestamp {
        /// Timestamp value, type defined by `time_mode`.
        int64_t video_time;

        /// How to interpret `video_time`.
        rerun::datatypes::VideoTimeMode time_mode;

      public:
        VideoTimestamp() = default;
    };
} // namespace rerun::datatypes

namespace rerun {
    template <typename T>
    struct Loggable;

    /// \private
    template <>
    struct Loggable<datatypes::VideoTimestamp> {
        static constexpr const char Name[] = "rerun.datatypes.VideoTimestamp";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Serializes an array of `rerun::datatypes::VideoTimestamp` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const datatypes::VideoTimestamp* instances, size_t num_instances
        );

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::StructBuilder* builder, const datatypes::VideoTimestamp* elements,
            size_t num_elements
        );
    };
} // namespace rerun