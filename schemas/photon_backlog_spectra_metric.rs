use spectra::spectra_metric;

spectra_metric! {
    PhotonBacklog {
        store: "photon",
        name: "photon_backlog",
        version: "0.1.0",
        description: "Photon publish minus drain gap (per topic). Labels: topic.",
        level: Trace,
        coalesce_ms: 200,
    }
}
