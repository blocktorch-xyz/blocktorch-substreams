// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZkEvmRollups {
    #[prost(message, repeated, tag="1")]
    pub rollups: ::prost::alloc::vec::Vec<ZkEvmRollup>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZkEvmRollup {
    #[prost(string, optional, tag="1")]
    pub zk_evm_deployer_contract: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub proxy_admin_contract: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub polygon_zk_evm_timelock_contract: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub native_token_contract: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub polygon_zk_evm_implementation: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub polygon_zk_evm_bridge_implementation: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub polygon_zk_evm_global_exit_root_implementation: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub polygon_zk_evm_proxy: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="9")]
    pub polygon_zk_evm_bridge_proxy: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="10")]
    pub polygon_zk_evm_global_exit_root_proxy: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag="11")]
    pub admin_address: ::prost::alloc::string::String,
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
    pub rollup_event_inbox_address: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub challenge_manager_address: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub admin_proxy_address: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub sequencer_inbox_address: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub bridge_address: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub upgrade_executor_address: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub validator_utils_address: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub validator_wallet_creator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag="13")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CombinedRollups {
    #[prost(message, repeated, tag="1")]
    pub nitro_rollups: ::prost::alloc::vec::Vec<NitroRollup>,
    #[prost(message, repeated, tag="2")]
    pub op_rollups: ::prost::alloc::vec::Vec<OpRollup>,
    #[prost(message, repeated, tag="3")]
    pub zk_evm_rollups: ::prost::alloc::vec::Vec<ZkEvmRollup>,
}
// @@protoc_insertion_point(module)
