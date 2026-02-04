use alloy_provider::Provider;
use alloy_rpc_types::eth::Filter;
use anyhow::Result;
use async_trait::async_trait;
use tokio_stream::StreamExt;
use utils::types::{Collector, CollectorStream, Event, LogEvent};

pub struct LogCollector<P> {
    provider: P,
    filter: Filter,
}

#[async_trait]
impl<P: Provider> Collector<Event> for LogCollector<P> {
    async fn collect_events(&self) -> Result<CollectorStream<Event>> {
        let sub = self.provider.subscribe_logs(&self.filter).await?;
        let stream = sub.into_stream();
        let stream = stream.map(|_log| Event::Log(LogEvent {}));
        Ok(Box::pin(stream))
    }
}

impl<P: Provider> LogCollector<P> {
    pub fn new(provider: P, filter: Option<Filter>) -> Self {
        LogCollector {
            provider,
            filter: filter.unwrap_or_default(),
        }
    }
}
