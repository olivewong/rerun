---
title: "LineStrips3D"
---
<!-- DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/docs/website.rs -->

3D line strips with positions and optional colors, radii, labels, etc.

## Fields
### Required
* `strips`: [`LineStrip3D`](../components/line_strip3d.md)

### Recommended
* `radii`: [`Radius`](../components/radius.md)
* `colors`: [`Color`](../components/color.md)

### Optional
* `labels`: [`Text`](../components/text.md)
* `show_labels`: [`ShowLabels`](../components/show_labels.md)
* `class_ids`: [`ClassId`](../components/class_id.md)


## Can be shown in
* [Spatial3DView](../views/spatial3d_view.md)
* [Spatial2DView](../views/spatial2d_view.md) (if logged above active projection)
* [DataframeView](../views/dataframe_view.md)

## API reference links
 * 🌊 [C++ API docs for `LineStrips3D`](https://ref.rerun.io/docs/cpp/stable/structrerun_1_1archetypes_1_1LineStrips3D.html)
 * 🐍 [Python API docs for `LineStrips3D`](https://ref.rerun.io/docs/python/stable/common/archetypes#rerun.archetypes.LineStrips3D)
 * 🦀 [Rust API docs for `LineStrips3D`](https://docs.rs/rerun/latest/rerun/archetypes/struct.LineStrips3D.html)

## Examples

### Simple example

snippet: archetypes/line_strips3d_simple

<picture data-inline-viewer="snippets/line_strips3d_simple">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/line_strip3d_simple/13036c0e71f78d3cec37d5724f97b47c4cf3c429/480w.png">
  <source media="(max-width: 768px)" srcset="https://static.rerun.io/line_strip3d_simple/13036c0e71f78d3cec37d5724f97b47c4cf3c429/768w.png">
  <source media="(max-width: 1024px)" srcset="https://static.rerun.io/line_strip3d_simple/13036c0e71f78d3cec37d5724f97b47c4cf3c429/1024w.png">
  <source media="(max-width: 1200px)" srcset="https://static.rerun.io/line_strip3d_simple/13036c0e71f78d3cec37d5724f97b47c4cf3c429/1200w.png">
  <img src="https://static.rerun.io/line_strip3d_simple/13036c0e71f78d3cec37d5724f97b47c4cf3c429/full.png">
</picture>

### Many individual segments

snippet: archetypes/line_strips3d_segments_simple

<picture data-inline-viewer="snippets/line_strips3d_segments_simple">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/line_segment3d_simple/aa800b2a6e6a7b8e32e762b42861bae36f5014bb/480w.png">
  <source media="(max-width: 768px)" srcset="https://static.rerun.io/line_segment3d_simple/aa800b2a6e6a7b8e32e762b42861bae36f5014bb/768w.png">
  <source media="(max-width: 1024px)" srcset="https://static.rerun.io/line_segment3d_simple/aa800b2a6e6a7b8e32e762b42861bae36f5014bb/1024w.png">
  <source media="(max-width: 1200px)" srcset="https://static.rerun.io/line_segment3d_simple/aa800b2a6e6a7b8e32e762b42861bae36f5014bb/1200w.png">
  <img src="https://static.rerun.io/line_segment3d_simple/aa800b2a6e6a7b8e32e762b42861bae36f5014bb/full.png">
</picture>

### Many strips

snippet: archetypes/line_strips3d_batch

<picture data-inline-viewer="snippets/line_strips3d_batch">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/line_strip3d_batch/15e8ff18a6c95a3191acb0eae6eb04adea3b4874/480w.png">
  <source media="(max-width: 768px)" srcset="https://static.rerun.io/line_strip3d_batch/15e8ff18a6c95a3191acb0eae6eb04adea3b4874/768w.png">
  <source media="(max-width: 1024px)" srcset="https://static.rerun.io/line_strip3d_batch/15e8ff18a6c95a3191acb0eae6eb04adea3b4874/1024w.png">
  <source media="(max-width: 1200px)" srcset="https://static.rerun.io/line_strip3d_batch/15e8ff18a6c95a3191acb0eae6eb04adea3b4874/1200w.png">
  <img src="https://static.rerun.io/line_strip3d_batch/15e8ff18a6c95a3191acb0eae6eb04adea3b4874/full.png">
</picture>

### Lines with scene & UI radius each

snippet: archetypes/line_strips3d_ui_radius

<picture data-inline-viewer="snippets/line_strips3d_ui_radius">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/line_strip3d_ui_radius/36b98f47e45747b5a3601511ff39b8d74c61d120/480w.png">
  <source media="(max-width: 768px)" srcset="https://static.rerun.io/line_strip3d_ui_radius/36b98f47e45747b5a3601511ff39b8d74c61d120/768w.png">
  <source media="(max-width: 1024px)" srcset="https://static.rerun.io/line_strip3d_ui_radius/36b98f47e45747b5a3601511ff39b8d74c61d120/1024w.png">
  <source media="(max-width: 1200px)" srcset="https://static.rerun.io/line_strip3d_ui_radius/36b98f47e45747b5a3601511ff39b8d74c61d120/1200w.png">
  <img src="https://static.rerun.io/line_strip3d_ui_radius/36b98f47e45747b5a3601511ff39b8d74c61d120/full.png">
</picture>

