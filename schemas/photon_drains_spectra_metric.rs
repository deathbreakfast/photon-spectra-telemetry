use spectra::spectra_metric;

spectra_metric! {
    PhotonDrains {
        store: "photon",
        name: "photon_drains",
        version: "0.1.0",
        description: "Durable handler processed and checkpoint committed. Labels: topic, subscription.",
        level: Debug,
    }
}
