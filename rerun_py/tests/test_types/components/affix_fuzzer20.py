# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/testing/components/fuzzy.fbs".

# You can extend this class by creating a "AffixFuzzer20Ext" class in "affix_fuzzer20_ext.py".

from __future__ import annotations

from rerun._baseclasses import (
    ComponentBatchMixin,
    ComponentMixin,
)

from .. import datatypes

__all__ = ["AffixFuzzer20", "AffixFuzzer20Batch"]


class AffixFuzzer20(datatypes.AffixFuzzer20, ComponentMixin):
    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of AffixFuzzer20Ext in affix_fuzzer20_ext.py

    # Note: there are no fields here because AffixFuzzer20 delegates to datatypes.AffixFuzzer20
    pass


class AffixFuzzer20Batch(datatypes.AffixFuzzer20Batch, ComponentBatchMixin):
    _COMPONENT_NAME: str = "rerun.testing.components.AffixFuzzer20"


# This is patched in late to avoid circular dependencies.
AffixFuzzer20._BATCH_TYPE = AffixFuzzer20Batch  # type: ignore[assignment]
