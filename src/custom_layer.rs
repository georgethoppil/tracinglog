use std::alloc::System;
use std::any::TypeId;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Debug;
use tracing::{Event, Id, Instrument, Metadata, Subscriber};
use tracing::field::Field;
use tracing::level_filters::LevelFilter;
use tracing::span::{Attributes, Record};
use tracing::subscriber::Interest;
use tracing_subscriber::filter::Filtered;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::{Context, Filter, Layered};
use tracing_subscriber::registry::LookupSpan;
use std::time::{SystemTime};
use chrono::prelude::*;
use crate::logs::LogStream;


pub struct RunResult {
    logs: LogStream,
}

impl Default for RunResult {
    fn default() -> Self {
        Self {
            logs: LogStream::default()
        }
    }
}


pub struct CustomLayer;

#[derive(Debug)]
struct CustomFieldStorage(Vec<BTreeMap<String, serde_json::Value>>);

#[derive(Debug)]
struct Visitor<'a>(&'a mut BTreeMap<String, serde_json::Value>);

impl<S> Layer<S> for CustomLayer
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
        S: for<'lookup> LookupSpan<'lookup>
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let mut fields = BTreeMap::new();
        let mut visitor = Visitor(&mut fields);
        attrs.record(&mut visitor);

        let mut storageArray = Vec::new();
        storageArray.push(fields);
        let storage = CustomFieldStorage(storageArray);

        let span = ctx.span(id).unwrap();
        let mut extensions = span.extensions_mut();
        extensions.insert::<CustomFieldStorage>(storage);
    }

    fn on_record(
        &self,
        id: &Id,
        values: &Record<'_>,
        ctx: Context<'_, S>,
    ) {
        let span = ctx.span(id).unwrap();
        println!("{:?}", values);
        let mut extensions_mut = span.extensions_mut();
        let custom_field_storage: &mut CustomFieldStorage =
            extensions_mut.get_mut::<CustomFieldStorage>().unwrap();
        let mut json_array_data: &mut Vec<BTreeMap<String, serde_json::Value>> = &mut custom_field_storage.0;

        let mut fields:  BTreeMap<String, serde_json::Value> =  BTreeMap::new();

        let mut visitor = Visitor(&mut fields);
        values.record(&mut visitor);

        json_array_data.push(fields);
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let current_span = ctx.current_span();
        if let Some(id) = current_span.id() {
            let span = ctx.span(&id).expect("Span not found, this is a bug");
            let mut extensions_mut = span.extensions_mut();
            let custom_field_storage: &mut CustomFieldStorage =
                extensions_mut.get_mut::<CustomFieldStorage>().unwrap();
            let mut json_array_data: &mut Vec<BTreeMap<String, serde_json::Value>> = &mut custom_field_storage.0;

            let mut fields:  BTreeMap<String, serde_json::Value> =  BTreeMap::new();

            let mut visitor = Visitor(&mut fields);
            visitor.0.insert("level".to_string(), serde_json::json!(format!("{:?}",event.metadata().level())));
            visitor.0.insert("line".to_string(), serde_json::json!(event.metadata().line()));
            visitor.0.insert("file".to_string(), serde_json::json!(event.metadata().file()));
            visitor.0.insert("time".to_string(), serde_json::json!(format!("{:?}", Utc::now())));


            event.record(&mut visitor);
            json_array_data.push(fields);
        }
    }

    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        let span = ctx.span(&id).expect("Span not found, this is a bug");
        let extensions = span.extensions();
        let storage= extensions
            .get::<CustomFieldStorage>()
            .expect("storage not found");
        println!("Closing span and has recorded {:#?}", storage);
        // self.logger.push("hello World".to_string());

    }
}

impl<'a> tracing::field::Visit for Visitor<'a> {
    fn record_f64(&mut self, field: &Field, value: f64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_i128(&mut self, field: &Field, value: i128) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_u128(&mut self, field: &Field, value: u128) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_error(&mut self, field: &Field, value: &(dyn Error + 'static)) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(format!("{:?}",value)));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(format!("{:?}",value)));
    }
}
