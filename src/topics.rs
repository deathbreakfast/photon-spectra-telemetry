//! Transport `*Payload` / `*_TOPIC` DTOs from Photon Spectra schemas.
//!
//! Each `*_TOPIC` constant is the Photon topic name a Spectra sink publishes to, and the
//! matching `*Payload` is the serialized wire type carried on that topic.
//!
//! # Examples
//!
//! ```rust,no_run
//! use photon_spectra_telemetry::topics::{PhotonPublishesPayload, PHOTON_PUBLISHES_TOPIC};
//!
//! assert_eq!(PhotonPublishesPayload::topic(), PHOTON_PUBLISHES_TOPIC);
//! ```

/// Payload and topic constant for `continuum_append_wall_ms`.
pub use crate::schemas::continuum_append_wall_ms::{
    ContinuumAppendWallMsPayload, CONTINUUM_APPEND_WALL_MS_TOPIC,
};
/// Payload and topic constant for `continuum_op_wall_ms`.
pub use crate::schemas::continuum_op_wall_ms::{
    ContinuumOpWallMsPayload, CONTINUUM_OP_WALL_MS_TOPIC,
};
/// Payload and topic constant for `photon_backlog`.
pub use crate::schemas::photon_backlog::{PhotonBacklogPayload, PHOTON_BACKLOG_TOPIC};
/// Payload and topic constant for `photon_dlq`.
pub use crate::schemas::photon_dlq::{PhotonDlqLogPayload, PHOTON_DLQ_LOG_TOPIC};
/// Payload and topic constant for `photon_drains`.
pub use crate::schemas::photon_drains::{PhotonDrainsPayload, PHOTON_DRAINS_TOPIC};
/// Payload and topic constant for `photon_handler_failures`.
pub use crate::schemas::photon_handler_failures::{
    PhotonHandlerFailuresPayload, PHOTON_HANDLER_FAILURES_TOPIC,
};
/// Payload and topic constant for `photon_ops_log`.
pub use crate::schemas::photon_ops_log::{PhotonOpsLogPayload, PHOTON_OPS_LOG_TOPIC};
/// Payload and topic constant for `photon_publish_errors`.
pub use crate::schemas::photon_publish_errors::{
    PhotonPublishErrorsPayload, PHOTON_PUBLISH_ERRORS_TOPIC,
};
/// Payload and topic constant for `photon_publishes`.
pub use crate::schemas::photon_publishes::{PhotonPublishesPayload, PHOTON_PUBLISHES_TOPIC};
