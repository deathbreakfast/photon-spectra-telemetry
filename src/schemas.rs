//! Photon Spectra schema modules (inventory + typed helpers + topics).
//!
//! Each module wraps one `spectra_schema!` / `spectra_metric!` invocation under
//! `schemas/` at the repo root (relative to this file, one directory up from `src/`); the
//! macro generates the row/payload types, the typed logger/recorder, the Photon topic
//! constant, and the `inventory` registration for that table or counter/gauge. This module
//! itself is private — see [`crate::helpers`] and [`crate::topics`] for the re-exported,
//! effectively-public names.
#![allow(clippy::too_many_arguments)]

/// `photon_publishes` counter schema (see `schemas/photon_publishes_spectra_metric.rs`).
#[path = "../schemas/photon_publishes_spectra_metric.rs"]
pub mod photon_publishes;

/// `photon_drains` counter schema (see `schemas/photon_drains_spectra_metric.rs`).
#[path = "../schemas/photon_drains_spectra_metric.rs"]
pub mod photon_drains;

/// `photon_backlog` gauge schema (see `schemas/photon_backlog_spectra_metric.rs`).
#[path = "../schemas/photon_backlog_spectra_metric.rs"]
pub mod photon_backlog;

/// `photon_publish_errors` counter schema (see `schemas/photon_publish_errors_spectra_metric.rs`).
#[path = "../schemas/photon_publish_errors_spectra_metric.rs"]
pub mod photon_publish_errors;

/// `photon_handler_failures` counter schema (see
/// `schemas/photon_handler_failures_spectra_metric.rs`).
#[path = "../schemas/photon_handler_failures_spectra_metric.rs"]
pub mod photon_handler_failures;

/// `photon_ops_log` event schema (see `schemas/photon_ops_log_spectra_event.rs`).
#[path = "../schemas/photon_ops_log_spectra_event.rs"]
pub mod photon_ops_log;

/// `photon_dlq` event schema (see `schemas/photon_dlq_spectra_event.rs`).
#[path = "../schemas/photon_dlq_spectra_event.rs"]
pub mod photon_dlq;

/// Legacy-named `continuum_append_wall_ms` `StoragePort` wall-clock gauge (see
/// `schemas/continuum_append_wall_ms_spectra_metric.rs`).
#[path = "../schemas/continuum_append_wall_ms_spectra_metric.rs"]
pub mod continuum_append_wall_ms;

/// Legacy-named `continuum_op_wall_ms` `StoragePort` wall-clock gauge (see
/// `schemas/continuum_op_wall_ms_spectra_metric.rs`).
#[path = "../schemas/continuum_op_wall_ms_spectra_metric.rs"]
pub mod continuum_op_wall_ms;
