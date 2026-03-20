// Phase 1-2 scaffold: shared contracts + shared strategy interface.
//
// This file contains TypeScript equivalents of the shared message contracts
// and the strategy interface. It does not connect to Polymarket yet.

export type OrderSide = "buy" | "sell";
export type IntentAction = "place" | "cancel" | "replace";
export type OrderType = "limit";

export interface OrderIntent {
  intent_id: string;
  strategy_id: string;
  timestamp_ms: number;

  intent_type: IntentAction;
  token_id: string;

  side: OrderSide;
  order_type: OrderType;

  price?: number; // normalized share price, when available
  price_ticks?: number; // exchange-native tick price, when available
  size: number; // shares

  client_order_id: string;
  target_order_id?: string;
}

export interface StrategyOutput {
  strategy_output_id: string;
  timestamp_ms: number;
  strategy_id: string;

  reason_codes: string[];
  intents: OrderIntent[];
  debug?: Record<string, unknown>;
}

export interface MarketReadModel {
  condition_id: string;
  token_id: string;
  last_update_timestamp_ms: number;

  best_bid?: number;
  best_ask?: number;
  spread?: number;
  midpoint?: number;
  imbalance?: number;
}

export interface FillEvent {
  fill_id: string;
  timestamp_ms: number;
  condition_id: string;
  token_id: string;

  order_id: string;
  client_order_id: string;

  side: OrderSide;
  price?: number;
  price_ticks?: number;
  size: number;
}

export interface Strategy {
  name(): string;
  onMarketUpdate(market: MarketReadModel): StrategyOutput;
  onFill?(fill: FillEvent): void;
}

export class NoopStrategy implements Strategy {
  constructor(private readonly strategyId: string) {}
  name(): string {
    return "noop";
  }
  onMarketUpdate(_market: MarketReadModel): StrategyOutput {
    return {
      strategy_output_id: `${this.strategyId}-out-0`,
      timestamp_ms: 0,
      strategy_id: this.strategyId,
      reason_codes: ["NOOP"],
      intents: []
    };
  }
}

