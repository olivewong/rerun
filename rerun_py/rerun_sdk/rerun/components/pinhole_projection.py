# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/pinhole_projection.fbs".

# You can extend this class by creating a "PinholeProjectionExt" class in "pinhole_projection_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import (
    ComponentBatchMixin,
    ComponentMixin,
)

__all__ = ["PinholeProjection", "PinholeProjectionBatch"]


class PinholeProjection(datatypes.Mat3x3, ComponentMixin):
    """
    **Component**: Camera projection, from image coordinates to view coordinates.

    Child from parent.
    Image coordinates from camera view coordinates.

    Example:
    -------
    ```text
    1496.1     0.0  980.5
       0.0  1496.1  744.5
       0.0     0.0    1.0
    ```

    """

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of PinholeProjectionExt in pinhole_projection_ext.py

    # Note: there are no fields here because PinholeProjection delegates to datatypes.Mat3x3
    pass


class PinholeProjectionBatch(datatypes.Mat3x3Batch, ComponentBatchMixin):
    _COMPONENT_NAME: str = "rerun.components.PinholeProjection"


# This is patched in late to avoid circular dependencies.
PinholeProjection._BATCH_TYPE = PinholeProjectionBatch  # type: ignore[assignment]
