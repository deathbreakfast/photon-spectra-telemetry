use spectra::spectra_metric;

spectra_metric! {
    PhotonHandlerFailures {
        store: "photon",
        name: "photon_handler_failures",
        version: "0.1.0",
        description: "Photon handler or Valence build failures. Labels: topic, reason.",
        level: Error,
    }
}
