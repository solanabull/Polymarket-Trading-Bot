# Shared Strategy Interface

Strategies are required to be **pure intent generators**:

- They may read market state (via a strategy-safe read model).
- They must emit `StrategyOutput` objects containing `OrderIntent` entries only.
- They must not place/cancel/replace orders directly.
- Risk engine is the only component allowed to approve/modify/reject intents.

## Required Responsibilities

1. **Initialize**
   - Receive `strategy_id`
   - Receive immutable configuration (strategy parameters, risk-aware thresholds that do not replace the global risk engine)

2. **On Market Updates**
   - Called whenever the strategy-safe view changes (L2 snapshot/delta or derived top-of-book metrics)
   - Strategy computes new intents (possibly empty)

3. **On Fills**
   - Called with fill events so the strategy can update internal models that are useful for forecasting/feature engineering
   - The strategy must remain deterministic given the sequence of events it receives

4. **On Mode Changes**
   - Backtest/paper/live differences are handled by infrastructure.
   - Strategy only needs to know mode for behavior toggles that affect intent generation, not for exchange calls.

## Common Data Types (Conceptual)

- `MarketReadModel`
  - last update timestamp
  - top-of-book metrics (best bid/ask, spread, midpoint)
  - imbalance and liquidity depth
  - computed per-token state for the markets it trades

- `StrategyOutput`
  - contains `intents: OrderIntent[]`
  - includes `reason_codes` for observability and risk debugging

## API Shape (Mapped to Each Language)

All languages must implement a strategy with the following operations:

- `on_market_update(market: MarketReadModel) -> StrategyOutput`
- `on_fill(fill: FillEvent) -> None`
- `name() -> str`

Implementations must keep strategy state inside the strategy instance, and all emitted intents must reference:

- `strategy_id`
- unique `intent_id` values
- correct `token_id` and side for order intent direction

## Strategy Examples (To Be Implemented in Later Phases)

This repo will include at least:

- microstructure market making strategy
- short-horizon momentum strategy
- event-driven breakout strategy

