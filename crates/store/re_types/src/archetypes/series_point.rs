// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/series_point.fbs".

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
#![expect(deprecated)]

use ::re_types_core::try_serialize_field;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch as _, SerializedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: Define the style properties for a point series in a chart.
///
/// This archetype only provides styling information and should be logged as static
/// when possible. The underlying data needs to be logged to the same entity-path using
/// [`archetypes::Scalars`][crate::archetypes::Scalars].
///
/// ⚠️ **Deprecated since 0.23.0**: Use `SeriesPoints` instead.
///
/// ## Example
///
/// ### Point series
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_series_point_style").spawn()?;
///
///     // Set up plot styling:
///     // They are logged static as they don't change over time and apply to all timelines.
///     // Log two point series under a shared root so that they show in the same plot by default.
///     rec.log_static(
///         "trig/sin",
///         &rerun::SeriesPoints::new()
///             .with_colors([[255, 0, 0]])
///             .with_names(["sin(0.01t)"])
///             .with_markers([rerun::components::MarkerShape::Circle])
///             .with_marker_sizes([4.0]),
///     )?;
///     rec.log_static(
///         "trig/cos",
///         &rerun::SeriesPoints::new()
///             .with_colors([[0, 255, 0]])
///             .with_names(["cos(0.01t)"])
///             .with_markers([rerun::components::MarkerShape::Cross])
///             .with_marker_sizes([2.0]),
///     )?;
///
///     for t in 0..((std::f32::consts::TAU * 2.0 * 10.0) as i64) {
///         rec.set_time_sequence("step", t);
///
///         // Log two time series under a shared root so that they show in the same plot by default.
///         rec.log("trig/sin", &rerun::Scalars::single((t as f64 / 10.0).sin()))?;
///         rec.log("trig/cos", &rerun::Scalars::single((t as f64 / 10.0).cos()))?;
///     }
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/series_point_style/82207a705da6c086b28ce161db1db9e8b12258b7/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/series_point_style/82207a705da6c086b28ce161db1db9e8b12258b7/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/series_point_style/82207a705da6c086b28ce161db1db9e8b12258b7/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/series_point_style/82207a705da6c086b28ce161db1db9e8b12258b7/1200w.png">
///   <img src="https://static.rerun.io/series_point_style/82207a705da6c086b28ce161db1db9e8b12258b7/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, Default)]
#[deprecated(note = "since 0.23.0: Use `SeriesPoints` instead.")]
pub struct SeriesPoint {
    /// Color for the corresponding series.
    pub color: Option<SerializedComponentBatch>,

    /// What shape to use to represent the point
    pub marker: Option<SerializedComponentBatch>,

    /// Display name of the series.
    ///
    /// Used in the legend.
    pub name: Option<SerializedComponentBatch>,

    /// Which point series are visible.
    ///
    /// If not set, all point series on this entity are visible.
    /// Unlike with the regular visibility property of the entire entity, any series that is hidden
    /// via this property will still be visible in the legend.
    pub visible_series: Option<SerializedComponentBatch>,

    /// Size of the marker.
    pub marker_size: Option<SerializedComponentBatch>,
}

impl SeriesPoint {
    /// Returns the [`ComponentDescriptor`] for [`Self::color`].
    #[inline]
    pub fn descriptor_color() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.SeriesPoint".into()),
            component_name: "rerun.components.Color".into(),
            archetype_field_name: Some("color".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::marker`].
    #[inline]
    pub fn descriptor_marker() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.SeriesPoint".into()),
            component_name: "rerun.components.MarkerShape".into(),
            archetype_field_name: Some("marker".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::name`].
    #[inline]
    pub fn descriptor_name() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.SeriesPoint".into()),
            component_name: "rerun.components.Name".into(),
            archetype_field_name: Some("name".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::visible_series`].
    #[inline]
    pub fn descriptor_visible_series() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.SeriesPoint".into()),
            component_name: "rerun.components.SeriesVisible".into(),
            archetype_field_name: Some("visible_series".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::marker_size`].
    #[inline]
    pub fn descriptor_marker_size() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.SeriesPoint".into()),
            component_name: "rerun.components.MarkerSize".into(),
            archetype_field_name: Some("marker_size".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the associated indicator component.
    #[inline]
    pub fn descriptor_indicator() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.SeriesPoint".into()),
            component_name: "rerun.components.SeriesPointIndicator".into(),
            archetype_field_name: None,
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [SeriesPoint::descriptor_indicator()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 5usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            SeriesPoint::descriptor_color(),
            SeriesPoint::descriptor_marker(),
            SeriesPoint::descriptor_name(),
            SeriesPoint::descriptor_visible_series(),
            SeriesPoint::descriptor_marker_size(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 6usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            SeriesPoint::descriptor_indicator(),
            SeriesPoint::descriptor_color(),
            SeriesPoint::descriptor_marker(),
            SeriesPoint::descriptor_name(),
            SeriesPoint::descriptor_visible_series(),
            SeriesPoint::descriptor_marker_size(),
        ]
    });

impl SeriesPoint {
    /// The total number of components in the archetype: 0 required, 1 recommended, 5 optional
    pub const NUM_COMPONENTS: usize = 6usize;
}

/// Indicator component for the [`SeriesPoint`] [`::re_types_core::Archetype`]
pub type SeriesPointIndicator = ::re_types_core::GenericIndicatorComponent<SeriesPoint>;

impl ::re_types_core::Archetype for SeriesPoint {
    type Indicator = SeriesPointIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.SeriesPoint".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Series point"
    }

