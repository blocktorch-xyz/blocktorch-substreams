// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChronicleTransactions {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<ChronicleTransaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChronicleTransaction {
    #[prost(string, tag="1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub hash: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub chain: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub status: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountAbstractionTransactions {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<AccountAbstractionTransaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountAbstractionTransaction {
    #[prost(string, tag="1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub hash: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub chain: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub account_abstraction_type: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub status: ::prost::alloc::string::String,
    #[prost(message, optional, tag="7")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NitroRollups {
    #[prost(message, repeated, tag="1")]
    pub rollups: ::prost::alloc::vec::Vec<NitroRollup>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NitroRollup {
    #[prost(string, tag="1")]
    pub rollup_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub native_token_address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub inbox_address: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub outbox_address: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub sequencer_inbox: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CombinedTransactions {
    #[prost(message, repeated, tag="1")]
    pub account_abstraction_transactions: ::prost::alloc::vec::Vec<AccountAbstractionTransaction>,
    #[prost(message, repeated, tag="2")]
    pub chronicle_transactions: ::prost::alloc::vec::Vec<ChronicleTransaction>,
}
// @@protoc_insertion_point(module)
