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
pub struct OpRollups {
    #[prost(message, repeated, tag="1")]
    pub rollups: ::prost::alloc::vec::Vec<OpRollup>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpRollup {
    #[prost(string, optional, tag="1")]
    pub address_manager_address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub proxy_admin_address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub proxy_to_optimism_portal: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub proxy_to_l1_cross_domain_messenger: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub proxy_to_l2_output_oracle: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub proxy_to_optimism_mintable_erc20_factory: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub proxy_to_system_config: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub proxy_to_l1_standard_bridge: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="9")]
    pub proxy_to_l1erc721_bridge: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="10")]
    pub implementation_optimism_portal: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="11")]
    pub implementation_l1_cross_domain_messenger: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="12")]
    pub implementation_l2_output_oracle: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="13")]
    pub implementation_optimism_mintable_erc20_factory: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="14")]
    pub implementation_system_config: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="15")]
    pub implementation_l1_standard_bridge: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="16")]
    pub implementation_l1erc721_bridge: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag="17")]
    pub deployer_address: ::prost::alloc::string::String,
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
