mod custom_layer;
mod logs;

use std::env::temp_dir;
use std::io;
use lazy_static::lazy_static;
use tracing::{debug, error, event, info, info_span, Level, span, Subscriber};
use tracing::field::debug;
use tracing::instrument::WithSubscriber;
use tracing_subscriber::fmt;
use crate::custom_layer::{CustomLayer, RunResult};
use tracing_subscriber::prelude::*;

fn main() {
    tracing_subscriber::registry().with(CustomLayer).init();
    let span = span!(Level::INFO, "doing_something", level = 1, field="5").entered();
    span.record("field", "90");
    info!("Test {}", 5);
    error!("Void error {}", true);
    debug!("Debug {}", "a problem");
    event!(Level::INFO, answer = 42, something = "life, the universe, and everything");
    event!(Level::INFO, answer_two = 50);
    event!(Level::ERROR, error_tag = "We got an error boss");
    event!(Level::TRACE, trace_tag = "Some tag");
    // customerLayer.append_logs_to_run_result(runResult);
    span.exit();

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .finish();

    let s = tracing::subscriber::with_default(subscriber, || {
        event!(Level::INFO, answer_two = 50);
        event!(Level::ERROR, error_tag = "We got an error boss");
        // println!("{:?}", subscriber.current_span().metadata());
        "done"
    });
}
