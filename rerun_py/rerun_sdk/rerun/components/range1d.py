# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/range1d.fbs".

# You can extend this class by creating a "Range1DExt" class in "range1d_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import (
    ComponentBatchMixin,
    ComponentMixin,
)

__all__ = ["Range1D", "Range1DBatch"]


class Range1D(datatypes.Range1D, ComponentMixin):
    """**Component**: A 1D range, specifying a lower and upper bound."""

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of Range1DExt in range1d_ext.py

    # Note: there are no fields here because Range1D delegates to datatypes.Range1D
    pass


class Range1DBatch(datatypes.Range1DBatch, ComponentBatchMixin):
    _COMPONENT_NAME: str = "rerun.components.Range1D"


# This is patched in late to avoid circular dependencies.
Range1D._BATCH_TYPE = Range1DBatch  # type: ignore[assignment]
