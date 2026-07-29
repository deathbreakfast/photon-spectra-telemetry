use spectra::spectra_metric;

spectra_metric! {
    PhotonPublishErrors {
        store: "photon",
        name: "photon_publish_errors",
        version: "0.1.0",
        description: "Photon publish append failures. Labels: topic, mode.",
        level: Error,
    }
}
