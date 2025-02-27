use codecs::decoding::BoxedFramingError;
use metrics::counter;
use vector_core::internal_event::InternalEvent;

use super::prelude::{error_stage, error_type};

#[derive(Debug)]
pub struct JournaldInvalidRecordError {
    pub error: serde_json::Error,
    pub text: String,
}

impl InternalEvent for JournaldInvalidRecordError {
    fn emit(self) {
        error!(
            message = "Invalid record from journald, discarding.",
            error = ?self.error,
            text = %self.text,
            stage = error_stage::PROCESSING,
            error_type = error_type::PARSER_FAILED,
        );
        counter!(
            "component_errors_total", 1,
            "stage" => error_stage::PROCESSING,
            "error_type" => error_type::PARSER_FAILED,
        );
        counter!("invalid_record_total", 1); // deprecated
        counter!("invalid_record_bytes_total", self.text.len() as u64); // deprecated
    }
}

#[derive(Debug)]
pub struct JournaldStartJournalctlError {
    pub error: crate::Error,
}

impl InternalEvent for JournaldStartJournalctlError {
    fn emit(self) {
        error!(
            message = "Error starting journalctl process.",
            error = %self.error,
            stage = error_stage::RECEIVING,
            error_type = error_type::COMMAND_FAILED,
        );
        counter!(
            "component_errors_total", 1,
            "stage" => error_stage::RECEIVING,
            "error_type" => error_type::COMMAND_FAILED,
        );
    }
}

#[derive(Debug)]
pub struct JournaldReadError {
    pub error: BoxedFramingError,
}

impl InternalEvent for JournaldReadError {
    fn emit(self) {
        error!(
            message = "Cound not read from journald.",
            error = %self.error,
            stage = error_stage::PROCESSING,
            error_type = error_type::READER_FAILED,
        );
        counter!(
            "component_errors_total",
            1,
            "stage" => error_stage::PROCESSING,
            "error_type" => error_type::READER_FAILED,
        );
    }
}

#[derive(Debug)]
pub struct JournaldCheckpointSetError {
    pub error: std::io::Error,
    pub filename: String,
}

impl InternalEvent for JournaldCheckpointSetError {
    fn emit(self) {
        error!(
            message = "Could not set journald checkpoint.",
            filename = ?self.filename,
            error = %self.error,
            stage = error_stage::PROCESSING,
            error_type = error_type::IO_FAILED,
        );
        counter!(
            "component_errors_total", 1,
            "stage" => error_stage::PROCESSING,
            "error_type" => error_type::IO_FAILED,
        );
    }
}

#[derive(Debug)]
pub struct JournaldCheckpointFileOpenError {
    pub error: std::io::Error,
    pub path: String,
}

impl InternalEvent for JournaldCheckpointFileOpenError {
    fn emit(self) {
        error!(
            message = "Unable to open checkpoint file.",
            path = ?self.path,
            error = %self.error,
            stage = error_stage::RECEIVING,
            error_type = error_type::IO_FAILED,
        );
        counter!(
            "component_errors_total", 1,
            "stage" => error_stage::RECEIVING,
            "error_type" => error_type::IO_FAILED,
        );
    }
}
