// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/text_document.fbs".

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

/// **Archetype**: A text element intended to be displayed in its own text box.
///
/// Supports raw text and markdown.
///
/// ## Example
///
/// ### Markdown text document
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_text_document").spawn()?;
///
///     rec.log(
///         "text_document",
///         &rerun::TextDocument::new("Hello, TextDocument!"),
///     )?;
///
///     rec.log(
///         "markdown",
///         &rerun::TextDocument::from_markdown(
///             r#"
/// # Hello Markdown!
/// [Click here to see the raw text](recording://markdown:Text).
///
/// Basic formatting:
///
/// | **Feature**       | **Alternative** |
/// | ----------------- | --------------- |
/// | Plain             |                 |
/// | *italics*         | _italics_       |
/// | **bold**          | __bold__        |
/// | ~~strikethrough~~ |                 |
/// | `inline code`     |                 |
///
/// ----------------------------------
///
/// ## Support
/// - [x] [Commonmark](https://commonmark.org/help/) support
/// - [x] GitHub-style strikethrough, tables, and checkboxes
/// - Basic syntax highlighting for:
///   - [x] C and C++
///   - [x] Python
///   - [x] Rust
///   - [ ] Other languages
///
/// ## Links
/// You can link to [an entity](recording://markdown),
/// a [specific instance of an entity](recording://markdown[#0]),
/// or a [specific component](recording://markdown:Text).
///
/// Of course you can also have [normal https links](https://github.com/rerun-io/rerun), e.g. <https://rerun.io>.
///
/// ## Image
/// ![A random image](https://picsum.photos/640/480)
/// "#.trim(),
///         )
///     )?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/textdocument/babda19558ee32ed8d730495b595aee7a5e2c174/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/textdocument/babda19558ee32ed8d730495b595aee7a5e2c174/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/textdocument/babda19558ee32ed8d730495b595aee7a5e2c174/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/textdocument/babda19558ee32ed8d730495b595aee7a5e2c174/1200w.png">
///   <img src="https://static.rerun.io/textdocument/babda19558ee32ed8d730495b595aee7a5e2c174/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq, Default)]
pub struct TextDocument {
    /// Contents of the text document.
    pub text: Option<SerializedComponentBatch>,

    /// The Media Type of the text.
    ///
    /// For instance:
    /// * `text/plain`
    /// * `text/markdown`
    ///
    /// If omitted, `text/plain` is assumed.
    pub media_type: Option<SerializedComponentBatch>,
}

impl TextDocument {
    /// Returns the [`ComponentDescriptor`] for [`Self::text`].
    #[inline]
    pub fn descriptor_text() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.TextDocument".into()),
            component_name: "rerun.components.Text".into(),
            archetype_field_name: Some("text".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::media_type`].
    #[inline]
    pub fn descriptor_media_type() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.TextDocument".into()),
            component_name: "rerun.components.MediaType".into(),
            archetype_field_name: Some("media_type".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the associated indicator component.
    #[inline]
    pub fn descriptor_indicator() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.TextDocument".into()),
            component_name: "rerun.components.TextDocumentIndicator".into(),
            archetype_field_name: None,
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [TextDocument::descriptor_text()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [TextDocument::descriptor_indicator()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [TextDocument::descriptor_media_type()]);

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            TextDocument::descriptor_text(),
            TextDocument::descriptor_indicator(),
            TextDocument::descriptor_media_type(),
        ]
    });

impl TextDocument {
    /// The total number of components in the archetype: 1 required, 1 recommended, 1 optional
    pub const NUM_COMPONENTS: usize = 3usize;
}

/// Indicator component for the [`TextDocument`] [`::re_types_core::Archetype`]
pub type TextDocumentIndicator = ::re_types_core::GenericIndicatorComponent<TextDocument>;

impl ::re_types_core::Archetype for TextDocument {
    type Indicator = TextDocumentIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.TextDocument".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Text document"
    }

    #[inline]
    fn indicator() -> ComponentBatchCowWithDescriptor<'static> {
        static INDICATOR: TextDocumentIndicator = TextDocumentIndicator::DEFAULT;
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
        let text = arrays_by_descr
            .get(&Self::descriptor_text())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_text()));
        let media_type = arrays_by_descr
            .get(&Self::descriptor_media_type())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_media_type())
            });
        Ok(Self { text, media_type })
    }
}

impl ::re_types_core::AsComponents for TextDocument {
    #[inline]
    fn as_serialized_batches(&self) -> Vec<SerializedComponentBatch> {
        use ::re_types_core::Archetype as _;
        [
            Self::indicator().serialized(),
            self.text.clone(),
            self.media_type.clone(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for TextDocument {}

impl TextDocument {
    /// Create a new `TextDocument`.
    #[inline]
    pub fn new(text: impl Into<crate::components::Text>) -> Self {
        Self {
            text: try_serialize_field(Self::descriptor_text(), [text]),
            media_type: None,
        }
    }

    /// Update only some specific fields of a `TextDocument`.
    #[inline]
    pub fn update_fields() -> Self {
        Self::default()
    }

    /// Clear all the fields of a `TextDocument`.
    #[inline]
    pub fn clear_fields() -> Self {
        use ::re_types_core::Loggable as _;
        Self {
            text: Some(SerializedComponentBatch::new(
                crate::components::Text::arrow_empty(),
                Self::descriptor_text(),
            )),
            media_type: Some(SerializedComponentBatch::new(
                crate::components::MediaType::arrow_empty(),
                Self::descriptor_media_type(),
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
            self.text
                .map(|text| text.partitioned(_lengths.clone()))
                .transpose()?,
            self.media_type
                .map(|media_type| media_type.partitioned(_lengths.clone()))
                .transpose()?,
        ];
        let indicator_column =
            ::re_types_core::indicator_column::<Self>(_lengths.into_iter().count())?;
        Ok(columns.into_iter().chain([indicator_column]).flatten())
    }

    /// Contents of the text document.
    #[inline]
    pub fn with_text(mut self, text: impl Into<crate::components::Text>) -> Self {
        self.text = try_serialize_field(Self::descriptor_text(), [text]);
        self
    }

    /// The Media Type of the text.
    ///
    /// For instance:
    /// * `text/plain`
    /// * `text/markdown`
    ///
    /// If omitted, `text/plain` is assumed.
    #[inline]
    pub fn with_media_type(mut self, media_type: impl Into<crate::components::MediaType>) -> Self {
        self.media_type = try_serialize_field(Self::descriptor_media_type(), [media_type]);
        self
    }
}

impl ::re_byte_size::SizeBytes for TextDocument {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.text.heap_size_bytes() + self.media_type.heap_size_bytes()
    }
}
