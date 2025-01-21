// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/graph_nodes.fbs".

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

/// **Archetype**: A list of nodes in a graph with optional labels, colors, etc.
///
/// ## Example
///
/// ### Simple directed graph
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_graph_directed").spawn()?;
///
///     rec.log(
///         "simple",
///         &[
///             &rerun::GraphNodes::new(["a", "b", "c"])
///                 .with_positions([(0.0, 100.0), (-100.0, 0.0), (100.0, 0.0)])
///                 .with_labels(["A", "B", "C"]) as &dyn rerun::AsComponents,
///             &rerun::GraphEdges::new([("a", "b"), ("b", "c"), ("c", "a")]).with_directed_edges(),
///         ],
///     )?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/graph_directed/ca29a37b65e1e0b6482251dce401982a0bc568fa/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/graph_directed/ca29a37b65e1e0b6482251dce401982a0bc568fa/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/graph_directed/ca29a37b65e1e0b6482251dce401982a0bc568fa/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/graph_directed/ca29a37b65e1e0b6482251dce401982a0bc568fa/1200w.png">
///   <img src="https://static.rerun.io/graph_directed/ca29a37b65e1e0b6482251dce401982a0bc568fa/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq, Default)]
pub struct GraphNodes {
    /// A list of node IDs.
    pub node_ids: Option<SerializedComponentBatch>,

    /// Optional center positions of the nodes.
    pub positions: Option<SerializedComponentBatch>,

    /// Optional colors for the boxes.
    pub colors: Option<SerializedComponentBatch>,

    /// Optional text labels for the node.
    pub labels: Option<SerializedComponentBatch>,

    /// Optional choice of whether the text labels should be shown by default.
    pub show_labels: Option<SerializedComponentBatch>,

    /// Optional radii for nodes.
    pub radii: Option<SerializedComponentBatch>,
}

