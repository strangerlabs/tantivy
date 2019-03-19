const { Search, SchemaBuilder, TopDocs, QueryParser } = require('../native')

// Initialize the Search object
let search = new Search()


// 1) Building a search schema
// Instantiate a SchemaBuilder object
let schemaBuilder = new SchemaBuilder()

// Add fields to the schema
// `addTextField` returns a handle to that field
// The handle is similar to a file descriptor in that it is a number (u32)

schemaBuilder.addTextField("_id", ["STRING"] )
let title = schemaBuilder.addTextField("title", ["TEXT", "STORED"] )
let year = schemaBuilder.addTextField("year", ["TEXT", "STORED"] )
let authors = schemaBuilder.addTextField("authors", ["TEXT", "STORED"] )
let url = schemaBuilder.addTextField("url", ["TEXT", "STORED"] )

// Build the schema
search.buildSchema(schemaBuilder)

// Set the default search fields
// This is to allow one-liner searching with `simpleSearch()`
search.defaultSearchFields([title, year, authors, url])


// 2) Create the index
// An error will throw if the index path does not exist. 
// If an index already exists then the previous index will be overwritten.
// To open a pre existing index use `search.openIndex(<path>)`
search.createIndex('./data')

// 3) Adding documents
// If a document contains a field that isn't specified in the schema, 
// `.addDoc(...)` will throw. 
// This behavior may change in the future

// Create an index writer
// The first argument is the buffer size in bytes
search.createIndexWriter(100000000)

let document = { 
  _id: "1", 
  title: "The Economic History of the Fur Trade: 1670 to 1870",
  year: "2008",
  authors: ["Ann M. Carlos, University of Colorado", "Frank D. Lewis, Queen’s University"],
  url: "http://eh.net/encyclopedia/the-economic-history-of-the-fur-trade-1670-to-1870/"
}


let document2 = {
  _id: "2",
  title: "Selling Beaver Skins in North America and Europe, 1720-1760: The Uses of Fur-Trade Imperialism.",
  year: "1990",
  authors: ["Wien, Thomas"],
  // url: ""
}

// addDoc may block if the pipeline is full
search.addDoc(JSON.stringify(document))
search.addDoc(JSON.stringify(document2))

// Commit
// Commit will create a point in which `Tantivy` will be able to rollback to 
// in the event of power-loss or someother catestrophic failure.
// Commit is blocking.
search.commit()

// loadSearchers
// Notify the segment searchers that the index has changed. 
// Only after calling this, any documents added will be visible to the searchers
search.loadSearchers()

// 4) Query parsing
// QueryParser
// Create a QueryParser object.
let queryParser = new QueryParser(search, [title, year, authors, url])

// Parse a query string
// This may fail if the provided string is not in the right format 
let query = queryParser.parse("fur")

// Search
// Create a collector.
// The first argument is the limit of documents to return
let collector = new TopDocs(10)

// Execute the search
// `top_search()` returns an array of arrays
// The inside array contains the DocScore (relevance rating between 0 and 1)
// as the first value and the search result as the second.
let results = search.topSearch(query, collector)

console.log(results)
