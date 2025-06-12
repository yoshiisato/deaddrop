#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bug {
    #[prost(string, tag = "1")]
    pub identifier: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub file: ::prost::alloc::vec::Vec<u8>,
}
