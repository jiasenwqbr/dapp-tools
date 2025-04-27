#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Swap {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub pair_address: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub token0_address: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub token1_address: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag = "7")]
    pub log_ordinal: u64,
    #[prost(string, tag = "8")]
    pub amount0_in: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub amount1_in: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub amount0_out: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub amount1_out: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Swaps {
    #[prost(message, repeated, tag = "1")]
    pub swaps: ::prost::alloc::vec::Vec<Swap>,
}