    #[inline]
    fn indicator() -> SerializedComponentBatch {
        #[allow(clippy::unwrap_used)]
        SeriesPointIndicator::DEFAULT.serialized().unwrap()
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
        let color = arrays_by_descr
            .get(&Self::descriptor_color())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_color()));
        let marker = arrays_by_descr
            .get(&Self::descriptor_marker())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_marker()));
        let name = arrays_by_descr
            .get(&Self::descriptor_name())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_name()));
        let visible_series = arrays_by_descr
            .get(&Self::descriptor_visible_series())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_visible_series())
            });
        let marker_size = arrays_by_descr
            .get(&Self::descriptor_marker_size())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_marker_size())
            });
        Ok(Self {
            color,
            marker,
            name,
            visible_series,
            marker_size,
        })
    }
}

impl ::re_types_core::AsComponents for SeriesPoint {
    #[inline]
    fn as_serialized_batches(&self) -> Vec<SerializedComponentBatch> {
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            self.color.clone(),
            self.marker.clone(),
            self.name.clone(),
            self.visible_series.clone(),
            self.marker_size.clone(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for SeriesPoint {}

impl SeriesPoint {
    /// Create a new `SeriesPoint`.
    #[inline]
    pub fn new() -> Self {
        Self {
            color: None,
            marker: None,
            name: None,
            visible_series: None,
            marker_size: None,
        }
    }

    /// Update only some specific fields of a `SeriesPoint`.
    #[inline]
    pub fn update_fields() -> Self {
        Self::default()
    }

    /// Clear all the fields of a `SeriesPoint`.
    #[inline]
    pub fn clear_fields() -> Self {
        use ::re_types_core::Loggable as _;
        Self {
            color: Some(SerializedComponentBatch::new(
                crate::components::Color::arrow_empty(),
                Self::descriptor_color(),
            )),
            marker: Some(SerializedComponentBatch::new(
                crate::components::MarkerShape::arrow_empty(),
                Self::descriptor_marker(),
            )),
            name: Some(SerializedComponentBatch::new(
                crate::components::Name::arrow_empty(),
                Self::descriptor_name(),
            )),
            visible_series: Some(SerializedComponentBatch::new(
                crate::components::SeriesVisible::arrow_empty(),
                Self::descriptor_visible_series(),
            )),
            marker_size: Some(SerializedComponentBatch::new(
                crate::components::MarkerSize::arrow_empty(),
                Self::descriptor_marker_size(),
            )),
        }
    }

    /// Partitions the component data into multiple sub-batches.
    ///
    /// Specifically, this transforms the existing [`SerializedComponentBatch`]es data into [`SerializedComponentColumn`]s
    /// instead, via [`SerializedComponentBatch::partitioned`].
    ///
    /// This makes it possible to use `RecordingStream::send_columns` to send columnar data directly into Rerun.
    ///
    /// The specified `lengths` must sum to the total length of the component batch.
    ///
    /// [`SerializedComponentColumn`]: [::re_types_core::SerializedComponentColumn]
    #[inline]
    pub fn columns<I>(
        self,
        _lengths: I,
    ) -> SerializationResult<impl Iterator<Item = ::re_types_core::SerializedComponentColumn>>
    where
        I: IntoIterator<Item = usize> + Clone,
    {
        let columns = [
            self.color
                .map(|color| color.partitioned(_lengths.clone()))
                .transpose()?,
            self.marker
                .map(|marker| marker.partitioned(_lengths.clone()))
                .transpose()?,
            self.name
                .map(|name| name.partitioned(_lengths.clone()))
                .transpose()?,
            self.visible_series
                .map(|visible_series| visible_series.partitioned(_lengths.clone()))
                .transpose()?,
            self.marker_size
                .map(|marker_size| marker_size.partitioned(_lengths.clone()))
                .transpose()?,
        ];
        Ok(columns
            .into_iter()
            .flatten()
            .chain([::re_types_core::indicator_column::<Self>(
                _lengths.into_iter().count(),
            )?]))
    }

    /// Helper to partition the component data into unit-length sub-batches.
    ///
    /// This is semantically similar to calling [`Self::columns`] with `std::iter::take(1).repeat(n)`,
    /// where `n` is automatically guessed.
    #[inline]
    pub fn columns_of_unit_batches(
        self,
    ) -> SerializationResult<impl Iterator<Item = ::re_types_core::SerializedComponentColumn>> {
        let len_color = self.color.as_ref().map(|b| b.array.len());
        let len_marker = self.marker.as_ref().map(|b| b.array.len());
        let len_name = self.name.as_ref().map(|b| b.array.len());
        let len_visible_series = self.visible_series.as_ref().map(|b| b.array.len());
        let len_marker_size = self.marker_size.as_ref().map(|b| b.array.len());
        let len = None
            .or(len_color)
            .or(len_marker)
            .or(len_name)
            .or(len_visible_series)
            .or(len_marker_size)
            .unwrap_or(0);
        self.columns(std::iter::repeat(1).take(len))
    }

    /// Color for the corresponding series.
    #[inline]
    pub fn with_color(mut self, color: impl Into<crate::components::Color>) -> Self {
        self.color = try_serialize_field(Self::descriptor_color(), [color]);
        self
    }

    /// This method makes it possible to pack multiple [`crate::components::Color`] in a single component batch.
    ///
    /// This only makes sense when used in conjunction with [`Self::columns`]. [`Self::with_color`] should
    /// be used when logging a single row's worth of data.
    #[inline]
    pub fn with_many_color(
        mut self,
        color: impl IntoIterator<Item = impl Into<crate::components::Color>>,
    ) -> Self {
        self.color = try_serialize_field(Self::descriptor_color(), color);
        self
    }

    /// What shape to use to represent the point
    #[inline]
    pub fn with_marker(mut self, marker: impl Into<crate::components::MarkerShape>) -> Self {
        self.marker = try_serialize_field(Self::descriptor_marker(), [marker]);
        self
    }

    /// This method makes it possible to pack multiple [`crate::components::MarkerShape`] in a single component batch.
    ///
    /// This only makes sense when used in conjunction with [`Self::columns`]. [`Self::with_marker`] should
    /// be used when logging a single row's worth of data.
    #[inline]
    pub fn with_many_marker(
        mut self,
        marker: impl IntoIterator<Item = impl Into<crate::components::MarkerShape>>,
    ) -> Self {
        self.marker = try_serialize_field(Self::descriptor_marker(), marker);
        self
    }

    /// Display name of the series.
    ///
    /// Used in the legend.
    #[inline]
    pub fn with_name(mut self, name: impl Into<crate::components::Name>) -> Self {
        self.name = try_serialize_field(Self::descriptor_name(), [name]);
        self
    }

    /// This method makes it possible to pack multiple [`crate::components::Name`] in a single component batch.
    ///
    /// This only makes sense when used in conjunction with [`Self::columns`]. [`Self::with_name`] should
    /// be used when logging a single row's worth of data.
    #[inline]
    pub fn with_many_name(
        mut self,
        name: impl IntoIterator<Item = impl Into<crate::components::Name>>,
    ) -> Self {
        self.name = try_serialize_field(Self::descriptor_name(), name);
        self
    }

    /// Which point series are visible.
    ///
    /// If not set, all point series on this entity are visible.
    /// Unlike with the regular visibility property of the entire entity, any series that is hidden
    /// via this property will still be visible in the legend.
    #[inline]
    pub fn with_visible_series(
        mut self,
        visible_series: impl IntoIterator<Item = impl Into<crate::components::SeriesVisible>>,
    ) -> Self {
        self.visible_series =
            try_serialize_field(Self::descriptor_visible_series(), visible_series);
        self
    }

    /// Size of the marker.
    #[inline]
    pub fn with_marker_size(
        mut self,
        marker_size: impl Into<crate::components::MarkerSize>,
    ) -> Self {
        self.marker_size = try_serialize_field(Self::descriptor_marker_size(), [marker_size]);
        self
    }

    /// This method makes it possible to pack multiple [`crate::components::MarkerSize`] in a single component batch.
    ///
    /// This only makes sense when used in conjunction with [`Self::columns`]. [`Self::with_marker_size`] should
    /// be used when logging a single row's worth of data.
    #[inline]
    pub fn with_many_marker_size(
        mut self,
        marker_size: impl IntoIterator<Item = impl Into<crate::components::MarkerSize>>,
    ) -> Self {
        self.marker_size = try_serialize_field(Self::descriptor_marker_size(), marker_size);
        self
    }
}

impl ::re_byte_size::SizeBytes for SeriesPoint {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.color.heap_size_bytes()
            + self.marker.heap_size_bytes()
            + self.name.heap_size_bytes()
            + self.visible_series.heap_size_bytes()
            + self.marker_size.heap_size_bytes()
    }
}
