use async_trait::async_trait;
use cardano_serialization_lib::error::JsError;

use crate::model::Action;

#[async_trait]
pub trait IEvaluator: Send {
    async fn evaluate_tx(
        &self,
        tx: &str,
        additional_txs: Vec<String>,
    ) -> Result<Vec<Action>, JsError>;
}