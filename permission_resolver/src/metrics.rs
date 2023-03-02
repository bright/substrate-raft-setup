// MIT License

// Copyright (c) 2023 Bright Inventions

// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:

// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
use async_trait::async_trait;
use prometheus_endpoint::{register, Counter, U64};
use sp_authority_permission::PermissionResolver;
use sp_consensus_slots::Slot;
use std::sync::Mutex;

struct DoubleCounter {
	current: Counter<U64>,
	base: Counter<U64>,
}

impl DoubleCounter {
	fn new(
		registry: &prometheus_endpoint::Registry,
		text: &str,
		description: &str,
	) -> std::result::Result<Self, Error> {
		Ok(Self {
			current: register(Counter::new(text, description)?, registry)?,
			base: register(Counter::new(text.to_string() + "_base", "Base value.")?, registry)?,
		})
	}
}

struct Metrics {
	round: Mutex<DoubleCounter>,
	slot: Mutex<DoubleCounter>,
	session: Mutex<DoubleCounter>,
}

impl Metrics {
	pub fn new(registry: &prometheus_endpoint::Registry) -> Result<Self, Error> {
		Ok(Self {
			round: Mutex::new(DoubleCounter::new(registry, "substrate_authority_permission_round", "Number of times authority permission granted the validator to participate in voting.")?),
			slot: Mutex::new(DoubleCounter::new(registry, "substrate_authority_permission_slot", "Number of times authority permission granted the validator to participate in block creation.")?),
			session: Mutex::new(DoubleCounter::new(registry, "substrate_authority_permission_session", "Number of times authority permission granted the validator to send \"I'm online\" message.")?),
		})
	}

	fn inc(metrics: &Mutex<DoubleCounter>, has_permission: bool) {
		if let Ok(m) = metrics.lock() {
			m.base.inc();
			if has_permission {
				m.current.inc();
			}
		}
	}
}

pub struct PermissionResolverMetrics {
	resolver: Box<dyn PermissionResolver>,
	metrics: Metrics,
}

//todo: use metrics
impl PermissionResolverMetrics {
	pub fn new(
		resolver: Box<dyn PermissionResolver>,
		registry: &prometheus_endpoint::Registry,
	) -> Result<Self, Error> {
		Metrics::new(&registry).map(|metrics| Self { resolver, metrics })
	}
}

#[async_trait]
impl PermissionResolver for PermissionResolverMetrics {
	async fn resolve_slot(&self, slot: Slot) -> bool {
		let permission = self.resolver.resolve_slot(slot).await;
		Metrics::inc(&self.metrics.slot, permission);
		return permission
	}

	async fn resolve_round(&self, round: u64) -> bool {
		let permission = self.resolver.resolve_round(round).await;
		Metrics::inc(&self.metrics.round, permission);
		permission
	}

	async fn resolve_session(&self, session_index: u32) -> bool {
		let permission = self.resolver.resolve_session(session_index).await;
		Metrics::inc(&self.metrics.session, permission);
		permission
	}
}

/// Error type for the authority discovery module.
#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Failed to register Prometheus metric.")]
	Prometheus(#[from] prometheus_endpoint::PrometheusError),
}
