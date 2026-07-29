use spectra::spectra_metric;

spectra_metric! {
    ContinuumOpWallMs {
        store: "photon",
        name: "continuum_op_wall_ms",
        version: "0.1.0",
        description: "Wall time for StoragePort transport operations on the Photon path (ms). Metric name is historical.",
        level: Trace,
        coalesce_ms: 200,
    }
}
