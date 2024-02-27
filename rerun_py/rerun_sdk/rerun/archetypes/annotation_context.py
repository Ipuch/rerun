# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/archetypes/annotation_context.fbs".

# You can extend this class by creating a "AnnotationContextExt" class in "annotation_context_ext.py".

from __future__ import annotations

from typing import Any

from attrs import define, field

from .. import components
from .._baseclasses import Archetype
from ..error_utils import catch_and_log_exceptions

__all__ = ["AnnotationContext"]


@define(str=False, repr=False, init=False)
class AnnotationContext(Archetype):
    """
    **Archetype**: The `AnnotationContext` provides additional information on how to display entities.

    Entities can use `ClassId`s and `KeypointId`s to provide annotations, and
    the labels and colors will be looked up in the appropriate
    `AnnotationContext`. We use the *first* annotation context we find in the
    path-hierarchy when searching up through the ancestors of a given entity
    path.

    See also [`ClassDescription`][rerun.datatypes.ClassDescription].

    Example
    -------
    ### Segmentation:
    ```python
    import numpy as np
    import rerun as rr

    rr.init("rerun_example_annotation_context_segmentation", spawn=True)

    # Create a simple segmentation image
    image = np.zeros((200, 300), dtype=np.uint8)
    image[50:100, 50:120] = 1
    image[100:180, 130:280] = 2

    # Log an annotation context to assign a label and color to each class
    rr.log("segmentation", rr.AnnotationContext([(1, "red", (255, 0, 0)), (2, "green", (0, 255, 0))]), timeless=True)

    rr.log("segmentation/image", rr.SegmentationImage(image))
    ```
    <center>
    <picture>
      <source media="(max-width: 480px)" srcset="https://static.rerun.io/annotation_context_segmentation/0e21c0a04e456fec41d16b0deaa12c00cddf2d9b/480w.png">
      <source media="(max-width: 768px)" srcset="https://static.rerun.io/annotation_context_segmentation/0e21c0a04e456fec41d16b0deaa12c00cddf2d9b/768w.png">
      <source media="(max-width: 1024px)" srcset="https://static.rerun.io/annotation_context_segmentation/0e21c0a04e456fec41d16b0deaa12c00cddf2d9b/1024w.png">
      <source media="(max-width: 1200px)" srcset="https://static.rerun.io/annotation_context_segmentation/0e21c0a04e456fec41d16b0deaa12c00cddf2d9b/1200w.png">
      <img src="https://static.rerun.io/annotation_context_segmentation/0e21c0a04e456fec41d16b0deaa12c00cddf2d9b/full.png" width="640">
    </picture>
    </center>

    """

    def __init__(self: Any, context: components.AnnotationContextLike):
        """
        Create a new instance of the AnnotationContext archetype.

        Parameters
        ----------
        context:
            List of class descriptions, mapping class indices to class names, colors etc.

        """

        # You can define your own __init__ function as a member of AnnotationContextExt in annotation_context_ext.py
        with catch_and_log_exceptions(context=self.__class__.__name__):
            self.__attrs_init__(context=context)
            return
        self.__attrs_clear__()

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            context=None,  # type: ignore[arg-type]
        )

    @classmethod
    def _clear(cls) -> AnnotationContext:
        """Produce an empty AnnotationContext, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    context: components.AnnotationContextBatch = field(
        metadata={"component": "required"},
        converter=components.AnnotationContextBatch._required,  # type: ignore[misc]
    )
    # List of class descriptions, mapping class indices to class names, colors etc.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__
