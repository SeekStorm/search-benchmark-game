#![macro_use]
extern crate seekstorm;
use seekstorm::index::{open_index, IndexArc};
use seekstorm::search::{QueryType, ResultType, Search};

use std::env;
use std::error::Error;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;

use gag::Gag;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();

    // open index
    //let index_arc = open_index(Path::new(&args[1])).await.unwrap();
    let index_arc:IndexArc= {
        //mute stdout of open_index
        let _print_gag = Gag::stdout().unwrap();
        open_index(Path::new(&args[1])).await.unwrap()
    };

    let stdin = std::io::stdin();

    //fix
    let mut stdout = std::io::stdout();
    let mut handle = stdout.lock();

    for line_res in stdin.lock().lines() {
        let line = line_res?;
        let fields: Vec<&str> = line.split('\t').collect();
        assert_eq!(
            fields.len(),
            2,
            "Expected a line in the format <COMMAND> query."
        );
        let command = fields[0];
        let query = fields[1].to_owned();
        let count = match command {
            "COUNT" => {
                let result_list = index_arc
                    .search(
                        query,
                        QueryType::Union,
                        0,
                        1,
                        ResultType::Count,
                        false,
                        Vec::new(),
                    )
                    .await;
                result_list.result_count_total
            }

            "TOP_10" => {
                let result_list = index_arc
                    .search(
                        query,
                        QueryType::Union,
                        0,
                        10,
                        ResultType::Topk,
                        false,
                        Vec::new(),
                    )
                    .await;
                let _top_k = result_list.results;
                1
            }
            "TOP_100" => {
                let result_list = index_arc
                    .search(
                        query,
                        QueryType::Union,
                        0,
                        100,
                        ResultType::Topk,
                        false,
                        Vec::new(),
                    )
                    .await;
                let _top_k = result_list.results;
                1
            }
            "TOP_1000" => {
                let result_list = index_arc
                    .search(
                        query,
                        QueryType::Union,
                        0,
                        1000,
                        ResultType::Topk,
                        false,
                        Vec::new(),
                    )
                    .await;
                let _top_k = result_list.results;
                1
            }
            "TOP_1_COUNT" => {
                let result_list = index_arc
                    .search(
                        query,
                        QueryType::Union,
                        0,
                        1,
                        ResultType::TopkCount,
                        false,
                        Vec::new(),
                    )
                    .await;
                let _top_k = result_list.results;
                result_list.result_count_total
            }
            "TOP_5_COUNT" => {
                let result_list = index_arc
                    .search(
                        query,
                        QueryType::Union,
                        0,
                        5,
                        ResultType::TopkCount,
                        false,
                        Vec::new(),
                    )
                    .await;
                let _top_k = result_list.results;
                result_list.result_count_total
            }
            "TOP_10_COUNT" => {
                let result_list = index_arc
                    .search(
                        query,
                        QueryType::Union,
                        0,
                        10,
                        ResultType::TopkCount,
                        false,
                        Vec::new(),
                    )
                    .await;
                let _top_k = result_list.results;
                result_list.result_count_total
            }
            "TOP_100_COUNT" => {
                let result_list = index_arc
                    .search(
                        query,
                        QueryType::Union,
                        0,
                        100,
                        ResultType::TopkCount,
                        false,
                        Vec::new(),
                    )
                    .await;
                let _top_k = result_list.results;
                result_list.result_count_total
            }
            "TOP_1000_COUNT" => {
                let result_list = index_arc
                    .search(
                        query,
                        QueryType::Union,
                        0,
                        1000,
                        ResultType::TopkCount,
                        false,
                        Vec::new(),
                    )
                    .await;
                let _top_k = result_list.results;
                result_list.result_count_total
            }
            _ => {
                println!("UNSUPPORTED");
                continue;
            }
        };
        //println!("{}", count);
        writeln!(handle, "{}", count);
        stdout.flush()?;
    }

    Ok(())
}
