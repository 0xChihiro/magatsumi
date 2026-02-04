use alloy_provider::Provider;
use anyhow::Result;
use async_trait::async_trait;
use rayon::{
    ThreadPool, ThreadPoolBuilder,
    iter::{IntoParallelIterator, ParallelIterator},
};
use tokio_stream::StreamExt;
use utils::types::{BlockEvent, Collector, CollectorStream, Event, Tx};

pub struct BlockCollector<P> {
    provider: P,
    thread_pool: ThreadPool,
    // Add channel here or do it directly in the engine?
}

#[async_trait]
impl<P: Provider> Collector<Event> for BlockCollector<P> {
    async fn collect_events(&self) -> Result<CollectorStream<Event>> {
        let sub = self.provider.subscribe_full_blocks().full();
        let stream = sub.into_stream().await?;
        let stream = stream.filter_map(|block| match block {
            Ok(b) => {
                let num = b.number();
                let hash = b.hash().0;
                let txs = b.transactions.as_transactions();
                match txs {
                    Some(inner) => self.thread_pool.install(|| {
                        let txs: Vec<Tx> = inner
                            .into_par_iter()
                            .map(|tx| Tx::from(tx.to_owned()))
                            .collect();
                        Some(Event::Block(BlockEvent { num, hash, txs }))
                    }),
                    None => None,
                }
            }
            Err(_) => None,
        });
        Ok(Box::pin(stream))
    }
}

impl<P: Provider> BlockCollector<P> {
    pub fn new(provider: P, num_threads: usize) -> Self {
        BlockCollector {
            provider,
            thread_pool: ThreadPoolBuilder::new()
                .num_threads(num_threads)
                .build()
                .unwrap(),
        }
    }
}
