# Shared Schemas

This folder contains canonical, language-agnostic JSON Schema definitions used by all implementations.

Canonical message contracts (single source of truth):

- `messages.schema.json`
  - market events (orderbook snapshots/deltas/trades/heartbeats)
  - order intents (strategy outputs)
  - fills
  - positions
  - risk limits
  - risk decisions and strategy outputs

## Design Notes

- IDs are explicit (`condition_id`, `token_id`, `intent_id`, `decision_id`, etc.)
- Timestamps are explicit and use `timestamp_ms` (milliseconds since epoch)
- Price/quantity representation is contractually defined as:
  - `price` (float, normalized share price, if available)
  - `price_ticks` (int, exchange-native ticks, if available)
  - `size` (float, shares)

Later phases may add stricter typing (e.g., fixed-point price wrappers) and code generation.

