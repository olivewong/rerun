# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/opacity.fbs".

# You can extend this class by creating a "OpacityExt" class in "opacity_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import (
    ComponentBatchMixin,
    ComponentMixin,
)

__all__ = ["Opacity", "OpacityBatch"]


class Opacity(datatypes.Float32, ComponentMixin):
    """
    **Component**: Degree of transparency ranging from 0.0 (fully transparent) to 1.0 (fully opaque).

    The final opacity value may be a result of multiplication with alpha values as specified by other color sources.
    Unless otherwise specified, the default value is 1.
    """

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of OpacityExt in opacity_ext.py

    # Note: there are no fields here because Opacity delegates to datatypes.Float32
    pass


class OpacityBatch(datatypes.Float32Batch, ComponentBatchMixin):
    _COMPONENT_NAME: str = "rerun.components.Opacity"


# This is patched in late to avoid circular dependencies.
Opacity._BATCH_TYPE = OpacityBatch  # type: ignore[assignment]
