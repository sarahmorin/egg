//! Utilities for recording Rerun metrics during optimization experiments
//! This module provides helper functions to create Rerun recordings and log optimization metrics in a standardized way.
//! It is intended to be used in integration tests or benchmarks to capture detailed performance data for analysis in the Rerun dashboard.
// #![cfg(feature = "rerun-metrics")]
use const_format::formatcp;
use log::warn;
use rerun;

use crate::{Iteration, Report};

pub type RerunStream = Option<rerun::RecordingStream>;

pub const EGG_PREFIX: &str = "egg";
pub const ITERATION: &str = formatcp!("{EGG_PREFIX}/iteration");
pub const SUMMARY: &str = formatcp!("{EGG_PREFIX}/summary");

pub const ITER_HOOK_TIME: &str = formatcp!("{ITERATION}/hook_time_s");
pub const ITER_SEARCH_TIME: &str = formatcp!("{ITERATION}/search_time_s");
pub const ITER_APPLY_TIME: &str = formatcp!("{ITERATION}/apply_time_s");
pub const ITER_REBUILD_TIME: &str = formatcp!("{ITERATION}/rebuild_time_s");
pub const ITER_TOTAL_TIME: &str = formatcp!("{ITERATION}/total_time_s");
pub const ITER_REBUILD_COUNT: &str = formatcp!("{ITERATION}/rebuild_count");
pub const ITER_APPLY_COUNT: &str = formatcp!("{ITERATION}/apply_count");

pub const SUMM_NODES: &str = formatcp!("{SUMMARY}/egraph_nodes");
pub const SUMM_CLASSES: &str = formatcp!("{SUMMARY}/egraph_classes");
pub const SUMM_SEARCH_TIME: &str = formatcp!("{SUMMARY}/search_time_s");
pub const SUMM_APPLY_TIME: &str = formatcp!("{SUMMARY}/apply_time_s");
pub const SUMM_REBUILD_TIME: &str = formatcp!("{SUMMARY}/rebuild_time_s");
pub const SUMM_TOTAL_TIME: &str = formatcp!("{SUMMARY}/total_time_s");
pub const SUMM_REBUILD_COUNT: &str = formatcp!("{SUMMARY}/rebuild_count");
pub const SUMM_ITERATIONS: &str = formatcp!("{SUMMARY}/iterations");

pub fn log_rerun_iteration<IterData>(
    rec: &RerunStream,
    iter_index: usize,
    iter: &Iteration<IterData>,
    report: &Report,
) {
    if rec.is_none() {
        warn!(
            "Attempted to log rerun metrics for iteration {}, but no recording stream is available",
            iter_index
        );
        return;
    }

    // Node/class counts are not cumulative; emit them once on summary paths to avoid duplicated metrics.
    log_scalar(rec, SUMM_NODES, report.egraph_nodes as f64);
    log_scalar(rec, SUMM_CLASSES, report.egraph_classes as f64);

    log_scalar(rec, ITER_HOOK_TIME, iter.hook_time);
    log_scalar(rec, ITER_SEARCH_TIME, iter.search_time);
    log_scalar(rec, ITER_APPLY_TIME, iter.apply_time);
    log_scalar(rec, ITER_REBUILD_TIME, iter.rebuild_time);
    log_scalar(rec, ITER_TOTAL_TIME, iter.total_time);
    log_scalar(rec, ITER_REBUILD_COUNT, iter.n_rebuilds as f64);
    let applied_total: usize = iter.applied.values().copied().sum();
    log_scalar(rec, ITER_APPLY_COUNT, applied_total as f64);

    log_scalar(rec, SUMM_ITERATIONS, report.iterations as f64);
    log_scalar(rec, SUMM_REBUILD_COUNT, report.rebuilds as f64);
    log_scalar(rec, SUMM_SEARCH_TIME, report.search_time);
    log_scalar(rec, SUMM_APPLY_TIME, report.apply_time);
    log_scalar(rec, SUMM_REBUILD_TIME, report.rebuild_time);
    log_scalar(rec, SUMM_TOTAL_TIME, report.total_time);
}

#[allow(dead_code)]
pub fn log_rerun_summary(_rec: &RerunStream, _report: &Report) {}

pub fn log_scalar(rec: &RerunStream, entity_path: &str, value: f64) {
    if rec.is_none() {
        warn!(
            "Attempted to log rerun scalar '{}' with value {}, but no recording stream is available",
            entity_path, value
        );
        return;
    }

    if let Err(err) = rec
        .as_ref()
        .unwrap()
        .log(entity_path, &rerun::Scalars::new([value]))
    {
        warn!(
            "Failed to emit rerun scalar '{}' with value {}: {}",
            entity_path, value, err
        );
    }
}
