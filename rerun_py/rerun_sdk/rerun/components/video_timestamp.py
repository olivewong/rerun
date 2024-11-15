# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/video_timestamp.fbs".

# You can extend this class by creating a "VideoTimestampExt" class in "video_timestamp_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import (
    ComponentBatchMixin,
    ComponentMixin,
)
from .video_timestamp_ext import VideoTimestampExt

__all__ = ["VideoTimestamp", "VideoTimestampBatch"]


class VideoTimestamp(VideoTimestampExt, datatypes.VideoTimestamp, ComponentMixin):
    """**Component**: Timestamp inside a [`archetypes.AssetVideo`][rerun.archetypes.AssetVideo]."""

    _BATCH_TYPE = None
    # __init__ can be found in video_timestamp_ext.py

    # Note: there are no fields here because VideoTimestamp delegates to datatypes.VideoTimestamp
    pass


class VideoTimestampBatch(datatypes.VideoTimestampBatch, ComponentBatchMixin):
    _COMPONENT_NAME: str = "rerun.components.VideoTimestamp"


# This is patched in late to avoid circular dependencies.
VideoTimestamp._BATCH_TYPE = VideoTimestampBatch  # type: ignore[assignment]
