extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate tantivy;

mod data_format;
mod error;

use std::path::Path;
use tantivy::Index;
use tantivy::schema::*;
use tantivy::collector::TopCollector;
use tantivy::query::QueryParser;
use tantivy::directory::Directory;
use error::Error;
use data_format::{Conversation, StoredMessage};

/// Generates an index for the conversation described by the JSON at the given path,
/// storing the generated index in the given directory (pre-existing indexes will be
/// over-written).
/// 
/// Returns the a tuple of the `Index` and the `opstamp` of the last successfully
/// committed document, or an error if something went wrong.
pub fn generate_index<D: Directory, P: AsRef<Path>>(
    index_dir: D,
    json_path: P
) -> Result<(Index, u64), Error> {
    let mut schema_builder = SchemaBuilder::default();
    schema_builder.add_text_field("content", TEXT | STORED);
    schema_builder.add_i64_field("timestamp", INT_STORED | INT_INDEXED);

    let schema = schema_builder.build();
    let index = Index::create(index_dir, schema.clone())?;
    // Writer created with 50 MB of heap
    let mut index_writer = index.writer(50_000_000)?;

    // Get fields to create documents with them
    let content = schema.get_field("content").unwrap();
    let timestamp = schema.get_field("timestamp").unwrap();

    let conversation = Conversation::from_json_file(json_path)?;

    for message in conversation.messages {
        index_writer.add_document(doc! {
            content => message.content,
            timestamp => message.timestamp_ms
        });
    }

    Ok((index, index_writer.commit()?))
}

/// Runs the given query with the given index.
/// 
/// Returns messages that matched the query or an error if something went wrong.
/// 
/// # Panics
/// 
/// If any tantivy document returned from the query fails to parse into a
/// `StoredMessage`, this function will panic.
pub fn search(
    index: &Index,
    query: &str
) -> Result<Vec<StoredMessage>, Error> {
    let schema = index.schema();
    index.load_searchers()?;

    let searcher = index.searcher();
    let query_parser = QueryParser::for_index(
        &index,
        vec![
            schema.get_field("content").unwrap(),
            // schema.get_field("timestamp").unwrap()
        ]
    );

    let query = query_parser.parse_query(query)?;
    // TODO: Don't hardcode limit
    let mut top_collector = TopCollector::with_limit(10);

    searcher.search(&*query, &mut top_collector)?;
    let doc_addresses = top_collector.docs();

    let mut messages = vec![];
    for address in doc_addresses {
        let retrieved_doc = searcher.doc(address)?;

        match StoredMessage::from_json_str(&schema.to_json(&retrieved_doc)) {
            Ok(message) => messages.push(message),
            // TODO: Improve panic message, right now it's "Box<Any>"
            Err(e) => panic!(e)
        }
    }

    Ok(messages)
}

/// Attempts to open an index in the given directory
pub fn open_index_in<P: AsRef<Path>>(folder: P) -> Result<Index, Error> {
    Ok(Index::open_in_dir(folder)?)
}

#[cfg(test)]
mod test {
    use super::{generate_index, search};
    use tantivy::directory::RAMDirectory;

    #[test]
    fn test_generate_index_and_query() {
        let (idx, _) = generate_index(RAMDirectory::create(), "sample-data/message.json").unwrap();

        // Some simple asserts to ensure things are still working
        assert_eq!(search(&idx, "test data").unwrap().len(), 5);
        assert_eq!(search(&idx, "github").unwrap().len(), 1);
    }
}
