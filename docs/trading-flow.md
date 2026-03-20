# Trading Flow

This document describes the event-driven flow used in all modes (backtest, paper, live).

## Runtime Inputs

- Public market WebSocket events (orderbook snapshots/deltas, trades, heartbeats)
- User/WebSocket events (order updates, fills, cancels, rejections)
- Internal timer ticks (used for stale-data detection and retry scheduling)
- Strategy configuration (limits, thresholds, mode)

## Core Loop (High-Level)

1. `MarketEvent` arrives
2. Orderbook state manager updates L2 and computes top-of-book metrics
3. Normalized book update is appended to the replay log (in non-production this may be local)
4. Strategy reads the latest strategy-safe view and produces `StrategyOutput`
5. Risk engine evaluates `StrategyOutput.intents`:
   - approve
   - modify (e.g., clamp size/price bounds)
   - reject (with explicit reason)
6. Execution engine converts approved intents into CLOB operations:
   - place/cancel/replace order
   - track order lifecycle and persist outcomes
7. Portfolio tracker updates inventory and PnL from fills and reconciled order state
8. Observability emits logs/metrics/traces with per-stage latency

## Replay/Deduplication

- Every event and decision record must be idempotent and replayable.
- Order submission uses an idempotency key (generated from intent_id + side + token_id, etc.)

