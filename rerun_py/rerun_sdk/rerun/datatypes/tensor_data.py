# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/datatypes/tensor_data.fbs".

# You can extend this class by creating a "TensorDataExt" class in "tensor_data_ext.py".

from __future__ import annotations

from typing import TYPE_CHECKING, Any, Sequence, Union

import numpy.typing as npt
import pyarrow as pa
from attrs import define, field

from .. import datatypes
from .._baseclasses import BaseBatch, BaseExtensionType
from .tensor_data_ext import TensorDataExt

__all__ = ["TensorData", "TensorDataArrayLike", "TensorDataBatch", "TensorDataLike", "TensorDataType"]


def _tensor_data__buffer__special_field_converter_override(x: datatypes.TensorBufferLike) -> datatypes.TensorBuffer:
    if isinstance(x, datatypes.TensorBuffer):
        return x
    else:
        return datatypes.TensorBuffer(x)


@define(init=False)
class TensorData(TensorDataExt):
    """
    **Datatype**: A multi-dimensional `Tensor` of data.

    The number of dimensions and their respective lengths is specified by the `shape` field.
    The dimensions are ordered from outermost to innermost. For example, in the common case of
    a 2D RGB Image, the shape would be `[height, width, channel]`.

    These dimensions are combined with an index to look up values from the `buffer` field,
    which stores a contiguous array of typed values.

    Note that the buffer may be encoded in a compressed format such as `jpeg` or
    in a format with downsampled chroma, such as NV12 or YUY2.
    For file formats, the shape is used as a hint, for chroma downsampled format
    the shape has to be the shape of the decoded image.
    """

    # __init__ can be found in tensor_data_ext.py

    shape: list[datatypes.TensorDimension] = field()
    buffer: datatypes.TensorBuffer = field(converter=_tensor_data__buffer__special_field_converter_override)


if TYPE_CHECKING:
    TensorDataLike = Union[TensorData, npt.ArrayLike]
else:
    TensorDataLike = Any

TensorDataArrayLike = Union[TensorData, Sequence[TensorDataLike], npt.ArrayLike]


class TensorDataType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.datatypes.TensorData"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(
            self,
            pa.struct(
                [
                    pa.field(
                        "shape",
                        pa.list_(
                            pa.field(
                                "item",
                                pa.struct(
                                    [
                                        pa.field("size", pa.uint64(), nullable=False, metadata={}),
                                        pa.field("name", pa.utf8(), nullable=True, metadata={}),
                                    ]
                                ),
                                nullable=False,
                                metadata={},
                            )
                        ),
                        nullable=False,
                        metadata={},
                    ),
                    pa.field(
                        "buffer",
                        pa.dense_union(
                            [
                                pa.field("_null_markers", pa.null(), nullable=True, metadata={}),
                                pa.field(
                                    "U8",
                                    pa.list_(pa.field("item", pa.uint8(), nullable=False, metadata={})),
                                    nullable=False,
                                    metadata={},
                                ),
                                pa.field(
                                    "U16",
                                    pa.list_(pa.field("item", pa.uint16(), nullable=False, metadata={})),
                                    nullable=False,
                                    metadata={},
                                ),
                                pa.field(
                                    "U32",
                                    pa.list_(pa.field("item", pa.uint32(), nullable=False, metadata={})),
                                    nullable=False,
                                    metadata={},
                                ),
                                pa.field(
                                    "U64",
                                    pa.list_(pa.field("item", pa.uint64(), nullable=False, metadata={})),
                                    nullable=False,
                                    metadata={},
                                ),
                                pa.field(
                                    "I8",
                                    pa.list_(pa.field("item", pa.int8(), nullable=False, metadata={})),
                                    nullable=False,
                                    metadata={},
                                ),
                                pa.field(
                                    "I16",
                                    pa.list_(pa.field("item", pa.int16(), nullable=False, metadata={})),
                                    nullable=False,
                                    metadata={},
                                ),
                                pa.field(
                                    "I32",
                                    pa.list_(pa.field("item", pa.int32(), nullable=False, metadata={})),
                                    nullable=False,
                                    metadata={},
                                ),
                                pa.field(
                                    "I64",
                                    pa.list_(pa.field("item", pa.int64(), nullable=False, metadata={})),
                                    nullable=False,
                                    metadata={},
                                ),
                                pa.field(
                                    "F16",
                                    pa.list_(pa.field("item", pa.float16(), nullable=False, metadata={})),
                                    nullable=False,
                                    metadata={},
                                ),
                                pa.field(
                                    "F32",
                                    pa.list_(pa.field("item", pa.float32(), nullable=False, metadata={})),
                                    nullable=False,
                                    metadata={},
                                ),
                                pa.field(
                                    "F64",
                                    pa.list_(pa.field("item", pa.float64(), nullable=False, metadata={})),
                                    nullable=False,
                                    metadata={},
                                ),
                                pa.field(
                                    "JPEG",
                                    pa.list_(pa.field("item", pa.uint8(), nullable=False, metadata={})),
                                    nullable=False,
                                    metadata={},
                                ),
                                pa.field(
                                    "NV12",
                                    pa.list_(pa.field("item", pa.uint8(), nullable=False, metadata={})),
                                    nullable=False,
                                    metadata={},
                                ),
                                pa.field(
                                    "YUY2",
                                    pa.list_(pa.field("item", pa.uint8(), nullable=False, metadata={})),
                                    nullable=False,
                                    metadata={},
                                ),
                            ]
                        ),
                        nullable=False,
                        metadata={},
                    ),
                ]
            ),
            self._TYPE_NAME,
        )


class TensorDataBatch(BaseBatch[TensorDataArrayLike]):
    _ARROW_TYPE = TensorDataType()

    @staticmethod
    def _native_to_pa_array(data: TensorDataArrayLike, data_type: pa.DataType) -> pa.Array:
        return TensorDataExt.native_to_pa_array_override(data, data_type)
