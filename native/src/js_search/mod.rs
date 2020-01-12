extern crate neon;
extern crate tantivy;


use tantivy::{
    query::{Query, QueryParser, AllQuery},
    Searcher,
    DocAddress,
    Score,
    schema::{Schema, SchemaBuilder, TextOptions, Field, Facet},
    Index, IndexWriter,
    collector::{TopDocs, FacetCollector, MultiCollector},
};
use neon::prelude::*;


mod sane_search;
use sane_search::SaneSearch;


use crate::optional_types::*;
use crate::js_top_docs::JsTopDocs;
use crate::js_schema_builder::JsSchemaBuilder;

declare_types! {
    pub class JsSearch for SaneSearch {
        init(mut _cx) {
            Ok(SaneSearch {
                index: None,
                schema: None,
                index_writer: None,
                default_search_fields: None
            })
        }

        method simpleSearch(mut cx) {
            let search_str = cx.argument::<JsString>(0)?.value(); 
            let this = cx.this();
        
            let results = {
                let guard = cx.lock();
                let instance = this.borrow(&guard);
                instance.simple_search(&search_str).unwrap()
            };
            let js_array = JsArray::new(&mut cx, results.len() as u32);

            for (i, obj) in results.iter().enumerate() {
                let js_string = cx.string(obj);
                js_array.set(&mut cx, i as u32, js_string).unwrap();
            }

            Ok(js_array.upcast()) 
        } 

        method topSearch(mut cx) {
            let query = cx.argument::<JsQuery>(0)?;
            let collector = cx.argument::<JsTopDocs>(1)?;

            let results = {
                let this = cx.this();
                let guard = cx.lock();
                let instance = this.borrow(&guard);

                let borrowed_query = query.borrow(&guard);
                let query = borrowed_query.query.as_ref().expect("Invalid query");

                let collector = collector.borrow(&guard);
                instance.top_search(query, &collector)
            };

            let js_array = JsArray::new(&mut cx, results.len() as u32);

            for (i, obj) in results.iter().enumerate() {
                let inside_js_arr = JsArray::new(&mut cx, 2u32);

                let score = cx.number(obj.0);
                let result = cx.string(&obj.1);

                inside_js_arr.set(&mut cx, 0u32, score).unwrap();
                inside_js_arr.set(&mut cx, 1u32, result).unwrap();
                js_array.set(&mut cx, i as u32, inside_js_arr).unwrap();
            }

            Ok(js_array.upcast())

        }

        method facetSearch(mut cx) {
            let mut collector = cx.argument::<JsFacetCollector>(0)?;

            let results = {
                let this = cx.this();
                let guard = cx.lock();

                let mut optional_collector = collector.borrow_mut(&guard);
                let mut collector = optional_collector.facet_collector.take().expect("invalid collector");
                
                let sanesearch = this.borrow(&guard);
                let index = sanesearch.index.as_ref().expect("facetSearch called on no index");
                let reader = index.reader().expect("Unable to acquire reader");
                let searcher = reader.searcher();
                let default_fields = sanesearch.default_search_fields.as_ref().unwrap();
                // let counts = collector.harvest();

                // let facets: Vec<(&Facet, u64)> = searcher.search(&AllQuery, collector).unwrap()

                let mut multicollector = MultiCollector::default();
                let facet_handler = multicollector.add_collector(collector);
                let topdoc_handler = multicollector.add_collector(TopDocs::with_limit(10));

                let facet = Facet::from("/book");

                // let results: Vec<((Score, DocAddress), (&Facet, u64))> = searcher.search(&AllQuery, &multicollector).unwrap();
                let qp = QueryParser::for_index(&index, default_fields.to_vec());
                let query = qp.parse_query("alex").unwrap();

                let mut multifruits  = searcher.search(&query, &multicollector).unwrap();

                let facet_results = facet_handler.extract(&mut multifruits);
                let topdoc_results = topdoc_handler.extract(&mut multifruits);
                let top = facet_results.top_k(facet, 10);
                //(facet_results.top_k(facet, 10), topdoc_results)
                println!("{:#?}", (top, topdoc_results));

            };
            Ok(cx.undefined().upcast())
        }

        method buildSchema(mut cx) {
            {
                let schema = {
                    let schema_builder = cx.argument::<JsValue>(0)?; 
                    let mut schema_builder = schema_builder.downcast::<JsSchemaBuilder>().unwrap();

                    let guard = cx.lock();
                    let mut instance = schema_builder.borrow_mut(&guard); // &

                    let schema = instance.build();
                    schema
                };

                let mut this = cx.this();
                {
                    let guard = cx.lock();
                    let mut this_instance = this.borrow_mut(&guard);
                    this_instance.schema = Some(schema);
                }
            }

            Ok(cx.string("results").upcast()) 

        }

        method createIndex(mut cx) {
            let path = cx.argument::<JsString>(0)?.value();
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut this = this.borrow_mut(&guard);
                this.create_index(path).unwrap();
            }
            Ok(cx.undefined().upcast())
        }

        method openIndex(mut cx) {
            let path = cx.argument::<JsString>(0)?.value();
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut this = this.borrow_mut(&guard);
                this.open_index(path).unwrap();
            }
            Ok(cx.undefined().upcast())
        }

        method createIndexWriter(mut cx) {
            let heap_size: usize = cx.argument::<JsNumber>(0)?.value() as usize;

            {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut this = this.borrow_mut(&guard);
                this.create_index_writer(heap_size).unwrap();
            }

            Ok(cx.undefined().upcast())
        }

        method addDoc(mut cx) {
            let json = cx.argument::<JsString>(0)?.value();

            {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut this = this.borrow_mut(&guard);
                this.add_doc(&json).unwrap();
            }

            Ok(cx.undefined().upcast())
        }

        
        method commit(mut cx) {

            {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut this = this.borrow_mut(&guard);
                this.commit_index_writer().unwrap();
            }

            Ok(cx.undefined().upcast())
        }

        method loadSearchers(mut cx) {
            
            {
                let this = cx.this();
                let guard = cx.lock();
                let this = this.borrow(&guard);
                this.load_searchers().unwrap();
            }

            Ok(cx.undefined().upcast())
        }

        method defaultSearchFields(mut cx) {
            let js_arr_handle: Handle<JsArray> = cx.argument(0)?;
            let vec: Vec<Handle<JsValue>> = js_arr_handle.to_vec(&mut cx)?;

            let mut fields: Vec<Field> = vec![];

            for handle in vec.iter() {
                let field: Handle<JsNumber> = handle.downcast().unwrap();
                let field = field.value();
                let field: Field = Field::from_field_id(field as u32);
                fields.push(field);
            }

            {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut this = this.borrow_mut(&guard);
                this.set_default_fields(fields);
            }

            Ok(cx.undefined().upcast())
        }

        method printSchema(mut cx) {
            let this = cx.this();
            {
                let guard = cx.lock();
                let instance = this.borrow(&guard);
                let schema = instance.schema.as_ref().expect("no schema found");
                println!("SCHEMA: {:#?}", schema.fields().collect::<Vec<_>>());
            }
            Ok(cx.undefined().upcast())
        }

        method numDocs(mut cx) {
            let this = cx.this();
            let num_docs = {
                let guard = cx.lock();
                let instance = this.borrow(&guard);
                let index = instance.index.as_ref().expect("no index found");
                let reader = index.reader().unwrap();
                let searcher = reader.searcher();
                searcher.num_docs()
            };

            Ok(cx.number(num_docs as f64).upcast())
        }

    }
}