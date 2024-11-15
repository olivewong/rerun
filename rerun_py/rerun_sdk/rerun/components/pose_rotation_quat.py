# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/rotation_quat.fbs".

# You can extend this class by creating a "PoseRotationQuatExt" class in "pose_rotation_quat_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import (
    ComponentBatchMixin,
    ComponentMixin,
)

__all__ = ["PoseRotationQuat", "PoseRotationQuatBatch"]


class PoseRotationQuat(datatypes.Quaternion, ComponentMixin):
    """
    **Component**: A 3D rotation expressed as a quaternion that doesn't propagate in the transform hierarchy.

    Note: although the x,y,z,w components of the quaternion will be passed through to the
    datastore as provided, when used in the Viewer, quaternions will always be normalized.
    """

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of PoseRotationQuatExt in pose_rotation_quat_ext.py

    # Note: there are no fields here because PoseRotationQuat delegates to datatypes.Quaternion
    pass


class PoseRotationQuatBatch(datatypes.QuaternionBatch, ComponentBatchMixin):
    _COMPONENT_NAME: str = "rerun.components.PoseRotationQuat"


# This is patched in late to avoid circular dependencies.
PoseRotationQuat._BATCH_TYPE = PoseRotationQuatBatch  # type: ignore[assignment]
