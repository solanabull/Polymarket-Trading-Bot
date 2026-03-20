# Polymarket Trading Bot (Rust, TypeScript, Python) | Low-Latency Prediction Market Bot

A production-grade **Polymarket trading bot** and **prediction market bot** framework for low-latency crypto trading bots, implemented in Rust, TypeScript, and Python.

Built from real Polymarket trading experience on the CLOB, it reflects practical requirements for safe order lifecycle handling, risk checks, and observability.

Polymarket profile (activity): https://polymarket.com/@nobuyoshi005?tab=activity

The project provides canonical schemas and a shared strategy-intent interface that Phase 3+ extends into **Polymarket API** WebSocket ingestion, backtesting, and paper trading.

Telegram: [@solanabull0](https://t.me/solanabull0)

## Status

Phase 1-2 scaffold (architecture + shared schemas + shared strategy interface). Phase 3+ connectivity is next.

![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)
![Languages](https://img.shields.io/badge/languages-Rust%20%7C%20TypeScript%20%7C%20Python-orange.svg)
![Status](https://img.shields.io/badge/status-Phase%201-2%20scaffold-lightgrey.svg)

## Features

- Low-latency **Polymarket trading bot** architecture (event-driven critical path)
- Shared JSON schemas for market events, order intents, fills, positions, and risk decisions
- Strategy interface that emits intents only (execution and risk are isolated modules)
- Deterministic replay and backtest-first design (replay log format + dataset placeholder)
- Paper trading mode foundations (config examples + contract-driven design)
- Multi-language implementations (Rust, TypeScript, Python)
- Safety-first risk controls design (kill switch, stale-data guard, exposure caps)

## Architecture

Market data is normalized into a shared contract, then flows through strategy, risk, and execution layers.

```text
Public Market WebSocket (orderbook/trades)
                 |
                 v
          Orderbook State Manager (L2 + top-of-book metrics)
                 |
                 v
         Strategy Engine (emits OrderIntents only)
                 |
                 v
             Risk Engine (approve/modify/reject)
                 |
                 v
     Execution Engine (Polymarket CLOB API order lifecycle)
                 |
                 v
Portfolio / Positions / PnL (from fills + reconciliation)
```

See also: [`/docs/architecture.md`](/docs/architecture.md), [`/docs/trading-flow.md`](/docs/trading-flow.md), [`/docs/risk-management.md`](/docs/risk-management.md).

## Repository Layout

- `/apps/`
  - `/apps/rust-bot/` (Rust core: contracts + strategy trait scaffolding)
  - `/apps/ts-bot/` (TypeScript control plane: contracts + strategy interface scaffolding)
  - `/apps/py-bot/` (Python: contracts + strategy interface scaffolding for research)
- `/shared/`
  - `/shared/schemas/` (canonical JSON schemas)
  - `/shared/spec/` (shared strategy interface specification)
  - `/shared/sample-configs/` (backtest/paper/live config examples)
- `/docs/` (architecture, trading flow, risk, integration notes, latency budget)

## Quick Start

This is a scaffold, so "run the bot" will be Phase 3+. For Phase 1-2, validate that the shared contracts and interfaces compile/typecheck.

1. Clone the repo
2. Typecheck/build each implementation

### Rust (contracts + strategy trait)

```bash
cd apps/rust-bot
cargo test
```

### TypeScript (contracts + strategy interface)

```bash
cd apps/ts-bot
npm install
npm run typecheck
```

### Python (contracts + strategy interface)

```bash
cd apps/py-bot
python -c "import py_bot; print('py_bot imported')"
```

## Use Cases

- Build a **prediction market bot** for Polymarket research and live trading
- Develop and compare short-horizon trading strategies (momentum, breakout, market making)
- Run deterministic backtests and event replay
- Prototype analytics and microstructure features in Python
- Integrate a safe **Polymarket API** execution layer with strict risk controls

## Roadmap

- Phase 1-2 (already in this repo): scaffold, contracts, shared strategy interface, config examples
- Phase 3+: market data layer (public WebSocket), orderbook state, authenticated trading layer (CLOB), risk engine, paper trading, tests, and full backtesting

## FAQ

### What is Polymarket trading bot?

A Polymarket trading bot is a system that reads prediction market data (typically via WebSockets), then uses a trading strategy to place/cancel/replace orders through Polymarket's CLOB, with fills reflected in portfolio and PnL tracking.

### Is this bot profitable?

This repo does not promise profitability. It provides the production-grade framework you need to implement strategies and evaluate them via backtesting and paper trading.

### Does this repository include Polymarket API integration yet?

Not yet. Phase 3+ will add authenticated order lifecycle and real-time market data ingestion through Polymarket's official WebSocket and CLOB trading flows.

### Which language should I use?

- Rust: performance-oriented core and low-latency components
- TypeScript: orchestration/services and monitoring
- Python: research, analytics, and backtesting

## Keywords

Polymarket trading bot, prediction market bot, crypto trading bot, Polymarket API, CLOB trading bot, low-latency event-driven trading system

