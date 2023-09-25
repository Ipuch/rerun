# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/archetypes/asset3d.fbs".

# You can extend this class by creating a "Asset3DExt" class in "asset3d_ext.py".

from __future__ import annotations

from attrs import define, field

from .. import components
from .._baseclasses import (
    Archetype,
)
from .asset3d_ext import Asset3DExt

__all__ = ["Asset3D"]


@define(str=False, repr=False)
class Asset3D(Asset3DExt, Archetype):
    """
    A prepacked 3D asset (.gltf, .glb, .obj, etc).

    Examples
    --------
    Simple 3D asset:
    ```python
    import sys

    import rerun as rr
    import rerun.experimental as rr2

    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <path_to_asset.[gltf|glb]>")
        sys.exit(1)

    rr.init("rerun_example_asset3d_simple", spawn=True)

    # TODO(#2816): some viewcoords would be nice here
    rr2.log("asset", rr2.Asset3D.from_file(sys.argv[1]))
    ```

    3D asset with out-of-tree transform:
    ```python
    import sys

    import numpy as np
    import rerun as rr
    import rerun.experimental as rr2

    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <path_to_asset.[gltf|glb]>")
        sys.exit(1)

    rr.init("rerun_example_asset3d_out_of_tree", spawn=True)

    # TODO(#2816): some viewcoords would be nice here

    rr.set_time_sequence("frame", 0)
    rr2.log("asset", rr2.Asset3D.from_file(sys.argv[1]))
    # Those points will not be affected by their parent's out-of-tree transform!
    rr2.log(
        "asset/points",
        rr2.Points3D(np.vstack([xyz.ravel() for xyz in np.mgrid[3 * [slice(-10, 10, 10j)]]]).T),
    )

    asset = rr2.Asset3D.from_file(sys.argv[1])
    for i in range(1, 20):
        rr.set_time_sequence("frame", i)

        translation = rr2.dt.TranslationRotationScale3D(translation=[0, 0, i - 10.0])
        rr2.log_components("asset", [rr2.cmp.OutOfTreeTransform3DBatch(translation)])
    ```
    """

    # You can define your own __init__ function as a member of Asset3DExt in asset3d_ext.py

    data: components.BlobArray = field(
        metadata={"component": "required"},
        converter=components.BlobArray.from_similar,  # type: ignore[misc]
    )
    """
    The asset's bytes.
    """

    media_type: components.MediaTypeArray | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.MediaTypeArray.optional_from_similar,  # type: ignore[misc]
    )
    """
    The Media Type of the asset.

    For instance:
    * `model/gltf-binary`
    * `model/obj`

    If omitted, the viewer will try to guess from the data.
    If it cannot guess, it won't be able to render the asset.
    """

    transform: components.OutOfTreeTransform3DArray | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.OutOfTreeTransform3DArray.optional_from_similar,  # type: ignore[misc]
    )
    """
    An out-of-tree transform.

    Applies a transformation to the asset itself without impacting its children.
    """

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__


if hasattr(Asset3DExt, "deferred_patch_class"):
    Asset3DExt.deferred_patch_class(Asset3D)
