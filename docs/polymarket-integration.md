# Polymarket Integration Notes

This repository targets Polymarket's trading model:

- Market data via public WebSocket channels (market/orderbook/trades).
- Authenticated trading via the CLOB API (order placement + order lifecycle operations).
- Settlement occurs on Polygon (CLOB -> onchain settlement).

## ID Terminology (Canonical in This Repo)

We normalize Polymarket identifiers into shared contracts:

- `condition_id`: the Polymarket condition identifier for a market
- `token_id`: the CLOB token identifier for a specific outcome position

All event, position, and risk messages are expressed using these IDs.

## Strategy-Safe Data

To avoid accidental leakage of inconsistent state into strategies:

- Strategies only read from a strategy-safe "read model" built from normalized market events.
- Execution/risk/portfolio components keep separate internal state used for validation and reconciliation.

