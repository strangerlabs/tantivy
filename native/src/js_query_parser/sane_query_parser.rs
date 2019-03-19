use tantivy::{
    query::{QueryParser, Query},
    Index,
    schema::Field,
};

pub struct SaneQueryParser {
    query_parser: Option<QueryParser>
}

impl SaneQueryParser {
    pub fn new(index: &Index, fields: Vec<Field>) -> SaneQueryParser {
        let query_parser = QueryParser::for_index(index, fields);
        let query_parser = Some(query_parser);
        SaneQueryParser {
            query_parser
        }
             
    }

    pub fn parse(&self, query_str: &str) -> Box<dyn Query>{
        let query_parser = self.query_parser.as_ref().expect("No query parser availble");
        query_parser.parse_query(query_str).unwrap()
    }
}