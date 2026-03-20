# Architecture

## Goals

- Low-latency, event-driven trading on Polymarket's CLOB + WebSocket market/user streams.
- Correctness and safety first: deterministic replay, strict risk controls, and circuit breakers.
- Separate **strategy** (intents) from **execution** (orders) and from **risk** (approval/modification/rejection).
- Monorepo with one implementation each in:
  - Rust (low-latency core)
  - TypeScript (control plane/orchestration + observability)
  - Python (research/backtesting/experiments)

## Conceptual Pipeline

1. **Market Data Layer**
   - Subscribe to public market WebSocket(s)
   - Maintain in-memory L2 orderbook state per asset
   - Normalize incoming snapshots/deltas into a shared event format
   - Persist raw events + normalized book updates for replay

2. **Strategy Layer**
- Read-only access to a "strategy-safe" view of market state
   - Emit **order intents** only (no direct exchange calls)

3. **Risk Layer**
   - Validate and bound every intent (position, slippage, stale data, kill switch, etc.)
   - Allow / modify / reject intents with an explicit decision record

4. **Execution/Auth Layer**
   - Convert approved intents into CLOB order operations (place/cancel/replace/status)
   - Enforce idempotency and safe retry rules
   - Persist all order lifecycle events

5. **Portfolio/Positions/PnL**
   - Update portfolio from fills, reconciled with exchange state
   - Persist realized/unrealized PnL and inventory

6. **Observability**
   - Structured logs
   - Metrics and latency measurement along the critical path
   - Health endpoints and trace spans for market->signal->order routing

## Shared Contracts

All implementations share a canonical set of JSON schemas under:

- `/shared/schemas/`

and a shared strategy interface spec under:

- `/shared/spec/strategy_interface.md`

