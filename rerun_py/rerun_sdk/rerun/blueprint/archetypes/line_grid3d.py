# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/line_grid_3d.fbs".

# You can extend this class by creating a "LineGrid3DExt" class in "line_grid3d_ext.py".

from __future__ import annotations

from typing import Any

from attrs import define, field

from ... import components, datatypes
from ..._baseclasses import (
    Archetype,
)
from ...blueprint import components as blueprint_components
from ...error_utils import catch_and_log_exceptions

__all__ = ["LineGrid3D"]


@define(str=False, repr=False, init=False)
class LineGrid3D(Archetype):
    """**Archetype**: Configuration for the 3D line grid."""

    def __init__(
        self: Any,
        *,
        visible: datatypes.BoolLike | None = None,
        spacing: datatypes.Float32Like | None = None,
        orientation: blueprint_components.PlaneOrientationLike | None = None,
        offset: datatypes.Float32Like | None = None,
        line_radius: datatypes.Float32Like | None = None,
        color: datatypes.Rgba32Like | None = None,
    ):
        """
        Create a new instance of the LineGrid3D archetype.

        Parameters
        ----------
        visible:
            Whether the grid is visible.

            Defaults to true.
        spacing:
            Space between grid lines spacing of one line to the next in scene units.
        orientation:
            How the grid is oriented.

            Defaults to whatever plane is determined as the down plane by view coordinates if present.
        offset:
            Offset of the grid along its normal.
        line_radius:
            How thick the lines should be in ui units.

            Default is 0.5 ui unit.
        color:
            Color used for the grid.

            Transparency via alpha channel is supported.
            Defaults to a slightly transparent light gray.

        """

        # You can define your own __init__ function as a member of LineGrid3DExt in line_grid3d_ext.py
        with catch_and_log_exceptions(context=self.__class__.__name__):
            self.__attrs_init__(
                visible=visible,
                spacing=spacing,
                orientation=orientation,
                offset=offset,
                line_radius=line_radius,
                color=color,
            )
            return
        self.__attrs_clear__()

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            visible=None,  # type: ignore[arg-type]
            spacing=None,  # type: ignore[arg-type]
            orientation=None,  # type: ignore[arg-type]
            offset=None,  # type: ignore[arg-type]
            line_radius=None,  # type: ignore[arg-type]
            color=None,  # type: ignore[arg-type]
        )

    @classmethod
    def _clear(cls) -> LineGrid3D:
        """Produce an empty LineGrid3D, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    visible: blueprint_components.VisibleBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=blueprint_components.VisibleBatch._optional,  # type: ignore[misc]
    )
    # Whether the grid is visible.
    #
    # Defaults to true.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    spacing: blueprint_components.GridSpacingBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=blueprint_components.GridSpacingBatch._optional,  # type: ignore[misc]
    )
    # Space between grid lines spacing of one line to the next in scene units.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    orientation: blueprint_components.PlaneOrientationBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=blueprint_components.PlaneOrientationBatch._optional,  # type: ignore[misc]
    )
    # How the grid is oriented.
    #
    # Defaults to whatever plane is determined as the down plane by view coordinates if present.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    offset: blueprint_components.PlaneOffsetBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=blueprint_components.PlaneOffsetBatch._optional,  # type: ignore[misc]
    )
    # Offset of the grid along its normal.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    line_radius: blueprint_components.UiRadiusBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=blueprint_components.UiRadiusBatch._optional,  # type: ignore[misc]
    )
    # How thick the lines should be in ui units.
    #
    # Default is 0.5 ui unit.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    color: components.ColorBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.ColorBatch._optional,  # type: ignore[misc]
    )
    # Color used for the grid.
    #
    # Transparency via alpha channel is supported.
    # Defaults to a slightly transparent light gray.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__  # type: ignore[assignment]
