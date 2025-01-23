// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/line_grid3d.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::try_serialize_field;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, ComponentBatchCowWithDescriptor, SerializedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: Configuration for the 3D line grid.
#[derive(Clone, Debug, Default)]
pub struct LineGrid3D {
    /// Whether the grid is visible.
    ///
    /// Defaults to true.
    pub visible: Option<SerializedComponentBatch>,

    /// Space between grid lines spacing of one line to the next in scene units.
    ///
    /// As you zoom out, successively only every tenth line is shown.
    /// This controls the closest zoom level.
    pub spacing: Option<SerializedComponentBatch>,

    /// In what plane the grid is drawn.
    ///
    /// Defaults to whatever plane is determined as the plane at zero units up/down as defined by [`components::ViewCoordinates`][crate::components::ViewCoordinates] if present.
    pub plane: Option<SerializedComponentBatch>,

    /// How thick the lines should be in ui units.
    ///
    /// Default is 1.0 ui unit.
    pub stroke_width: Option<SerializedComponentBatch>,

    /// Color used for the grid.
    ///
    /// Transparency via alpha channel is supported.
    /// Defaults to a slightly transparent light gray.
    pub color: Option<SerializedComponentBatch>,
}

impl LineGrid3D {
    /// Returns the [`ComponentDescriptor`] for [`Self::visible`].
    #[inline]
    pub fn descriptor_visible() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.LineGrid3D".into()),
            component_name: "rerun.blueprint.components.Visible".into(),
            archetype_field_name: Some("visible".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::spacing`].
    #[inline]
    pub fn descriptor_spacing() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.LineGrid3D".into()),
            component_name: "rerun.blueprint.components.GridSpacing".into(),
            archetype_field_name: Some("spacing".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::plane`].
    #[inline]
    pub fn descriptor_plane() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.LineGrid3D".into()),
            component_name: "rerun.components.Plane3D".into(),
            archetype_field_name: Some("plane".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::stroke_width`].
    #[inline]
    pub fn descriptor_stroke_width() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.LineGrid3D".into()),
            component_name: "rerun.components.StrokeWidth".into(),
            archetype_field_name: Some("stroke_width".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::color`].
    #[inline]
    pub fn descriptor_color() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.LineGrid3D".into()),
            component_name: "rerun.components.Color".into(),
            archetype_field_name: Some("color".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the associated indicator component.
    #[inline]
    pub fn descriptor_indicator() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.LineGrid3D".into()),
            component_name: "rerun.blueprint.components.LineGrid3DIndicator".into(),
            archetype_field_name: None,
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [LineGrid3D::descriptor_indicator()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 5usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            LineGrid3D::descriptor_visible(),
            LineGrid3D::descriptor_spacing(),
            LineGrid3D::descriptor_plane(),
            LineGrid3D::descriptor_stroke_width(),
            LineGrid3D::descriptor_color(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 6usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            LineGrid3D::descriptor_indicator(),
            LineGrid3D::descriptor_visible(),
            LineGrid3D::descriptor_spacing(),
            LineGrid3D::descriptor_plane(),
            LineGrid3D::descriptor_stroke_width(),
            LineGrid3D::descriptor_color(),
        ]
    });

impl LineGrid3D {
    /// The total number of components in the archetype: 0 required, 1 recommended, 5 optional
    pub const NUM_COMPONENTS: usize = 6usize;
}

/// Indicator component for the [`LineGrid3D`] [`::re_types_core::Archetype`]
pub type LineGrid3DIndicator = ::re_types_core::GenericIndicatorComponent<LineGrid3D>;

impl ::re_types_core::Archetype for LineGrid3D {
    type Indicator = LineGrid3DIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.blueprint.archetypes.LineGrid3D".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Line grid 3D"
    }

    #[inline]
    fn indicator() -> ComponentBatchCowWithDescriptor<'static> {
        static INDICATOR: LineGrid3DIndicator = LineGrid3DIndicator::DEFAULT;
        ComponentBatchCowWithDescriptor::new(&INDICATOR as &dyn ::re_types_core::ComponentBatch)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentDescriptor, arrow::array::ArrayRef)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_descr: ::nohash_hasher::IntMap<_, _> = arrow_data.into_iter().collect();
        let visible = arrays_by_descr
            .get(&Self::descriptor_visible())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_visible()));
        let spacing = arrays_by_descr
            .get(&Self::descriptor_spacing())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_spacing()));
        let plane = arrays_by_descr
            .get(&Self::descriptor_plane())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_plane()));
        let stroke_width = arrays_by_descr
            .get(&Self::descriptor_stroke_width())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_stroke_width())
            });
        let color = arrays_by_descr
            .get(&Self::descriptor_color())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_color()));
        Ok(Self {
            visible,
            spacing,
            plane,
            stroke_width,
            color,
        })
    }
}

