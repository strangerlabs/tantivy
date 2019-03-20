# tantivy

[![standard-readme compliant](https://img.shields.io/badge/standard--readme-OK-green.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)

> NodeJS bindings for Tantivy

TODO: Fill out this long description.

## Semver Notice

Please note this is an unstable API. Until 1.0.0 is released, breaking changes may be published without a major version increment.

## Table of Contents

- [Install](#install)
- [Usage](#usage)
- [API](#api)
- [Maintainers](#maintainers)
- [Contributing](#contributing)
- [License](#license)

## Install

Clone this repository.

Then run the following in the project's root directory.

```sh
npm i
npm run build

# for for use with electron
npm run build-electron
```

## Usage

```js
const { Search, SchemaBuilder, TopDocs, QueryParser } = require('@strangerlabs/tantivy')

let search = new Search()
let schemaBuilder = new SchemaBuilder()

schemaBuilder.addTextField("_id", ["STRING"] )
let title = schemaBuilder.addTextField("title", ["TEXT", "STORED"] )
let year = schemaBuilder.addTextField("year", ["TEXT", "STORED"] )
let authors = schemaBuilder.addTextField("authors", ["TEXT", "STORED"] )
let url = schemaBuilder.addTextField("url", ["TEXT", "STORED"] )

search.buildSchema(schemaBuilder)
search.defaultSearchFields([title, year, authors, url])

search.createIndex('./data')
search.createIndexWriter(100000000)

let document = {
  _id: "1",
  title: "The Economic History of the Fur Trade: 1670 to 1870",
  year: "2008",
  authors: ["Ann M. Carlos, University of Colorado", "Frank D. Lewis, Queen’s University"],
  url: "http://eh.net/encyclopedia/the-economic-history-of-the-fur-trade-1670-to-1870/"
}

search.addDoc(JSON.stringify(document))

search.commit()
search.loadSearchers()

let queryParser = new QueryParser(search, [title, year, authors, url])
let query = queryParser.parse("fur")
let collector = new TopDocs(10)

let results = search.topSearch(query, collector)

console.log(results)
```

## API

Please browse to the examples folder for line-by-line documentation of the API.

## Maintainers

StJohn Giddy [@thecallsign](https://github.com/thecallsign)

## Contributing

PRs accepted.

Small note: If editing the README, please conform to the [standard-readme](https://github.com/RichardLitt/standard-readme) specification.

## License

MIT © 2019 Stranger Labs, Inc.
