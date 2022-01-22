pub mod fasta_to_hash_values;
pub mod analyze_queries;

use std::env;

use std::collections::HashMap;

// crates for measuring time and memory usage
use std::time::Instant;
use std::mem::MaybeUninit;

use seq_io::fasta::{Reader, Record};

use fasta_to_hash_values::*;
use analyze_queries::match_query_by_common_minimizers;


/// Takes in 5 parameters: reference file, query file, k, l, and min_count
/// Matches the minimizers of query sequences to the reference sequences and assigns the query sequence to 
/// the reference sequence with the most hits if that number is more than the min_count
/// 
fn main() {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let queryfile = &args[2];
    let k:i32 = args[3].parse().unwrap();
    let l:i32 = args[4].parse().unwrap();
    let min_count:i32 = args[5].parse().unwrap();
    println!("The file name is {} and the query file name is {}", filename, queryfile);
    
    let map:HashMap<String, Vec<u64>> = create_dictionary(filename, k, l);
    //println!{"{:?}", map};
    println!("map created");

    let results = simple_hash_query(queryfile, map, k, l, min_count);
    println!("{:?}", results);

    println!("Execution time is {:?}", now.elapsed());
    println!("Maximum RSS: {:?}GB", (get_memory_rusage() as f32) / 1024.0 / 1024.0/ 1024.0);
}

/// Function to get memory ussage
fn get_memory_rusage() -> usize {
    let usage = unsafe{
        let mut usage = MaybeUninit::uninit();
        assert_eq!(libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()), 0);
        usage.assume_init()
    };
    usage.ru_maxrss as usize * 1024
}

/// Reads the query file and creates a Dictionary for each query sequence, recording the number of hits to each reference genome
fn simple_hash_query(filename: &String, references:HashMap<String, Vec<u64>>, k:i32, l:i32, min_count:i32) -> HashMap<String, String> {
    let mut reader = Reader::from_path(filename).unwrap();
    let mut results:HashMap<String, String> = HashMap::new();
    while let Some(record) = reader.next() {
        let record = record.expect("Error reading record");
        let query = (record.id().unwrap().to_string(), 
        create_minimizers(k, l, record.seq().to_vec()));
        let result = match_query_by_common_minimizers(query, &references, min_count);
        results.insert(result.0, result.1);
    }
    results
}
