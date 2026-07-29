use spectra::spectra_schema;

spectra_schema! {
    PhotonDlqLog {
        store: "photon",
        table: "photon_dlq",
        version: "0.1.0",
        description: "Photon dead-letter metadata (no event payloads).",
        level: Error,
        fields: [
            event_id: {
                r#type: String,
                classification: { pii: false, safe_for_console: false },
            },
            topic: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            topic_key: {
                r#type: String,
                classification: { pii: false, safe_for_console: false },
            },
            seq: {
                r#type: i64,
                classification: { pii: false, safe_for_console: true },
            },
            subscription: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            reason: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            error: {
                r#type: String,
                classification: { pii: false, safe_for_console: false },
            },
        ],
    }
}
