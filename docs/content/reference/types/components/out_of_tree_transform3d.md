---
title: "OutOfTreeTransform3D"
---
<!-- DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/docs/mod.rs -->

An out-of-tree affine transform between two 3D spaces, represented in a given direction.

"Out-of-tree" means that the transform only affects its own entity: children don't inherit from it.

## Fields

* repr: [`Transform3D`](../datatypes/transform3d.md)

## API reference links
 * 🌊 [C++ API docs for `OutOfTreeTransform3D`](https://ref.rerun.io/docs/cpp/stable/structrerun_1_1components_1_1OutOfTreeTransform3D.html)
 * 🐍 [Python API docs for `OutOfTreeTransform3D`](https://ref.rerun.io/docs/python/stable/common/components#rerun.components.OutOfTreeTransform3D)
 * 🦀 [Rust API docs for `OutOfTreeTransform3D`](https://docs.rs/rerun/latest/rerun/components/struct.OutOfTreeTransform3D.html)


## Used by

* [`Asset3D`](../archetypes/asset3d.md)