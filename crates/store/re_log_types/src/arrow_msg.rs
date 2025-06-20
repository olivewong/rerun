//! [`ArrowMsg`] is the [`crate::LogMsg`] sub-type containing an Arrow payload.
//!
//! We have custom implementations of [`serde::Serialize`] and [`serde::Deserialize`] that wraps
//! the inner Arrow serialization of an [`ArrowRecordBatch`].

use std::sync::Arc;

use arrow::array::RecordBatch as ArrowRecordBatch;

/// An arbitrary callback to be run when an [`ArrowMsg`], and more specifically the
/// [`ArrowRecordBatch`] within it, goes out of scope.
///
/// If the [`ArrowMsg`] has been cloned in a bunch of places, the callback will run for each and
/// every instance.
/// It is up to the callback implementer to handle this, if needed.
//
// TODO(#6412): probably don't need this anymore.
#[allow(clippy::type_complexity)]
#[derive(Clone)]
pub struct ArrowRecordBatchReleaseCallback(Arc<dyn Fn(ArrowRecordBatch) + Send + Sync>);

impl std::ops::Deref for ArrowRecordBatchReleaseCallback {
    type Target = dyn Fn(ArrowRecordBatch) + Send + Sync;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<F> From<F> for ArrowRecordBatchReleaseCallback
where
    F: Fn(ArrowRecordBatch) + Send + Sync + 'static,
{
    #[inline]
    fn from(f: F) -> Self {
        Self(Arc::new(f))
    }
}

impl ArrowRecordBatchReleaseCallback {
    #[inline]
    fn as_ptr(&self) -> *const () {
        Arc::as_ptr(&self.0).cast::<()>()
    }
}

impl PartialEq for ArrowRecordBatchReleaseCallback {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for ArrowRecordBatchReleaseCallback {}

impl std::fmt::Debug for ArrowRecordBatchReleaseCallback {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ArrowRecordBatchReleaseCallback")
            .field(&format!("{:p}", self.as_ptr()))
            .finish()
    }
}

/// Message containing an Arrow payload
#[derive(Clone, Debug, PartialEq)]
#[must_use]
pub struct ArrowMsg {
    /// Unique identifier for the chunk in this message.
    pub chunk_id: re_tuid::Tuid,

    /// Schema and data for all control & data columns.
    pub batch: ArrowRecordBatch,

    pub on_release: Option<ArrowRecordBatchReleaseCallback>,
}

impl Drop for ArrowMsg {
    fn drop(&mut self) {
        if let Some(on_release) = self.on_release.take() {
            (*on_release)(self.batch.clone() /* shallow */);
        }
    }
}

impl ArrowMsg {
    /// Insert a metadata key-value pair into the record batch of an [`ArrowMsg`].
    // TODO(apache/arrow-rs#7628): remove once we can do this with arrow-rs directly.
    pub fn with_record_batch_metadata(self, key: String, value: String) -> Self {
        Self {
            chunk_id: self.chunk_id,
            batch: re_arrow_util::insert_metadata(self.batch.clone(), key, value),
            on_release: self.on_release.clone(),
        }
    }
}
