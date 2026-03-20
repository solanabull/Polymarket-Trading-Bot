use rust_bot::*;

#[test]
fn order_intent_roundtrip() {
    let intent = OrderIntent {
        intent_id: "intent-1".to_string(),
        strategy_id: "strat-1".to_string(),
        timestamp_ms: 1_700_000_000_000,
        intent_type: IntentAction::Place,
        token_id: "token-1".to_string(),
        side: OrderSide::Buy,
        order_type: OrderType::Limit,
        price: Some(0.42),
        price_ticks: None,
        size: 10.0,
        client_order_id: "coid-1".to_string(),
        target_order_id: None,
    };

    let json = serde_json::to_string(&intent).expect("serialize");
    let decoded: OrderIntent = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(decoded.intent_id, intent.intent_id);
    assert!((decoded.size - intent.size).abs() < f64::EPSILON);
}

