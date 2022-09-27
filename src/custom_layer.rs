use std::collections::{BTreeMap};
use std::error::Error;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use tracing::{Event, Id, Subscriber};
use tracing::field::Field;
use tracing::span::{Attributes, Record};
use tracing_subscriber::Layer;
use tracing_subscriber::layer::{Context};
use tracing_subscriber::registry::LookupSpan;
use chrono::prelude::*;

#[derive(Clone)]
pub struct Logs(Arc<RwLock<Vec<String>>>);

impl Logs {
    pub fn read(&self) {
        for item in self.0.read().unwrap().iter() {
            println!("{}", item);
        }
    }
}

pub struct CustomLayer {
    logs: Logs,
}

impl CustomLayer {
    pub fn new() -> (Self, Logs) {
        let logs = Logs(Arc::new(RwLock::new(vec![])));
        let layer = CustomLayer {
            logs: logs.clone(),
        };
        (layer, logs)
    }
}


#[derive(Debug)]
struct CustomFieldStorage(Vec<BTreeMap<String, String>>);

#[derive(Debug)]
struct Visitor<'a>(&'a mut BTreeMap<String, String>);

impl<S> Layer<S> for CustomLayer
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
        S: for<'lookup> LookupSpan<'lookup>
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let mut fields = BTreeMap::new();
        let mut visitor = Visitor(&mut fields);
        visitor.0.insert("time".to_string(), Utc::now().to_string());
        attrs.record(&mut visitor);

        let mut storage_array = Vec::new();
        storage_array.push(fields);
        let storage = CustomFieldStorage(storage_array);

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
        let mut extensions_mut = span.extensions_mut();
        let custom_field_storage: &mut CustomFieldStorage =
            extensions_mut.get_mut::<CustomFieldStorage>().unwrap();
        let json_array_data: &mut Vec<BTreeMap<String, String>> = &mut custom_field_storage.0;

        let mut fields:  BTreeMap<String, String> =  BTreeMap::new();

        let mut visitor = Visitor(&mut fields);
        visitor.0.insert("time".to_string(), Utc::now().to_string());
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
            let json_array_data: &mut Vec<BTreeMap<String, String>> = &mut custom_field_storage.0;

            let mut fields:  BTreeMap<String, String> =  BTreeMap::new();

            let mut visitor = Visitor(&mut fields);
            visitor.0.insert("level".to_string(), event.metadata().level().to_string());
            visitor.0.insert("line".to_string(), event.metadata().line().unwrap().to_string());
            visitor.0.insert("file".to_string(), event.metadata().file().unwrap().to_string());
            visitor.0.insert("time".to_string(), Utc::now().to_string());

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
        let mut log = self.logs.0.write().unwrap();
        for item in &storage.0 {
            log.push(format!("{:?}", item));
        }
    }
}

impl<'a> tracing::field::Visit for Visitor<'a> {
    fn record_f64(&mut self, field: &Field, value: f64) {
        self.0
            .insert(field.name().to_string(), value.to_string());
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.0
            .insert(field.name().to_string(), value.to_string());
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.0
            .insert(field.name().to_string(), value.to_string());
    }

    fn record_i128(&mut self, field: &Field, value: i128) {
        self.0
            .insert(field.name().to_string(), value.to_string());
    }

    fn record_u128(&mut self, field: &Field, value: u128) {
        self.0
            .insert(field.name().to_string(), value.to_string());
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.0
            .insert(field.name().to_string(), value.to_string());
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.0
            .insert(field.name().to_string(), value.to_string());
    }

    fn record_error(&mut self, field: &Field, value: &(dyn Error + 'static)) {
        self.0
            .insert(field.name().to_string(), value.to_string());
    }

    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        self.0
            .insert(field.name().to_string(), format!("{:?}", value));
    }
}