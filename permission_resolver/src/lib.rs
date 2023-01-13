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
use log::{debug, error};
use reqwest::{Client, ClientBuilder};
use sp_authority_permission::{PermissionResolver, PermissionResolverFactory};
use sp_consensus_slots::Slot;

pub struct RemoteAuthorityPermissionResolver {
	client: Client,
	base_url: String,
}

impl RemoteAuthorityPermissionResolver {
	pub fn new(base_url: &str) -> RemoteAuthorityPermissionResolver {
		let client = ClientBuilder::new().build().expect("Could not create client");
		RemoteAuthorityPermissionResolver { client, base_url: base_url.to_owned() }
	}

	async fn do_resolve_slot(&self, slot: Slot) -> Result<bool, String> {
		debug!(target: "permission-resolver", "Checking slot {} permission...", slot);
		let url = format!("{}/authorize/slot/{}", self.base_url, slot);
		let resp = self
			.client
			.put(url)
			.send()
			.await
			.map_err(|_| "Could not reach out to remote service")?;
		let can: bool = resp
			.text()
			.await
			.expect("Failed to parse response")
			.parse()
			.map_err(|_| "Could not parse response")?;
		debug!(target: "permission-resolver", "Got slot {} permission: {}", slot, can);
		Ok(can)
	}

	async fn do_resolve_round(&self, round: u64) -> Result<bool, String> {
		debug!(target: "permission-resolver", "Checking round  {} permission...", round);
		let url = format!("{}/authorize/round/{}", self.base_url, round);
		let resp = self
			.client
			.put(url)
			.send()
			.await
			.map_err(|_| "Could not reach out to remote service")?;
		let can: bool = resp
			.text()
			.await
			.expect("Failed to parse response")
			.parse()
			.map_err(|_| "Could not parse response")?;
		debug!(target: "permission-resolver", "Got round {} permission: {}", round, can);
		Ok(can)
	}

	async fn do_resolve_session(&self, session_index: u32) -> Result<bool, String> {
		let url = format!("{}/authorize/session/{}", self.base_url, session_index);
		let resp = self
			.client
			.put(url)
			.send()
			.await
			.map_err(|_| "Could not reach out to remote service")?;
		let can: bool = resp
			.text()
			.await
			.expect("Failed to parse response")
			.parse()
			.map_err(|_| "Could not parse response")?;
		Ok(can)
	}
}

#[async_trait]
impl PermissionResolver for RemoteAuthorityPermissionResolver {
	async fn resolve_slot(&self, slot: Slot) -> bool {
		match self.do_resolve_slot(slot).await {
			Ok(result) => result,
			Err(e) => {
				error!(
					target: "permission-resolver",
					"Could not resolve permission, reason: {}", e);
				false
			},
		}
	}

	async fn resolve_round(&self, round: u64) -> bool {
		match self.do_resolve_round(round).await {
			Ok(result) => result,
			Err(e) => {
				error!(
					target: "permission-resolver",
					"Could not resolve permission, reason: {}", e);
				false
			},
		}
	}

	async fn resolve_session(&self, session_index: u32) -> bool {
		match self.do_resolve_session(session_index).await {
			Ok(result) => result,
			Err(e) => {
				error!(
                    target: "permission-resolver",
                    "Could not resolve permission, reason: {}", e);
				false
			},
		}
	}
}

pub struct RemoteAuthorityPermissionResolverFactory {
	pub remote_url: String,
}

#[async_trait]
impl PermissionResolverFactory for RemoteAuthorityPermissionResolverFactory {
	async fn create(&self) -> Box<dyn PermissionResolver> {
		let resolver = RemoteAuthorityPermissionResolver::new(&self.remote_url);
		Box::new(resolver)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use httpmock::MockServer;
	use sp_authority_permission::PermissionResolver;

	#[tokio::test]
	async fn test_remote_permits_slot() {
		let server = MockServer::start();
		let mock = server.mock(|when, then| {
			when.path("/authorize/slot/1");
			then.status(200).body("true");
		});
		let permission_resolver = RemoteAuthorityPermissionResolver::new(&server.base_url());
		let permission = permission_resolver.resolve_slot(1.into()).await;

		mock.assert();
		assert!(permission)
	}

	#[tokio::test]
	async fn test_remote_denies_slot() {
		let server = MockServer::start();
		let mock = server.mock(|when, then| {
			when.path("/authorize/slot/1");
			then.status(200).body("false");
		});
		let permission_resolver = RemoteAuthorityPermissionResolver::new(&server.base_url());
		let permission = permission_resolver.resolve_slot(1.into()).await;

		mock.assert();
		assert!(!permission)
	}

	#[tokio::test]
	async fn test_remote_permits_round() {
		let server = MockServer::start();
		let mock = server.mock(|when, then| {
			when.path("/authorize/round/1");
			then.status(200).body("true");
		});
		let permission_resolver = RemoteAuthorityPermissionResolver::new(&server.base_url());
		let permission = permission_resolver.resolve_round(1).await;

		mock.assert();
		assert!(permission)
	}

	#[tokio::test]
	async fn test_remote_denies_round() {
		let server = MockServer::start();
		let mock = server.mock(|when, then| {
			when.path("/authorize/round/1");
			then.status(200).body("false");
		});
		let permission_resolver = RemoteAuthorityPermissionResolver::new(&server.base_url());
		let permission = permission_resolver.resolve_round(1).await;

		mock.assert();
		assert!(!permission)
	}

	#[tokio::test]
	async fn test_remote_permits_session() {
		let server = MockServer::start();
		let mock = server.mock(|when, then| {
			when.path("/authorize/session/1");
			then.status(200).body("true");
		});
		let permission_resolver = RemoteAuthorityPermissionResolver::new(&server.base_url());
		let permission = permission_resolver.resolve_session(1).await;

		mock.assert();
		assert!(permission)
	}

	#[tokio::test]
	async fn test_remote_denies_session() {
		let server = MockServer::start();
		let mock = server.mock(|when, then| {
			when.path("/authorize/session/1");
			then.status(200).body("false");
		});
		let permission_resolver = RemoteAuthorityPermissionResolver::new(&server.base_url());
		let permission = permission_resolver.resolve_session(1).await;

		mock.assert();
		assert!(!permission)
	}

	#[tokio::test]
	async fn test_permission_denied_in_case_of_integration_error() {
		let permission_resolver = RemoteAuthorityPermissionResolver::new("localhost");
		let permission = permission_resolver.resolve_slot(1.into()).await;
		assert!(!permission)
	}
}
