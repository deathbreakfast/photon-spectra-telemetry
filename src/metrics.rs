//! Filtered remapping from [`photon_telemetry::OpsLog`] onto typed Spectra schemas.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use serde_json::{Map, Value};
use spectra_core::{try_log_event, try_record_counter, try_record_gauge};

use crate::sanitize::sanitize_error_message;

const HIGH_CARDINALITY_LABELS: &[&str] = &["topic", "subscription", "topic_key"];
const DLQ_SENSITIVE_FIELDS: &[&str] = &["event_id", "topic_key"];
const EVENT_ERROR_FIELDS: &[&str] = &["error"];

const fn counter_delta(value: f64) -> i64 {
    #[allow(clippy::cast_possible_truncation)]
    let delta = value.trunc() as i64;
    delta
}

fn hash_label_value(value: &str) -> String {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    format!("h{:016x}", hasher.finish())
}

fn sanitize_label_value(key: &str, value: &str) -> String {
    if HIGH_CARDINALITY_LABELS.contains(&key) {
        hash_label_value(value)
    } else {
        value.to_owned()
    }
}

fn metric_label_keys(name: &str) -> Option<&'static [&'static str]> {
    match name {
        "photon_publishes" | "photon_publish_errors" => Some(&["topic", "mode"]),
        "photon_drains" => Some(&["topic", "subscription"]),
        "photon_backlog" => Some(&["topic"]),
        "photon_handler_failures" => Some(&["topic", "reason"]),
        "continuum_append_wall_ms" | "continuum_op_wall_ms" => Some(&[]),
        _ => None,
    }
}

fn filter_metric_labels(name: &str, labels: &[(&str, &str)]) -> Option<Vec<(String, String)>> {
    let allowed = metric_label_keys(name)?;
    let mut out = Vec::new();
    for (key, value) in labels {
        if allowed.contains(key) {
            out.push(((*key).to_owned(), sanitize_label_value(key, value)));
        }
    }
    Some(out)
}

fn event_field_keys(name: &str) -> Option<&'static [&'static str]> {
    match name {
        "photon_ops_log" => Some(&[
            "component",
            "operation",
            "message",
            "topic",
            "subscription",
            "error",
        ]),
        "photon_dlq" => Some(&[
            "event_id",
            "topic",
            "topic_key",
            "seq",
            "subscription",
            "reason",
            "error",
        ]),
        _ => None,
    }
}

fn sanitize_event_field(name: &str, key: &str, value: &Value) -> Value {
    match value {
        Value::String(text) if EVENT_ERROR_FIELDS.contains(&key) => {
            Value::String(sanitize_error_message(text))
        }
        Value::String(text)
            if HIGH_CARDINALITY_LABELS.contains(&key)
                || (name == "photon_dlq" && DLQ_SENSITIVE_FIELDS.contains(&key)) =>
        {
            Value::String(hash_label_value(text))
        }
        other => other.clone(),
    }
}

fn sanitize_event_payload(name: &str, payload: &Value) -> Option<Value> {
    let allowed = event_field_keys(name)?;
    let object = payload.as_object()?;
    let mut filtered = Map::new();
    for key in allowed {
        if let Some(value) = object.get(*key) {
            filtered.insert((*key).to_owned(), sanitize_event_field(name, key, value));
        }
    }
    Some(Value::Object(filtered))
}

/// Handle a [`photon_telemetry::OpsLog::record_counter`] call with allowlisted labels.
pub fn record_counter(name: &str, labels: &[(&str, &str)], value: f64) {
    let Some(filtered) = filter_metric_labels(name, labels) else {
        return;
    };
    let refs: Vec<(&str, &str)> = filtered
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    try_record_counter(name, &refs, counter_delta(value));
}

/// Handle a [`photon_telemetry::OpsLog::record_gauge`] call with allowlisted labels.
pub fn record_gauge(name: &str, labels: &[(&str, &str)], value: f64) {
    let Some(filtered) = filter_metric_labels(name, labels) else {
        return;
    };
    let refs: Vec<(&str, &str)> = filtered
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    try_record_gauge(name, &refs, value);
}

/// Handle a [`photon_telemetry::OpsLog::log_event`] call with schema-filtered payloads.
pub fn log_event(name: &str, payload: &Value) {
    let Some(filtered) = sanitize_event_payload(name, payload) else {
        return;
    };
    try_log_event(name, &filtered);
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn counter_delta_integer_values_happy() {
        assert_eq!(counter_delta(0.0), 0);
        assert_eq!(counter_delta(1.0), 1);
        assert_eq!(counter_delta(42.0), 42);
        assert_eq!(counter_delta(-3.0), -3);
    }

    #[test]
    fn hash_label_value_is_stable_and_bounded() {
        let a = hash_label_value("orders.created");
        let b = hash_label_value("orders.created");
        let c = hash_label_value("other.topic");
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert!(a.starts_with('h'));
        assert_eq!(a.len(), 17);
    }

    #[test]
    fn filter_metric_labels_hashes_topic_sad() {
        let filtered = filter_metric_labels(
            "photon_publishes",
            &[("topic", "demo"), ("mode", "durable")],
        )
        .expect("known metric");
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].0, "topic");
        assert_ne!(filtered[0].1, "demo");
        assert_eq!(filtered[1].0, "mode");
        assert_eq!(filtered[1].1, "durable");
    }

    #[test]
    fn unknown_metric_name_returns_none_sad() {
        assert!(filter_metric_labels("unknown_photon_metric", &[]).is_none());
    }

    #[test]
    fn sanitize_event_payload_strips_unknown_fields_sad() {
        let payload = json!({
            "component": "runtime",
            "message": "ready",
            "secret": "drop-me",
            "topic": "demo.topic"
        });
        let filtered = sanitize_event_payload("photon_ops_log", &payload).expect("known event");
        assert_eq!(filtered["component"], "runtime");
        assert_eq!(filtered["message"], "ready");
        assert!(filtered.get("secret").is_none());
        assert_ne!(filtered["topic"], "demo.topic");
    }

    #[test]
    fn sanitize_event_payload_redacts_ops_error_and_dlq_ids_sad() {
        let ops = json!({
            "component": "handler",
            "error": "decode failed password=hunter2 body={\"ssn\":\"123\"}"
        });
        let filtered = sanitize_event_payload("photon_ops_log", &ops).expect("known event");
        let err = filtered["error"].as_str().unwrap_or("");
        assert!(err.contains("[redacted]"));
        assert!(!err.contains("hunter2"));

        let dlq = json!({
            "event_id": "tenant-acme-user-42",
            "topic_key": "partition/user-99",
            "topic": "orders.created",
            "error": "handler failed"
        });
        let filtered = sanitize_event_payload("photon_dlq", &dlq).expect("known event");
        assert_ne!(filtered["event_id"], "tenant-acme-user-42");
        assert_ne!(filtered["topic_key"], "partition/user-99");
        assert_ne!(filtered["topic"], "orders.created");
    }
}