impl ::re_types_core::AsComponents for LineGrid3D {
    #[inline]
    fn as_serialized_batches(&self) -> Vec<SerializedComponentBatch> {
        use ::re_types_core::Archetype as _;
        [
            Self::indicator().serialized(),
            self.visible.clone(),
            self.spacing.clone(),
            self.plane.clone(),
            self.stroke_width.clone(),
            self.color.clone(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for LineGrid3D {}

impl LineGrid3D {
    /// Create a new `LineGrid3D`.
    #[inline]
    pub fn new() -> Self {
        Self {
            visible: None,
            spacing: None,
            plane: None,
            stroke_width: None,
            color: None,
        }
    }

    /// Update only some specific fields of a `LineGrid3D`.
    #[inline]
    pub fn update_fields() -> Self {
        Self::default()
    }

    /// Clear all the fields of a `LineGrid3D`.
    #[inline]
    pub fn clear_fields() -> Self {
        use ::re_types_core::Loggable as _;
        Self {
            visible: Some(SerializedComponentBatch::new(
                crate::blueprint::components::Visible::arrow_empty(),
                Self::descriptor_visible(),
            )),
            spacing: Some(SerializedComponentBatch::new(
                crate::blueprint::components::GridSpacing::arrow_empty(),
                Self::descriptor_spacing(),
            )),
            plane: Some(SerializedComponentBatch::new(
                crate::components::Plane3D::arrow_empty(),
                Self::descriptor_plane(),
            )),
            stroke_width: Some(SerializedComponentBatch::new(
                crate::components::StrokeWidth::arrow_empty(),
                Self::descriptor_stroke_width(),
            )),
            color: Some(SerializedComponentBatch::new(
                crate::components::Color::arrow_empty(),
                Self::descriptor_color(),
            )),
        }
    }

    /// Whether the grid is visible.
    ///
    /// Defaults to true.
    #[inline]
    pub fn with_visible(
        mut self,
        visible: impl Into<crate::blueprint::components::Visible>,
    ) -> Self {
        self.visible = try_serialize_field(Self::descriptor_visible(), [visible]);
        self
    }

    /// Space between grid lines spacing of one line to the next in scene units.
    ///
    /// As you zoom out, successively only every tenth line is shown.
    /// This controls the closest zoom level.
    #[inline]
    pub fn with_spacing(
        mut self,
        spacing: impl Into<crate::blueprint::components::GridSpacing>,
    ) -> Self {
        self.spacing = try_serialize_field(Self::descriptor_spacing(), [spacing]);
        self
    }

    /// In what plane the grid is drawn.
    ///
    /// Defaults to whatever plane is determined as the plane at zero units up/down as defined by [`components::ViewCoordinates`][crate::components::ViewCoordinates] if present.
    #[inline]
    pub fn with_plane(mut self, plane: impl Into<crate::components::Plane3D>) -> Self {
        self.plane = try_serialize_field(Self::descriptor_plane(), [plane]);
        self
    }

    /// How thick the lines should be in ui units.
    ///
    /// Default is 1.0 ui unit.
    #[inline]
    pub fn with_stroke_width(
        mut self,
        stroke_width: impl Into<crate::components::StrokeWidth>,
    ) -> Self {
        self.stroke_width = try_serialize_field(Self::descriptor_stroke_width(), [stroke_width]);
        self
    }

    /// Color used for the grid.
    ///
    /// Transparency via alpha channel is supported.
    /// Defaults to a slightly transparent light gray.
    #[inline]
    pub fn with_color(mut self, color: impl Into<crate::components::Color>) -> Self {
        self.color = try_serialize_field(Self::descriptor_color(), [color]);
        self
    }
}

impl ::re_byte_size::SizeBytes for LineGrid3D {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.visible.heap_size_bytes()
            + self.spacing.heap_size_bytes()
            + self.plane.heap_size_bytes()
            + self.stroke_width.heap_size_bytes()
            + self.color.heap_size_bytes()
    }
}
