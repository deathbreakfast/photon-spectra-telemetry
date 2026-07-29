use spectra::spectra_metric;

spectra_metric! {
    ContinuumAppendWallMs {
        store: "photon",
        name: "continuum_append_wall_ms",
        version: "0.1.0",
        description: "Wall time for StoragePort append on the Photon publish path (ms). Metric name is historical.",
        level: Trace,
        coalesce_ms: 200,
    }
}
