// This file is @generated by prost-build.
/// unique recording identifier. At this point in time it is the same id as the ChunkStore's StoreId
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordingId {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// A recording can have multiple timelines, each is identified by a name, for example `log_tick`, `log_time`, etc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Timeline {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A time range between start and end time points. Each 64 bit number can represent different time point data
/// depending on the timeline it is associated with. Time range is inclusive for both start and end time points.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TimeRange {
    #[prost(int64, tag = "1")]
    pub start: i64,
    #[prost(int64, tag = "2")]
    pub end: i64,
}
/// arrow IPC serialized schema
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schema {
    #[prost(bytes = "vec", tag = "1")]
    pub arrow_schema: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Query {
    /// The subset of the database that the query will run on: a set of EntityPath(s) and their
    /// associated Component(s)
    #[prost(message, optional, tag = "1")]
    pub view_contents: ::core::option::Option<ViewContents>,
    /// Whether the view_contents should ignore semantically empty columns
    /// A semantically empty column is a column that either contains no data at all, or where all
    /// values are either nulls or empty arrays (\[\]).
    #[prost(bool, tag = "2")]
    pub include_semantically_empty_columns: bool,
    /// Whether the view_contents should ignore columns corresponding to indicator components
    /// Indicator components are marker components, generally automatically inserted by Rerun, that
    /// helps keep track of the original context in which a piece of data was logged/sent.
    #[prost(bool, tag = "3")]
    pub include_indicator_columns: bool,
    /// Whether the view_contents should ignore columns corresponding to Clear-related components
    #[prost(bool, tag = "4")]
    pub include_tombstone_columns: bool,
    /// The index used to filter out _rows_ from the view contents.
    /// Only rows where at least 1 column contains non-null data at that index will be kept in the
    /// final dataset. If left unspecified, the results will only contain static data.
    #[prost(message, optional, tag = "5")]
    pub filtered_index: ::core::option::Option<IndexColumnSelector>,
    /// The range of index values used to filter out _rows_ from the view contents
    /// Only rows where at least 1 of the view-contents contains non-null data within that range will be kept in
    /// the final dataset.
    /// This has no effect if filtered_index isn't set.
    /// This has no effect if using_index_values is set.
    #[prost(message, optional, tag = "6")]
    pub filtered_index_range: ::core::option::Option<IndexRange>,
    /// The specific index values used to filter out _rows_ from the view contents.
    /// Only rows where at least 1 column contains non-null data at these specific values will be kept
    /// in the final dataset.
    /// This has no effect if filtered_index isn't set.
    /// This has no effect if using_index_values is set.
    #[prost(message, optional, tag = "7")]
    pub filtered_index_values: ::core::option::Option<IndexValues>,
    /// The specific index values used to sample _rows_ from the view contents.
    /// The final dataset will contain one row per sampled index value, regardless of whether data
    /// existed for that index value in the view contents.
    /// The semantics of the query are consistent with all other settings: the results will be
    /// sorted on the filtered_index, and only contain unique index values.
    ///
    /// This has no effect if filtered_index isn't set.
    /// If set, this overrides both filtered_index_range and filtered_index_values.
    #[prost(message, optional, tag = "8")]
    pub using_index_values: ::core::option::Option<IndexValues>,
    /// The component column used to filter out _rows_ from the view contents.
    /// Only rows where this column contains non-null data be kept in the final dataset.
    #[prost(message, optional, tag = "9")]
    pub filtered_is_not_null: ::core::option::Option<ComponentColumnSelector>,
    /// / The specific _columns_ to sample from the final view contents.
    /// / The order of the samples will be respected in the final result.
    /// /
    /// / If unspecified, it means - everything.
    #[prost(message, optional, tag = "10")]
    pub column_selection: ::core::option::Option<ColumnSelection>,
    /// Specifies how null values should be filled in the returned dataframe.
    #[prost(enumeration = "SparseFillStrategy", tag = "11")]
    pub sparse_fill_strategy: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnSelection {
    #[prost(message, repeated, tag = "1")]
    pub columns: ::prost::alloc::vec::Vec<ColumnSelector>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnSelector {
    #[prost(oneof = "column_selector::SelectorType", tags = "2, 3")]
    pub selector_type: ::core::option::Option<column_selector::SelectorType>,
}
/// Nested message and enum types in `ColumnSelector`.
pub mod column_selector {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SelectorType {
        #[prost(message, tag = "2")]
        ComponentColumn(super::ComponentColumnSelector),
        #[prost(message, tag = "3")]
        TimeColumn(super::TimeColumnSelector),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexColumnSelector {
    /// TODO(zehiko) we need to add support for other types of index selectors
    #[prost(message, optional, tag = "1")]
    pub timeline: ::core::option::Option<Timeline>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct IndexRange {
    /// TODO(zehiko) support for other ranges for other index selectors
    #[prost(message, optional, tag = "1")]
    pub time_range: ::core::option::Option<TimeRange>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexValues {
    /// TODO(zehiko) we need to add support for other types of index selectors
    #[prost(message, repeated, tag = "1")]
    pub time_points: ::prost::alloc::vec::Vec<TimeInt>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SampledIndexValues {
    #[prost(message, repeated, tag = "1")]
    pub sample_points: ::prost::alloc::vec::Vec<TimeInt>,
}
/// A 64-bit number describing either nanoseconds, sequence numbers or fully static data.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TimeInt {
    #[prost(int64, tag = "1")]
    pub time: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewContents {
    #[prost(message, repeated, tag = "1")]
    pub contents: ::prost::alloc::vec::Vec<ViewContentsPart>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewContentsPart {
    #[prost(message, optional, tag = "1")]
    pub path: ::core::option::Option<EntityPath>,
    #[prost(message, optional, tag = "2")]
    pub components: ::core::option::Option<ComponentsSet>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComponentsSet {
    #[prost(message, repeated, tag = "1")]
    pub components: ::prost::alloc::vec::Vec<Component>,
}
/// The unique identifier of an entity, e.g. `camera/3/points`
/// See <<https://www.rerun.io/docs/concepts/entity-path>> for more on entity paths.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityPath {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
}
/// Component describes semantic data that can be used by any number of  rerun's archetypes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Component {
    /// component name needs to be a string as user can define their own component
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Used to telect a time column.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeColumnSelector {
    #[prost(message, optional, tag = "1")]
    pub timeline: ::core::option::Option<Timeline>,
}
/// Used to select a component based on its EntityPath and Component name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComponentColumnSelector {
    #[prost(message, optional, tag = "1")]
    pub entity_path: ::core::option::Option<EntityPath>,
    #[prost(message, optional, tag = "2")]
    pub component: ::core::option::Option<Component>,
}
/// Specifies how null values should be filled in the returned dataframe.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SparseFillStrategy {
    None = 0,
    LatestAtGlobal = 1,
}
impl SparseFillStrategy {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::LatestAtGlobal => "LATEST_AT_GLOBAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NONE" => Some(Self::None),
            "LATEST_AT_GLOBAL" => Some(Self::LatestAtGlobal),
            _ => None,
        }
    }
}
/// Error codes for application level errors
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ErrorCode {
    /// unused
    Unused = 0,
    /// object store access error
    ObjectStoreError = 1,
    /// metadata database access error
    MetadataDbError = 2,
}
impl ErrorCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unused => "_UNUSED",
            Self::ObjectStoreError => "OBJECT_STORE_ERROR",
            Self::MetadataDbError => "METADATA_DB_ERROR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "_UNUSED" => Some(Self::Unused),
            "OBJECT_STORE_ERROR" => Some(Self::ObjectStoreError),
            "METADATA_DB_ERROR" => Some(Self::MetadataDbError),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterRecordingsRequest {
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub obj_storage: ::core::option::Option<ObjectStorage>,
    /// TODO(zehiko) should this be auto-discoverable?
    #[prost(enumeration = "RecordingType", tag = "3")]
    pub typ: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectStorage {
    #[prost(string, tag = "1")]
    pub bucket_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterRecordingsResponse {
    /// Note / TODO(zehiko): this implies we read the record (for example go through entire .rrd file
    /// chunk by chunk) and extract the metadata. So we might want to 1/ not do this i.e.
    /// only do it as part of explicit GetMetadata request or 2/ do it if Request has "include_metadata=true"
    /// or 3/ do it always
    #[prost(message, repeated, tag = "2")]
    pub metadata: ::prost::alloc::vec::Vec<RecordingMetadata>,
}
/// Server can include details about the error as part of gRPC error (Status)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistrationError {
    /// error code
    #[prost(enumeration = "ErrorCode", tag = "1")]
    pub code: i32,
    /// url of the recording that failed to register
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    /// human readable details about the error
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRecordingMetadataRequest {
    #[prost(message, optional, tag = "1")]
    pub recording_id: ::core::option::Option<RecordingId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRecordingMetadataResponse {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<RecordingMetadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordingMetadata {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<RecordingId>,
    #[prost(message, optional, tag = "2")]
    pub schema: ::core::option::Option<Schema>,
    #[prost(message, repeated, tag = "3")]
    pub time_metadata: ::prost::alloc::vec::Vec<TimeMetadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeMetadata {
    #[prost(message, optional, tag = "1")]
    pub timeline: ::core::option::Option<Timeline>,
    #[prost(message, optional, tag = "2")]
    pub time_range: ::core::option::Option<TimeRange>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequest {
    /// unique identifier of the recording
    #[prost(message, optional, tag = "1")]
    pub recording_id: ::core::option::Option<RecordingId>,
    /// query to execute
    #[prost(message, optional, tag = "3")]
    pub query: ::core::option::Option<Query>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResponse {
    /// TODO(zehiko) we need to expand this to become something like 'encoder options'
    /// as we will need to specify additional options like compression, including schema
    /// in payload, etc.
    #[prost(enumeration = "EncoderVersion", tag = "1")]
    pub encoder_version: i32,
    /// payload is raw bytes that the relevant codec can interpret
    #[prost(bytes = "vec", tag = "2")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ListRecordingsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRecordingsResponse {
    #[prost(message, repeated, tag = "1")]
    pub recordings: ::prost::alloc::vec::Vec<RecordingInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordingInfo {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<RecordingId>,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub storage_url: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub size_bytes: u64,
    #[prost(enumeration = "RecordingType", tag = "5")]
    pub typ: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EncoderVersion {
    V0 = 0,
}
impl EncoderVersion {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::V0 => "V0",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "V0" => Some(Self::V0),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RecordingType {
    Rrd = 0,
}
impl RecordingType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Rrd => "RRD",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RRD" => Some(Self::Rrd),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod storage_node_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value
    )]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct StorageNodeClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StorageNodeClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> StorageNodeClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> StorageNodeClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            StorageNodeClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn list_recordings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRecordingsRequest>,
        ) -> std::result::Result<tonic::Response<super::ListRecordingsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/rerun.remote_store.v0.StorageNode/ListRecordings",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "rerun.remote_store.v0.StorageNode",
                "ListRecordings",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::QueryResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/rerun.remote_store.v0.StorageNode/Query");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "rerun.remote_store.v0.StorageNode",
                "Query",
            ));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn get_recording_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRecordingMetadataRequest>,
        ) -> std::result::Result<tonic::Response<super::GetRecordingMetadataResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/rerun.remote_store.v0.StorageNode/GetRecordingMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "rerun.remote_store.v0.StorageNode",
                "GetRecordingMetadata",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// TODO(zehiko) - should this be singular recording registration? Currently we can have 1 rrd => many recordings
        pub async fn register_recordings(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterRecordingsRequest>,
        ) -> std::result::Result<tonic::Response<super::RegisterRecordingsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/rerun.remote_store.v0.StorageNode/RegisterRecordings",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "rerun.remote_store.v0.StorageNode",
                "RegisterRecordings",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod storage_node_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with StorageNodeServer.
    #[async_trait]
    pub trait StorageNode: std::marker::Send + std::marker::Sync + 'static {
        async fn list_recordings(
            &self,
            request: tonic::Request<super::ListRecordingsRequest>,
        ) -> std::result::Result<tonic::Response<super::ListRecordingsResponse>, tonic::Status>;
        /// Server streaming response type for the Query method.
        type QueryStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::QueryResponse, tonic::Status>,
            > + std::marker::Send
            + 'static;
        async fn query(
            &self,
            request: tonic::Request<super::QueryRequest>,
        ) -> std::result::Result<tonic::Response<Self::QueryStream>, tonic::Status>;
        async fn get_recording_metadata(
            &self,
            request: tonic::Request<super::GetRecordingMetadataRequest>,
        ) -> std::result::Result<tonic::Response<super::GetRecordingMetadataResponse>, tonic::Status>;
        /// TODO(zehiko) - should this be singular recording registration? Currently we can have 1 rrd => many recordings
        async fn register_recordings(
            &self,
            request: tonic::Request<super::RegisterRecordingsRequest>,
        ) -> std::result::Result<tonic::Response<super::RegisterRecordingsResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct StorageNodeServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> StorageNodeServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for StorageNodeServer<T>
    where
        T: StorageNode,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/rerun.remote_store.v0.StorageNode/ListRecordings" => {
                    #[allow(non_camel_case_types)]
                    struct ListRecordingsSvc<T: StorageNode>(pub Arc<T>);
                    impl<T: StorageNode> tonic::server::UnaryService<super::ListRecordingsRequest>
                        for ListRecordingsSvc<T>
                    {
                        type Response = super::ListRecordingsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListRecordingsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StorageNode>::list_recordings(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListRecordingsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rerun.remote_store.v0.StorageNode/Query" => {
                    #[allow(non_camel_case_types)]
                    struct QuerySvc<T: StorageNode>(pub Arc<T>);
                    impl<T: StorageNode> tonic::server::ServerStreamingService<super::QueryRequest> for QuerySvc<T> {
                        type Response = super::QueryResponse;
                        type ResponseStream = T::QueryStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as StorageNode>::query(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = QuerySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rerun.remote_store.v0.StorageNode/GetRecordingMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct GetRecordingMetadataSvc<T: StorageNode>(pub Arc<T>);
                    impl<T: StorageNode>
                        tonic::server::UnaryService<super::GetRecordingMetadataRequest>
                        for GetRecordingMetadataSvc<T>
                    {
                        type Response = super::GetRecordingMetadataResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRecordingMetadataRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StorageNode>::get_recording_metadata(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetRecordingMetadataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rerun.remote_store.v0.StorageNode/RegisterRecordings" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterRecordingsSvc<T: StorageNode>(pub Arc<T>);
                    impl<T: StorageNode>
                        tonic::server::UnaryService<super::RegisterRecordingsRequest>
                        for RegisterRecordingsSvc<T>
                    {
                        type Response = super::RegisterRecordingsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RegisterRecordingsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StorageNode>::register_recordings(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RegisterRecordingsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    let mut response = http::Response::new(empty_body());
                    let headers = response.headers_mut();
                    headers.insert(
                        tonic::Status::GRPC_STATUS,
                        (tonic::Code::Unimplemented as i32).into(),
                    );
                    headers.insert(
                        http::header::CONTENT_TYPE,
                        tonic::metadata::GRPC_CONTENT_TYPE,
                    );
                    Ok(response)
                }),
            }
        }
    }
    impl<T> Clone for StorageNodeServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "rerun.remote_store.v0.StorageNode";
    impl<T> tonic::server::NamedService for StorageNodeServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
