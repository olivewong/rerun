# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/blueprint/components/visible_time_range.fbs".

# You can extend this class by creating a "VisibleTimeRangeExt" class in "visible_time_range_ext.py".

from __future__ import annotations

from ... import datatypes
from ..._baseclasses import (
    ComponentBatchMixin,
    ComponentMixin,
)

__all__ = ["VisibleTimeRange", "VisibleTimeRangeBatch"]


class VisibleTimeRange(datatypes.VisibleTimeRange, ComponentMixin):
    """
    **Component**: The range of values on a given timeline that will be included in a view's query.

    Refer to `VisibleTimeRanges` archetype for more information.
    """

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of VisibleTimeRangeExt in visible_time_range_ext.py

    # Note: there are no fields here because VisibleTimeRange delegates to datatypes.VisibleTimeRange
    pass


class VisibleTimeRangeBatch(datatypes.VisibleTimeRangeBatch, ComponentBatchMixin):
    _COMPONENT_NAME: str = "rerun.blueprint.components.VisibleTimeRange"


# This is patched in late to avoid circular dependencies.
VisibleTimeRange._BATCH_TYPE = VisibleTimeRangeBatch  # type: ignore[assignment]
