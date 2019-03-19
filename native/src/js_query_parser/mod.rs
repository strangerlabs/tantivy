extern crate neon;
extern crate tantivy;


use neon::prelude::*;

use tantivy::{
    query::{Query, QueryParser, AllQuery},
    Searcher,
    DocAddress,
    Score,
    schema::{Schema, SchemaBuilder, TextOptions, Field, Facet},
    Index, IndexWriter,
    collector::{TopDocs, FacetCollector, MultiCollector},
};

mod sane_query_parser;
use sane_query_parser::SaneQueryParser;

use crate::optional_types::JsQuery;
use crate::js_search::JsSearch;

declare_types! {
    pub class JsQueryParser for SaneQueryParser {
        init(mut cx) {

            let sane_search = cx.argument::<JsValue>(0)?; 
            let sane_search = sane_search.downcast::<JsSearch>().unwrap();

            let fields = {
                let js_arr_handle: Handle<JsArray> = cx.argument(1)?;
                let vec: Vec<Handle<JsValue>> = js_arr_handle.to_vec(&mut cx)?;

                let mut fields: Vec<Field> = vec![];

                for handle in vec.iter() {
                    let field: Handle<JsNumber> = handle.downcast().unwrap();
                    let field = field.value();
                    let field: Field = Field(field as u32);
                    fields.push(field);
                }
                fields.clone()
            };

            let parser = { 
                let guard = cx.lock();
                let mut instance = sane_search.borrow(&guard);
                let index = instance.index.as_ref().expect("Called QueryParse constructer on an unintialized index");
                let parser = SaneQueryParser::new(index, fields);
                parser
            };
           
            
            Ok(parser)
        }

        method parse(mut cx) {
            let query_str = cx.argument::<JsString>(0)?.value();

            let query = {
                let this = cx.this();
                let guard = cx.lock();
                let instance = this.borrow(&guard);
                instance.parse(&query_str)
            };

            let js_query =  {
                let undefined = cx.undefined();
                let mut js_query = JsQuery::new(&mut cx, std::iter::once(undefined))?;
                let guard = cx.lock();
                {
                    let mut borrowed_query = js_query.borrow_mut(&guard);
                    borrowed_query.query = Some(query);
                }
                js_query
            };

            Ok(js_query.upcast())
        }
    }

}