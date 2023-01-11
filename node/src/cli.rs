use clap::Parser;
use permission_resolver::RemoteAuthorityPermissionResolverFactory;
use sc_cli::{
	ChainSpec, CliConfiguration, Error, ImportParams, KeystoreParams, NetworkParams,
	OffchainWorkerParams, Role, SharedParams,
};
use sc_service::{config::PrometheusConfig, BasePath, TransactionPoolOptions};
use sc_telemetry::TelemetryEndpoints;
use sp_authority_permission::{AlwaysPermissionGrantedFactory, PermissionResolverFactory};
use std::net::SocketAddr;

#[derive(Debug, clap::Parser)]
pub struct Cli {
	#[clap(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[clap(flatten)]
	pub run: RunCmd,
}

#[derive(Debug, Clone, Parser)]
pub struct RunCmd {
	#[clap(flatten)]
	pub base: sc_cli::RunCmd,

	#[clap(long)]
	pub remote_authority: Vec<String>,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
	/// Key management cli utilities
	#[clap(subcommand)]
	Key(sc_cli::KeySubcommand),

	/// Build a chain specification.
	BuildSpec(sc_cli::BuildSpecCmd),

	/// Validate blocks.
	CheckBlock(sc_cli::CheckBlockCmd),

	/// Export blocks.
	ExportBlocks(sc_cli::ExportBlocksCmd),

	/// Export the state of a given block into a chain spec.
	ExportState(sc_cli::ExportStateCmd),

	/// Import blocks.
	ImportBlocks(sc_cli::ImportBlocksCmd),

	/// Remove the whole chain.
	PurgeChain(sc_cli::PurgeChainCmd),

	/// Revert the chain to a previous state.
	Revert(sc_cli::RevertCmd),

	/// Sub-commands concerned with benchmarking.
	#[clap(subcommand)]
	Benchmark(frame_benchmarking_cli::BenchmarkCmd),

	/// Try some command against runtime state.
	#[cfg(feature = "try-runtime")]
	TryRuntime(try_runtime_cli::TryRuntimeCmd),

	/// Try some command against runtime state. Note: `try-runtime` feature must be enabled.
	#[cfg(not(feature = "try-runtime"))]
	TryRuntime,

	/// Db meta columns information.
	ChainInfo(sc_cli::ChainInfoCmd),
}

pub type Result<T> = std::result::Result<T, Error>;

impl CliConfiguration for RunCmd {
	fn shared_params(&self) -> &SharedParams {
		&self.base.shared_params
	}

	fn import_params(&self) -> Option<&ImportParams> {
		Some(&self.base.import_params)
	}

	fn network_params(&self) -> Option<&NetworkParams> {
		Some(&self.base.network_params)
	}

	fn keystore_params(&self) -> Option<&KeystoreParams> {
		Some(&self.base.keystore_params)
	}

	fn offchain_worker_params(&self) -> Option<&OffchainWorkerParams> {
		Some(&self.base.offchain_worker_params)
	}

	fn node_name(&self) -> Result<String> {
		self.base.node_name()
	}

	fn dev_key_seed(&self, is_dev: bool) -> Result<Option<String>> {
		self.base.dev_key_seed(is_dev)
	}

	fn telemetry_endpoints(
		&self,
		chain_spec: &Box<dyn ChainSpec>,
	) -> Result<Option<TelemetryEndpoints>> {
		self.base.telemetry_endpoints(chain_spec)
	}

	fn role(&self, is_dev: bool) -> Result<Role> {
		self.base.role(is_dev)
	}

	fn force_authoring(&self) -> Result<bool> {
		self.base.force_authoring()
	}

	fn prometheus_config(
		&self,
		default_listen_port: u16,
		chain_spec: &Box<dyn ChainSpec>,
	) -> Result<Option<PrometheusConfig>> {
		self.base.prometheus_config(default_listen_port, chain_spec)
	}

	fn disable_grandpa(&self) -> Result<bool> {
		self.base.disable_grandpa()
	}

	fn rpc_ws_max_connections(&self) -> Result<Option<usize>> {
		self.base.rpc_ws_max_connections()
	}

	fn rpc_cors(&self, is_dev: bool) -> Result<Option<Vec<String>>> {
		self.base.rpc_cors(is_dev)
	}

	fn rpc_http(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
		self.base.rpc_http(default_listen_port)
	}

	fn rpc_ipc(&self) -> Result<Option<String>> {
		self.base.rpc_ipc()
	}

	fn rpc_ws(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
		self.base.rpc_ws(default_listen_port)
	}

	fn rpc_methods(&self) -> Result<sc_service::config::RpcMethods> {
		self.base.rpc_methods()
	}

	fn rpc_max_payload(&self) -> Result<Option<usize>> {
		self.base.rpc_max_payload()
	}

	fn rpc_max_request_size(&self) -> Result<Option<usize>> {
		self.base.rpc_max_request_size()
	}

	fn rpc_max_response_size(&self) -> Result<Option<usize>> {
		self.base.rpc_max_response_size()
	}

	fn rpc_max_subscriptions_per_connection(&self) -> Result<Option<usize>> {
		self.base.rpc_max_subscriptions_per_connection()
	}

	fn ws_max_out_buffer_capacity(&self) -> Result<Option<usize>> {
		self.base.ws_max_out_buffer_capacity()
	}

	fn transaction_pool(&self, is_dev: bool) -> Result<TransactionPoolOptions> {
		self.base.transaction_pool(is_dev)
	}

	fn max_runtime_instances(&self) -> Result<Option<usize>> {
		self.base.max_runtime_instances()
	}

	fn runtime_cache_size(&self) -> Result<u8> {
		self.base.runtime_cache_size()
	}

	fn base_path(&self) -> Result<Option<BasePath>> {
		self.base.base_path()
	}

	fn permission_resolver_factory(&self) -> Box<dyn PermissionResolverFactory> {
		if self.remote_authority.is_empty() {
			Box::new(AlwaysPermissionGrantedFactory {})
		} else {
			Box::new(RemoteAuthorityPermissionResolverFactory {
				remote_urls: self.remote_authority.clone(),
				cached: true,
			})
		}
	}
}
