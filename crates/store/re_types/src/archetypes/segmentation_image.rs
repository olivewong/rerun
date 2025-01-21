// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/segmentation_image.fbs".

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

/// **Archetype**: An image made up of integer [`components::ClassId`][crate::components::ClassId]s.
///
/// Each pixel corresponds to a [`components::ClassId`][crate::components::ClassId] that will be mapped to a color based on annotation context.
///
/// In the case of floating point images, the label will be looked up based on rounding to the nearest
/// integer value.
///
/// See also [`archetypes::AnnotationContext`][crate::archetypes::AnnotationContext] to associate each class with a color and a label.
///
/// ## Example
///
/// ### Simple segmentation image
/// ```ignore
/// use ndarray::{s, Array, ShapeBuilder};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_segmentation_image").spawn()?;
///
///     // create a segmentation image
///     let mut image = Array::<u8, _>::zeros((8, 12).f());
///     image.slice_mut(s![0..4, 0..6]).fill(1);
///     image.slice_mut(s![4..8, 6..12]).fill(2);
///
///     // create an annotation context to describe the classes
///     let annotation = rerun::AnnotationContext::new([
///         (1, "red", rerun::Rgba32::from_rgb(255, 0, 0)),
///         (2, "green", rerun::Rgba32::from_rgb(0, 255, 0)),
///     ]);
///
///     // log the annotation and the image
///     rec.log_static("/", &annotation)?;
///
///     rec.log("image", &rerun::SegmentationImage::try_from(image)?)?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/segmentation_image_simple/f8aac62abcf4c59c5d62f9ebc2d86fd0285c1736/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/segmentation_image_simple/f8aac62abcf4c59c5d62f9ebc2d86fd0285c1736/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/segmentation_image_simple/f8aac62abcf4c59c5d62f9ebc2d86fd0285c1736/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/segmentation_image_simple/f8aac62abcf4c59c5d62f9ebc2d86fd0285c1736/1200w.png">
///   <img src="https://static.rerun.io/segmentation_image_simple/f8aac62abcf4c59c5d62f9ebc2d86fd0285c1736/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SegmentationImage {
    /// The raw image data.
    pub buffer: Option<SerializedComponentBatch>,

    /// The format of the image.
    pub format: Option<SerializedComponentBatch>,

    /// Opacity of the image, useful for layering the segmentation image on top of another image.
    ///
    /// Defaults to 0.5 if there's any other images in the scene, otherwise 1.0.
    pub opacity: Option<SerializedComponentBatch>,

    /// An optional floating point value that specifies the 2D drawing order.
    ///
    /// Objects with higher values are drawn on top of those with lower values.
    pub draw_order: Option<SerializedComponentBatch>,
}

