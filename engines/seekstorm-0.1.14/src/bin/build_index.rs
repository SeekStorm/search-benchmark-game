//use futures::executor::block_on;
use seekstorm::index::{
    create_index, AccessType, Document, IndexDocument, IndexMetaObject, SimilarityType,
    TokenizerType,
};

use std::env;
use std::error::Error;
use std::io::BufRead;
use std::path::Path;
use std::sync::Arc;
use std::thread::available_parallelism;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();
    let index_path = &Path::new(&args[1]); //Path::new("C:/index/");//

    // create schema
    let schema_json = r#"
    [{"field_name":"text","field_type":"Text","field_stored":false,"field_indexed":true},
    {"field_name":"id","field_type":"Text","field_stored":false,"field_indexed":false}]"#;
    let schema = serde_json::from_str(schema_json).unwrap();

    // create index
    let meta = IndexMetaObject {
        id: 0,
        name: "test_index".to_string(),
        similarity: SimilarityType::Bm25f,
        tokenizer: TokenizerType::AsciiAlphabetic,
        access_type: AccessType::Mmap,
    };
    let serialize_schema = true;
    let segment_number_bits1 = 11;
    let index = create_index(
        index_path,
        meta,
        &schema,
        serialize_schema,
        segment_number_bits1,
    )
    .unwrap();
    let index_arc = Arc::new(RwLock::new(index));

    let thread_number = available_parallelism().unwrap().get();
    let index_arc_clone2 = index_arc.clone();
    let index_ref = index_arc_clone2.read().await;
    let index_permits = index_ref.permits.clone();
    drop(index_ref);

    // index documents
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let index_arc_clone = index_arc.clone();
        let line = line.unwrap_or("".to_string());
        if line.trim().is_empty() {
            continue;
        }

        let document: Document = serde_json::from_str(&line).unwrap();
        index_arc_clone.index_document(document).await;
    }

    // consume all permits / complete indexing threads before commit
    let mut permit_vec = Vec::new();
    for _i in 0..thread_number {
        permit_vec.push(index_permits.acquire().await.unwrap());
    }

    // commit
    let mut index_mut = index_arc.write().await;
    let indexed_doc_count = index_mut.indexed_doc_count;
    index_mut.commit(indexed_doc_count);

    Ok(())
}
