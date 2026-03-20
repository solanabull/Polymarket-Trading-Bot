# Risk Management

The risk engine is mandatory and sits between strategies and execution.

## Non-Negotiable Guards

Strategies never call the exchange directly; they can only emit intents.

Risk engine must enforce (at minimum):

- `kill_switch`: if degraded connectivity or stale market data, reject all new intents
- `stale_market_data`: reject intents if the last L2 update is too old
- `max_position_per_token`
- `max_order_size`
- `max_quote_width` / slippage tolerance (deny quotes that exceed tolerances)
- `max_daily_loss`
- circuit breakers (rate-limited failures, repeated rejections, etc.)

## Decision Records

Every intent decision produces a structured record:

- `decision_id`
- `intent_id`
- `status`: `approved` | `modified` | `rejected`
- `reasons`: list of reason codes + human-readable messages
- `effective_intent`: only when modified

These decision records must be persisted so replay/backtesting can reproduce outcomes exactly.

