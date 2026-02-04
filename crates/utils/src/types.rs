use alloy_primitives::U256;
use alloy_rpc_types::{TransactionTrait, eth::transaction::Transaction};
use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;
use std::pin::Pin;
use tokio_stream::Stream;

pub type CollectorStream<'a, E> = Pin<Box<dyn Stream<Item = E> + Send + 'a>>;

#[async_trait]
pub trait Collector<E>: Send + Sync {
    async fn collect_events(&self) -> Result<CollectorStream<E>>;
}

pub trait Executor<A>: Send + Sync {}

pub trait Orchestrator<E, A>: Send + Sync {}

pub struct Tx {
    pub to: [u8; 20],
    pub from: [u8; 20],
    pub value: U256,
    pub input: Bytes,
}

impl From<Transaction> for Tx {
    fn from(value: Transaction) -> Tx {
        let recovered = value.into_recovered();
        let from = recovered.signer().0.0;
        let (to, input) = match recovered.to() {
            Some(addr) => (addr.0.0, Bytes::from(recovered.input().to_owned())),
            // If there is no to address it is contract creation and we do not want to put all of the contract bytes
            // onto the heap unecessarily
            None => ([0; 20], Bytes::new()),
        };
        Tx {
            to,
            from,
            value: recovered.value(),
            input,
        }
    }
}

pub struct BlockEvent {
    pub num: u64,
    pub hash: [u8; 32],
    pub txs: Vec<Tx>,
}

pub struct LogEvent {}

pub enum Event {
    Block(BlockEvent),
    Log(LogEvent),
}

pub enum Action {}
