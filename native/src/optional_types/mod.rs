extern crate tantivy;
extern crate neon;

use tantivy:: {
    query::{Query, QueryParser, AllQuery},
    schema::{TextOptions, Field, Facet},
    collector::FacetCollector,
};

use neon::prelude::*;

declare_types! {
    pub class JsQuery for OptionalQuery {
        init(mut cx) {
            Ok(OptionalQuery { query: None } )
        }
    }

    pub class JsFacet for OptionalFacet {
        init(mut cx) {
            let facet = Facet::root();
            let optional_facet = OptionalFacet {
                facet: Some(facet)
            };
            Ok(optional_facet)
        }

        method from(mut cx) {
            let facet_str = cx.argument::<JsString>(0)?.value();

            let facet = Facet::from(&facet_str);
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut instance = this.borrow_mut(&guard);
                instance.facet.replace(facet);
            }

            Ok(cx.this().upcast())
        }
    }


    pub class JsFacetCollector for OptionalFacetCollector {
        init(mut cx) {
            Ok(OptionalFacetCollector {
                facet_collector: None
            })
        }

        method forField(mut cx) {
            let number = cx.argument::<JsNumber>(0)?.value();
            let field = Field(number as u32);
            // let arg = arg.downcast::<JsNumber>().unwrap();

            {
                let mut this = cx.this();
                let guard = cx.lock();

                let mut instance = this.borrow_mut(&guard);

                if instance.facet_collector.is_some() {
                    return Err(neon::result::Throw);
                }

                // let mut facet_collector = instance.facet_collector.as_mut();
                
                let mut collector = FacetCollector::for_field(field);
                instance.facet_collector = Some(collector);
                // Ok(())
            };

            Ok(cx.this().upcast())
        }

        method addFacet(mut cx) {
            let arg = cx.argument::<JsValue>(0)?;
            let mut arg = arg.downcast::<JsFacet>().unwrap();

            {   
                
                let mut this = cx.this();
                let guard = cx.lock();
                let mut field = arg.borrow_mut(&guard);
                // println!("{:?}", field.facet);
                let field = field.facet.take().expect("Facet already consumed");

                let mut instance = this.borrow_mut(&guard);
                let mut instance = instance.facet_collector.as_mut().expect("Called `for_field` on a consumed FacetCollector");
    
                instance.add_facet(field)
            }

            Ok(cx.this().upcast())
        }
    }
}

pub trait Optional {}
pub struct OptionalQuery {
    pub query: Option<Box<Query>>
}
impl Optional for OptionalQuery {}

pub struct OptionalFacet {
    pub facet: Option<Facet>
}
impl Optional for OptionalFacet {}

pub struct OptionalFacetCollector {
    pub facet_collector: Option<FacetCollector>
}


impl Optional for OptionalFacetCollector {}