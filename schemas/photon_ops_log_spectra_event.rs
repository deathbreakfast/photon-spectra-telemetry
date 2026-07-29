use spectra::spectra_schema;

spectra_schema! {
    PhotonOpsLog {
        store: "photon",
        table: "photon_ops_log",
        version: "0.1.0",
        description: "Photon operational trace (boot, transport, WebSocket lifecycle).",
        level: Debug,
        fields: [
            component: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            operation: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            message: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            topic: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            subscription: {
                r#type: String,
                classification: { pii: false, safe_for_console: false },
            },
            error: {
                r#type: String,
                classification: { pii: false, safe_for_console: false },
            },
        ],
    }
}
