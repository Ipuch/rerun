// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/datatypes/plane3d.fbs".

#pragma once

#include "../result.hpp"

#include <array>
#include <cstdint>
#include <memory>

namespace arrow {
    class Array;
    class DataType;
    class FixedSizeListBuilder;
} // namespace arrow

namespace rerun::datatypes {
    /// **Datatype**: An infinite 3D plane represented by a unit normal vector and a distance.
    ///
    /// Any point P on the plane fulfills the equation `dot(xyz, P) - d = 0`,
    /// where `xyz` is the plane's normal and `d` the distance of the plane from the origin.
    /// This representation is also known as the Hesse normal form.
    ///
    /// Note: although the normal will be passed through to the
    /// datastore as provided, when used in the Viewer, planes will always be normalized.
    /// I.e. the plane with xyz = (2, 0, 0), d = 1 is equivalent to xyz = (1, 0, 0), d = 0.5
    struct Plane3D {
        std::array<float, 4> xyzd;

      public:
        Plane3D() = default;
    };
} // namespace rerun::datatypes

namespace rerun {
    template <typename T>
    struct Loggable;

    /// \private
    template <>
    struct Loggable<datatypes::Plane3D> {
        static constexpr const char Name[] = "rerun.datatypes.Plane3D";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Serializes an array of `rerun::datatypes::Plane3D` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const datatypes::Plane3D* instances, size_t num_instances
        );

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::FixedSizeListBuilder* builder, const datatypes::Plane3D* elements,
            size_t num_elements
        );
    };
} // namespace rerun