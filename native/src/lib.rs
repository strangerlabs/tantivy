// #![deny(warnings)]
#[macro_use]
extern crate neon;
extern crate tantivy;
extern crate serde_derive;

use neon::prelude::*;

mod optional_types;
use optional_types::{JsFacet, JsFacetCollector};

mod js_search;
use js_search::JsSearch;

mod js_schema_builder;
use js_schema_builder::JsSchemaBuilder;

mod js_query_parser;
use js_query_parser::JsQueryParser;

mod js_top_docs;
use js_top_docs::JsTopDocs;

register_module!(mut cx, {
    cx.export_class::<JsSearch>("Search")?;
    cx.export_class::<JsSchemaBuilder>("SchemaBuilder")?;
    cx.export_class::<JsTopDocs>("TopDocs")?;
    cx.export_class::<JsQueryParser>("QueryParser")?;
    cx.export_class::<JsFacet>("Facet")?;
    cx.export_class::<JsFacetCollector>("FacetCollector")?;
    Ok(())
});