impl GraphNodes {
    /// Returns the [`ComponentDescriptor`] for [`Self::node_ids`].
    #[inline]
    pub fn descriptor_node_ids() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.GraphNodes".into()),
            component_name: "rerun.components.GraphNode".into(),
            archetype_field_name: Some("node_ids".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::positions`].
    #[inline]
    pub fn descriptor_positions() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.GraphNodes".into()),
            component_name: "rerun.components.Position2D".into(),
            archetype_field_name: Some("positions".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::colors`].
    #[inline]
    pub fn descriptor_colors() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.GraphNodes".into()),
            component_name: "rerun.components.Color".into(),
            archetype_field_name: Some("colors".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::labels`].
    #[inline]
    pub fn descriptor_labels() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.GraphNodes".into()),
            component_name: "rerun.components.Text".into(),
            archetype_field_name: Some("labels".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::show_labels`].
    #[inline]
    pub fn descriptor_show_labels() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.GraphNodes".into()),
            component_name: "rerun.components.ShowLabels".into(),
            archetype_field_name: Some("show_labels".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::radii`].
    #[inline]
    pub fn descriptor_radii() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.GraphNodes".into()),
            component_name: "rerun.components.Radius".into(),
            archetype_field_name: Some("radii".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the associated indicator component.
    #[inline]
    pub fn descriptor_indicator() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.GraphNodes".into()),
            component_name: "rerun.components.GraphNodesIndicator".into(),
            archetype_field_name: None,
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [GraphNodes::descriptor_node_ids()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [GraphNodes::descriptor_indicator()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 5usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            GraphNodes::descriptor_positions(),
            GraphNodes::descriptor_colors(),
            GraphNodes::descriptor_labels(),
            GraphNodes::descriptor_show_labels(),
            GraphNodes::descriptor_radii(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 7usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            GraphNodes::descriptor_node_ids(),
            GraphNodes::descriptor_indicator(),
            GraphNodes::descriptor_positions(),
            GraphNodes::descriptor_colors(),
            GraphNodes::descriptor_labels(),
            GraphNodes::descriptor_show_labels(),
            GraphNodes::descriptor_radii(),
        ]
    });

impl GraphNodes {
    /// The total number of components in the archetype: 1 required, 1 recommended, 5 optional
    pub const NUM_COMPONENTS: usize = 7usize;
}

/// Indicator component for the [`GraphNodes`] [`::re_types_core::Archetype`]
pub type GraphNodesIndicator = ::re_types_core::GenericIndicatorComponent<GraphNodes>;

impl ::re_types_core::Archetype for GraphNodes {
    type Indicator = GraphNodesIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.GraphNodes".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Graph nodes"
    }

    #[inline]
    fn indicator() -> ComponentBatchCowWithDescriptor<'static> {
        static INDICATOR: GraphNodesIndicator = GraphNodesIndicator::DEFAULT;
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
        let node_ids = arrays_by_descr
            .get(&Self::descriptor_node_ids())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_node_ids()));
        let positions = arrays_by_descr
            .get(&Self::descriptor_positions())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_positions())
            });
        let colors = arrays_by_descr
            .get(&Self::descriptor_colors())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_colors()));
        let labels = arrays_by_descr
            .get(&Self::descriptor_labels())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_labels()));
        let show_labels = arrays_by_descr
            .get(&Self::descriptor_show_labels())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_show_labels())
            });
        let radii = arrays_by_descr
            .get(&Self::descriptor_radii())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_radii()));
        Ok(Self {
            node_ids,
            positions,
            colors,
            labels,
            show_labels,
            radii,
        })
    }
}

impl ::re_types_core::AsComponents for GraphNodes {
    #[inline]
    fn as_serialized_batches(&self) -> Vec<SerializedComponentBatch> {
        use ::re_types_core::Archetype as _;
        [
            Self::indicator().serialized(),
            self.node_ids.clone(),
            self.positions.clone(),
            self.colors.clone(),
            self.labels.clone(),
            self.show_labels.clone(),
            self.radii.clone(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for GraphNodes {}

impl GraphNodes {
    /// Create a new `GraphNodes`.
    #[inline]
    pub fn new(
        node_ids: impl IntoIterator<Item = impl Into<crate::components::GraphNode>>,
    ) -> Self {
        Self {
            node_ids: try_serialize_field(Self::descriptor_node_ids(), node_ids),
            positions: None,
            colors: None,
            labels: None,
            show_labels: None,
            radii: None,
        }
    }

    /// Update only some specific fields of a `GraphNodes`.
    #[inline]
    pub fn update_fields() -> Self {
        Self::default()
    }

    /// Clear all the fields of a `GraphNodes`.
    #[inline]
    pub fn clear_fields() -> Self {
        use ::re_types_core::Loggable as _;
        Self {
            node_ids: Some(SerializedComponentBatch::new(
                crate::components::GraphNode::arrow_empty(),
                Self::descriptor_node_ids(),
            )),
            positions: Some(SerializedComponentBatch::new(
                crate::components::Position2D::arrow_empty(),
                Self::descriptor_positions(),
            )),
            colors: Some(SerializedComponentBatch::new(
                crate::components::Color::arrow_empty(),
                Self::descriptor_colors(),
            )),
            labels: Some(SerializedComponentBatch::new(
                crate::components::Text::arrow_empty(),
                Self::descriptor_labels(),
            )),
            show_labels: Some(SerializedComponentBatch::new(
                crate::components::ShowLabels::arrow_empty(),
                Self::descriptor_show_labels(),
            )),
            radii: Some(SerializedComponentBatch::new(
                crate::components::Radius::arrow_empty(),
                Self::descriptor_radii(),
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
            self.node_ids
                .map(|node_ids| node_ids.partitioned(_lengths.clone()))
                .transpose()?,
            self.positions
                .map(|positions| positions.partitioned(_lengths.clone()))
                .transpose()?,
            self.colors
                .map(|colors| colors.partitioned(_lengths.clone()))
                .transpose()?,
            self.labels
                .map(|labels| labels.partitioned(_lengths.clone()))
                .transpose()?,
            self.show_labels
                .map(|show_labels| show_labels.partitioned(_lengths.clone()))
                .transpose()?,
            self.radii
                .map(|radii| radii.partitioned(_lengths.clone()))
                .transpose()?,
        ];
        let indicator_column =
            ::re_types_core::indicator_column::<Self>(_lengths.into_iter().count())?;
        Ok(columns.into_iter().chain([indicator_column]).flatten())
    }

    /// A list of node IDs.
    #[inline]
    pub fn with_node_ids(
        mut self,
        node_ids: impl IntoIterator<Item = impl Into<crate::components::GraphNode>>,
    ) -> Self {
        self.node_ids = try_serialize_field(Self::descriptor_node_ids(), node_ids);
        self
    }

    /// Optional center positions of the nodes.
    #[inline]
    pub fn with_positions(
        mut self,
        positions: impl IntoIterator<Item = impl Into<crate::components::Position2D>>,
    ) -> Self {
        self.positions = try_serialize_field(Self::descriptor_positions(), positions);
        self
    }

    /// Optional colors for the boxes.
    #[inline]
    pub fn with_colors(
        mut self,
        colors: impl IntoIterator<Item = impl Into<crate::components::Color>>,
    ) -> Self {
        self.colors = try_serialize_field(Self::descriptor_colors(), colors);
        self
    }

    /// Optional text labels for the node.
    #[inline]
    pub fn with_labels(
        mut self,
        labels: impl IntoIterator<Item = impl Into<crate::components::Text>>,
    ) -> Self {
        self.labels = try_serialize_field(Self::descriptor_labels(), labels);
        self
    }

    /// Optional choice of whether the text labels should be shown by default.
    #[inline]
    pub fn with_show_labels(
        mut self,
        show_labels: impl Into<crate::components::ShowLabels>,
    ) -> Self {
        self.show_labels = try_serialize_field(Self::descriptor_show_labels(), [show_labels]);
        self
    }

    /// Optional radii for nodes.
    #[inline]
    pub fn with_radii(
        mut self,
        radii: impl IntoIterator<Item = impl Into<crate::components::Radius>>,
    ) -> Self {
        self.radii = try_serialize_field(Self::descriptor_radii(), radii);
        self
    }
}

impl ::re_byte_size::SizeBytes for GraphNodes {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.node_ids.heap_size_bytes()
            + self.positions.heap_size_bytes()
            + self.colors.heap_size_bytes()
            + self.labels.heap_size_bytes()
            + self.show_labels.heap_size_bytes()
            + self.radii.heap_size_bytes()
    }
}
