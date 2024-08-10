# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/testing/components/enum_test.fbs".

# You can extend this class by creating a "MultiEnumExt" class in "multi_enum_ext.py".

from __future__ import annotations

from typing import Any, Sequence, Union

import pyarrow as pa
from attrs import define, field
from rerun._baseclasses import (
    BaseBatch,
    BaseExtensionType,
)

from .. import datatypes

__all__ = ["MultiEnum", "MultiEnumArrayLike", "MultiEnumBatch", "MultiEnumLike", "MultiEnumType"]


@define(init=False)
class MultiEnum:
    def __init__(self: Any, value1: datatypes.EnumTestLike, value2: datatypes.ValuedEnumLike | None = None):
        """
        Create a new instance of the MultiEnum datatype.

        Parameters
        ----------
        value1:
            The first value.
        value2:
            The second value.

        """

        # You can define your own __init__ function as a member of MultiEnumExt in multi_enum_ext.py
        self.__attrs_init__(value1=value1, value2=value2)

    value1: datatypes.EnumTest = field()
    # The first value.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    value2: datatypes.ValuedEnum | None = field(default=None)
    # The second value.
    #
    # (Docstring intentionally commented out to hide this field from the docs)


MultiEnumLike = MultiEnum
MultiEnumArrayLike = Union[
    MultiEnum,
    Sequence[MultiEnumLike],
]


class MultiEnumType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.testing.datatypes.MultiEnum"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(
            self,
            pa.struct([
                pa.field("value1", pa.uint8(), nullable=False, metadata={}),
                pa.field("value2", pa.uint8(), nullable=True, metadata={}),
            ]),
            self._TYPE_NAME,
        )


class MultiEnumBatch(BaseBatch[MultiEnumArrayLike]):
    _ARROW_TYPE = MultiEnumType()

    @staticmethod
    def _native_to_pa_array(data: MultiEnumArrayLike, data_type: pa.DataType) -> pa.Array:
        from rerun.testing.datatypes import EnumTestBatch, ValuedEnumBatch

        if isinstance(data, MultiEnum):
            data = [data]

        return pa.StructArray.from_arrays(
            [
                EnumTestBatch([x.value1 for x in data]).as_arrow_array().storage,  # type: ignore[misc, arg-type]
                ValuedEnumBatch([x.value2 for x in data]).as_arrow_array().storage,  # type: ignore[misc, arg-type]
            ],
            fields=list(data_type),
        )