impl SegmentationImage {
    /// Returns the [`ComponentDescriptor`] for [`Self::buffer`].
    #[inline]
    pub fn descriptor_buffer() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.SegmentationImage".into()),
            component_name: "rerun.components.ImageBuffer".into(),
            archetype_field_name: Some("buffer".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::format`].
    #[inline]
    pub fn descriptor_format() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.SegmentationImage".into()),
            component_name: "rerun.components.ImageFormat".into(),
            archetype_field_name: Some("format".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::opacity`].
    #[inline]
    pub fn descriptor_opacity() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.SegmentationImage".into()),
            component_name: "rerun.components.Opacity".into(),
            archetype_field_name: Some("opacity".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::draw_order`].
    #[inline]
    pub fn descriptor_draw_order() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.SegmentationImage".into()),
            component_name: "rerun.components.DrawOrder".into(),
            archetype_field_name: Some("draw_order".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the associated indicator component.
    #[inline]
    pub fn descriptor_indicator() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.SegmentationImage".into()),
            component_name: "rerun.components.SegmentationImageIndicator".into(),
            archetype_field_name: None,
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            SegmentationImage::descriptor_buffer(),
            SegmentationImage::descriptor_format(),
        ]
    });

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [SegmentationImage::descriptor_indicator()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            SegmentationImage::descriptor_opacity(),
            SegmentationImage::descriptor_draw_order(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 5usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            SegmentationImage::descriptor_buffer(),
            SegmentationImage::descriptor_format(),
            SegmentationImage::descriptor_indicator(),
            SegmentationImage::descriptor_opacity(),
            SegmentationImage::descriptor_draw_order(),
        ]
    });

impl SegmentationImage {
    /// The total number of components in the archetype: 2 required, 1 recommended, 2 optional
    pub const NUM_COMPONENTS: usize = 5usize;
}

/// Indicator component for the [`SegmentationImage`] [`::re_types_core::Archetype`]
pub type SegmentationImageIndicator = ::re_types_core::GenericIndicatorComponent<SegmentationImage>;

impl ::re_types_core::Archetype for SegmentationImage {
    type Indicator = SegmentationImageIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.SegmentationImage".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Segmentation image"
    }

    #[inline]
    fn indicator() -> ComponentBatchCowWithDescriptor<'static> {
        static INDICATOR: SegmentationImageIndicator = SegmentationImageIndicator::DEFAULT;
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
        let buffer = arrays_by_descr
            .get(&Self::descriptor_buffer())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_buffer()));
        let format = arrays_by_descr
            .get(&Self::descriptor_format())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_format()));
        let opacity = arrays_by_descr
            .get(&Self::descriptor_opacity())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_opacity()));
        let draw_order = arrays_by_descr
            .get(&Self::descriptor_draw_order())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_draw_order())
            });
        Ok(Self {
            buffer,
            format,
            opacity,
            draw_order,
        })
    }
}

impl ::re_types_core::AsComponents for SegmentationImage {
    #[inline]
    fn as_serialized_batches(&self) -> Vec<SerializedComponentBatch> {
        use ::re_types_core::Archetype as _;
        [
            Self::indicator().serialized(),
            self.buffer.clone(),
            self.format.clone(),
            self.opacity.clone(),
            self.draw_order.clone(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for SegmentationImage {}

impl SegmentationImage {
    /// Create a new `SegmentationImage`.
    #[inline]
    pub fn new(
        buffer: impl Into<crate::components::ImageBuffer>,
        format: impl Into<crate::components::ImageFormat>,
    ) -> Self {
        Self {
            buffer: try_serialize_field(Self::descriptor_buffer(), [buffer]),
            format: try_serialize_field(Self::descriptor_format(), [format]),
            opacity: None,
            draw_order: None,
        }
    }

    /// Update only some specific fields of a `SegmentationImage`.
    #[inline]
    pub fn update_fields() -> Self {
        Self::default()
    }

    /// Clear all the fields of a `SegmentationImage`.
    #[inline]
    pub fn clear_fields() -> Self {
        use ::re_types_core::Loggable as _;
        Self {
            buffer: Some(SerializedComponentBatch::new(
                crate::components::ImageBuffer::arrow_empty(),
                Self::descriptor_buffer(),
            )),
            format: Some(SerializedComponentBatch::new(
                crate::components::ImageFormat::arrow_empty(),
                Self::descriptor_format(),
            )),
            opacity: Some(SerializedComponentBatch::new(
                crate::components::Opacity::arrow_empty(),
                Self::descriptor_opacity(),
            )),
            draw_order: Some(SerializedComponentBatch::new(
                crate::components::DrawOrder::arrow_empty(),
                Self::descriptor_draw_order(),
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
            self.buffer
                .map(|buffer| buffer.partitioned(_lengths.clone()))
                .transpose()?,
            self.format
                .map(|format| format.partitioned(_lengths.clone()))
                .transpose()?,
            self.opacity
                .map(|opacity| opacity.partitioned(_lengths.clone()))
                .transpose()?,
            self.draw_order
                .map(|draw_order| draw_order.partitioned(_lengths.clone()))
                .transpose()?,
        ];
        let indicator_column =
            ::re_types_core::indicator_column::<Self>(_lengths.into_iter().count())?;
        Ok(columns.into_iter().chain([indicator_column]).flatten())
    }

    /// The raw image data.
    #[inline]
    pub fn with_buffer(mut self, buffer: impl Into<crate::components::ImageBuffer>) -> Self {
        self.buffer = try_serialize_field(Self::descriptor_buffer(), [buffer]);
        self
    }

    /// The format of the image.
    #[inline]
    pub fn with_format(mut self, format: impl Into<crate::components::ImageFormat>) -> Self {
        self.format = try_serialize_field(Self::descriptor_format(), [format]);
        self
    }

    /// Opacity of the image, useful for layering the segmentation image on top of another image.
    ///
    /// Defaults to 0.5 if there's any other images in the scene, otherwise 1.0.
    #[inline]
    pub fn with_opacity(mut self, opacity: impl Into<crate::components::Opacity>) -> Self {
        self.opacity = try_serialize_field(Self::descriptor_opacity(), [opacity]);
        self
    }

    /// An optional floating point value that specifies the 2D drawing order.
    ///
    /// Objects with higher values are drawn on top of those with lower values.
    #[inline]
    pub fn with_draw_order(mut self, draw_order: impl Into<crate::components::DrawOrder>) -> Self {
        self.draw_order = try_serialize_field(Self::descriptor_draw_order(), [draw_order]);
        self
    }
}

impl ::re_byte_size::SizeBytes for SegmentationImage {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.buffer.heap_size_bytes()
            + self.format.heap_size_bytes()
            + self.opacity.heap_size_bytes()
            + self.draw_order.heap_size_bytes()
    }
}
