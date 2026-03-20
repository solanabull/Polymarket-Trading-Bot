"""
Phase 1-2 scaffold: shared message contracts + shared strategy interface.

This package defines language-idiomatic equivalents of the canonical shared
JSON schemas under `/shared/schemas/messages.schema.json`.

No Polymarket connectivity is implemented in this phase.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Optional, Protocol


OrderSide = str  # "buy" | "sell" (kept as str for simplicity / portability)
IntentAction = str  # "place" | "cancel" | "replace"
OrderType = str  # "limit"


@dataclass(frozen=True)
class OrderIntent:
    intent_id: str
    strategy_id: str
    timestamp_ms: int

    intent_type: IntentAction
    token_id: str

    side: OrderSide
    order_type: OrderType

    price: Optional[float]
    price_ticks: Optional[int]
    size: float

    client_order_id: str
    target_order_id: Optional[str]


@dataclass(frozen=True)
class StrategyOutput:
    strategy_output_id: str
    timestamp_ms: int
    strategy_id: str

    reason_codes: list[str]
    intents: list[OrderIntent]
    debug: Optional[dict[str, Any]] = None


@dataclass(frozen=True)
class MarketReadModel:
    condition_id: str
    token_id: str
    last_update_timestamp_ms: int

    best_bid: Optional[float]
    best_ask: Optional[float]
    spread: Optional[float]
    midpoint: Optional[float]
    imbalance: Optional[float]


@dataclass(frozen=True)
class FillEvent:
    fill_id: str
    timestamp_ms: int
    condition_id: str
    token_id: str

    order_id: str
    client_order_id: str

    side: OrderSide
    price: Optional[float]
    price_ticks: Optional[int]
    size: float


class Strategy(Protocol):
    def name(self) -> str: ...

    def on_market_update(self, market: MarketReadModel) -> StrategyOutput: ...

    def on_fill(self, fill: FillEvent) -> None: ...


@dataclass
class NoopStrategy:
    strategy_id: str

    def name(self) -> str:
        return "noop"

    def on_market_update(self, market: MarketReadModel) -> StrategyOutput:
        _ = market
        return StrategyOutput(
            strategy_output_id=f"{self.strategy_id}-out-0",
            timestamp_ms=0,
            strategy_id=self.strategy_id,
            reason_codes=["NOOP"],
            intents=[],
            debug=None,
        )

    def on_fill(self, fill: FillEvent) -> None:
        _ = fill
        return None

