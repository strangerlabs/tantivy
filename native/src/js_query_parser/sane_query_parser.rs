use tantivy::{
    query::{QueryParser, Query},
    Index,
    schema::Field,
};

pub struct SaneQueryParser {
    query_parser: QueryParser
}

impl SaneQueryParser {
    pub fn new(index: &Index, fields: Vec<Field>) -> SaneQueryParser {
        let query_parser = QueryParser::for_index(index, fields);
        SaneQueryParser {
            query_parser
        }
             
    }

    pub fn parse(&self, query_str: &str) -> Box<dyn Query> {
        self.query_parser.parse_query(query_str).unwrap()
    }
}