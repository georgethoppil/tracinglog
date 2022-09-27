mod custom_layer;

use tracing::{debug, error, event, info, Level, span};
use crate::custom_layer::{CustomLayer};
use tracing_subscriber::prelude::*;

fn main() {
    let (custom_layer, logs) = CustomLayer::new();
    tracing_subscriber::registry().with(custom_layer).init();
    let span = span!(Level::INFO, "doing_something", level = 1, field="5").entered();
    span.record("field", "90");
    info!("Test {}", 5);
    error!("Void error {}", true);
    debug!("Debug {}", "a problem");
    event!(Level::INFO, answer = 42, something = "life, the universe, and everything");
    event!(Level::INFO, answer_two = 50);
    event!(Level::ERROR, error_tag = "We got an error boss");
    event!(Level::TRACE, trace_tag = "Some tag");
    span.exit();

    logs.read();

}
