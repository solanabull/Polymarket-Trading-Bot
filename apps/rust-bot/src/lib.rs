//! Phase 1-2 scaffold: shared message contracts + shared strategy interface.
//!
//! This crate does NOT connect to Polymarket yet. It exists to define common types
//! and trait contracts that later market-data/execution/risk layers will use.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IntentAction {
    Place,
    Cancel,
    Replace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Limit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderIntent {
    pub intent_id: String,
    pub strategy_id: String,
    pub timestamp_ms: u64,

    pub intent_type: IntentAction,
    pub token_id: String,

    pub side: OrderSide,
    pub order_type: OrderType,

    /// Normalized share price, when available.
    pub price: Option<f64>,
    /// Exchange-native tick price, when available.
    pub price_ticks: Option<u64>,
    /// Size in shares.
    pub size: f64,

    pub client_order_id: String,

    /// Used for cancel/replace when exchange order id is known.
    pub target_order_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyOutput {
    pub strategy_output_id: String,
    pub timestamp_ms: u64,
    pub strategy_id: String,

    pub reason_codes: Vec<String>,
    pub intents: Vec<OrderIntent>,

    /// Optional debugging metadata that must be safe to persist/log.
    pub debug: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskDecisionStatus {
    Approved,
    Modified,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskDecisionReason {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskDecision {
    pub decision_id: String,
    pub timestamp_ms: u64,

    pub intent_id: String,
    pub status: RiskDecisionStatus,

    pub reasons: Vec<RiskDecisionReason>,

    pub effective_intent: Option<OrderIntent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskLimits {
    pub max_position_per_token: f64,
    pub max_order_size: f64,
    pub max_quote_width_bps: f64,
    pub max_daily_loss: f64,
    pub stale_market_data_ms: u64,
    pub kill_switch: bool,
    pub category_exposure: Vec<CategoryExposure>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryExposure {
    pub category: String,
    pub max_exposure: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FillEvent {
    pub fill_id: String,
    pub timestamp_ms: u64,
    pub condition_id: String,
    pub token_id: String,

    pub order_id: String,
    pub client_order_id: String,

    pub side: OrderSide,
    pub price: Option<f64>,
    pub price_ticks: Option<u64>,
    pub size: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub condition_id: String,
    pub token_id: String,
    pub quantity: f64,
    pub avg_cost: f64,
    pub realized_pnl: f64,
    pub unrealized_pnl: f64,
}

/// Strategy-safe, read-only market view.
/// This struct is defined by the shared strategy spec; later phases will
/// populate it from the orderbook state manager.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketReadModel {
    pub condition_id: String,
    pub token_id: String,
    pub last_update_timestamp_ms: u64,

    /// Top-of-book metrics.
    pub best_bid: Option<f64>,
    pub best_ask: Option<f64>,
    pub spread: Option<f64>,
    pub midpoint: Option<f64>,
    pub imbalance: Option<f64>,
}

pub trait Strategy {
    /// Human-readable strategy name for logs/metrics.
    fn name(&self) -> &str;

    fn on_market_update(&mut self, market: &MarketReadModel) -> StrategyOutput;
    fn on_fill(&mut self, _fill: &FillEvent) {}
}

/// Noop strategy is useful for integration smoke-tests of the shared contracts.
#[derive(Debug, Default)]
pub struct NoopStrategy {
    pub strategy_id: String,
    pub now_ms: u64,
}

impl NoopStrategy {
    pub fn new(strategy_id: impl Into<String>) -> Self {
        Self { strategy_id: strategy_id.into(), now_ms: 0 }
    }
}

impl Strategy for NoopStrategy {
    fn name(&self) -> &str {
        "noop"
    }

    fn on_market_update(&mut self, _market: &MarketReadModel) -> StrategyOutput {
        StrategyOutput {
            strategy_output_id: format!("{}-out-{}", self.strategy_id, self.now_ms),
            timestamp_ms: self.now_ms,
            strategy_id: self.strategy_id.clone(),
            reason_codes: vec!["NOOP".to_string()],
            intents: vec![],
            debug: None,
        }
    }

    fn on_fill(&mut self, _fill: &FillEvent) {}
}

