# messenger-search

A Rust CLI tool (and underlying library) that uses [Tantivy](https://github.com/tantivy-search/tantivy) to index and search downloaded Messenger conversation archives.

This tool is currently not in a state where it is usable by the general public.

## Project Structure

* **messenger-search**: A library that provides functionality to parse JSON conversation archives, generate a Tantivy index using the parsed results, and perform search queries against the index.
* **messenger-search-cli**: A tool that exposes the functionality of the `messenger-search` library to the command line.

## Motivation

Messenger's built-in search feature appears to provide no way to perform phrase queries, making it pretty useless. Also, Tantivy is really cool and I felt like doing something with it.
