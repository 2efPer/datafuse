// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use common_exception::Result;

pub type Elapsed = Duration;

#[async_trait::async_trait]
pub trait AbortableService<Args, R> {
    fn abort(&self, force: bool) -> Result<()>;

    async fn start(&self, args: Args) -> Result<R>;

    async fn wait_terminal(&self, duration: Option<Duration>) -> Result<Elapsed>;
}

pub type AbortableServer = Arc<dyn AbortableService<(String, u16), SocketAddr> + Send + Sync>;
