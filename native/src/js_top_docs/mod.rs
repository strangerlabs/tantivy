extern crate neon;
extern crate tantivy;

use neon::prelude::*;
use tantivy::collector::TopDocs;

declare_types! {
    pub class JsTopDocs for TopDocs {
        init(mut cx) {
            let n_docs = cx.argument::<JsNumber>(0)?.value() as usize;
            Ok(TopDocs::with_limit(n_docs))
        }
    }
}