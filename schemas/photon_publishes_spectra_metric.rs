use spectra::spectra_metric;

spectra_metric! {
    PhotonPublishes {
        store: "photon",
        name: "photon_publishes",
        version: "0.1.0",
        description: "Photon transport publishes (append ok). Labels: topic, mode.",
        level: Debug,
    }
}
