# Latency Budget

Low-latency in this context means predictable, bounded per-event processing and avoiding blocking operations in the critical path.

## Critical Path (Conceptual)

`market_ws_event` -> `L2 update + top-of-book metrics` -> `strategy signal` -> `risk decision` -> `order route`

## Design Constraints

- Event-driven updates only; avoid batch polling in live mode.
- Use bounded queues between stages.
- Separate CPU-heavy tasks (feature computation, analytics) from the critical path when needed.
- Emit latency metrics for:
  - time in market ingestion stage
  - time in strategy stage
  - time in risk stage
  - time in execution/auth stage

## Replay Mode

Latency numbers in replay mode should be comparable but may reflect recorded timestamps vs wall-clock processing.

