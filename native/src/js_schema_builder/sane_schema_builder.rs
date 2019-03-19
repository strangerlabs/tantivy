extern crate tantivy;
use std::cell::RefCell;
use std::rc::Rc;


use tantivy::{
    schema::{Schema, SchemaBuilder, TextOptions, Field},
};

pub struct SaneSchemaBuilder {
    schema_builder: Option<SchemaBuilder>
}

impl SaneSchemaBuilder {
    pub fn new() -> SaneSchemaBuilder {
        let schema_builder = Schema::builder();

        SaneSchemaBuilder {
            schema_builder: Some(schema_builder)
        }
    }

    pub fn add_text_field(&mut self, field: &str, text_options: TextOptions) -> Field {
        let option = &mut self.schema_builder;
        let builder = option.as_mut().expect("No interior SchemaBuilder (Have you already called `.build()`?");
        builder.add_text_field(field, text_options)
    }

    pub fn add_facet_field(&mut self, field: &str) -> Field {
        let option = &mut self.schema_builder;
        let builder = option.as_mut().expect("No interior SchemaBuilder (Have you already called `.build()`?");
        builder.add_facet_field(field)
    }

    pub fn build(&mut self) -> Schema {
        let builder = &mut self.schema_builder;
        let builder = builder.take().expect("No interior SchemaBuilder (Have you already called `.build()`?");
        builder.build()
    }
}
