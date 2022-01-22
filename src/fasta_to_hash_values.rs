

use seq_io::fasta::{Reader, Record};

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn create_dictionary(filename: &String, k:i32, l:i32) -> HashMap<String, Vec<u64>> {
    let mut reader = Reader::from_path(filename).unwrap();
    //let records: Result<Vec<_>, _> = reader.records().collect();
    let mut maps:HashMap<String, Vec<u64>> = HashMap::new();
    while let Some(record) = reader.next() {
        let record = record.expect("Error reading record");
        //println!("id: {}, seq: {:?}", record.id().unwrap(), record.seq());
        maps.insert(record.id().unwrap().to_owned(), create_minimizers(k, l, record.seq().to_vec()));
    }
    maps
}

pub fn create_minimizers(k:i32, l:i32, seq:Vec<u8>) -> Vec<u64> {
    let mut n = 0;
    let mut minimizers:Vec<u64> = Vec::new();
    let mut windows = seq.windows(l as usize);
    while n < (*&seq.len() as i32) -l +1 {
        minimizers.push(find_min(windows.next().unwrap(), k));
        n += 1;
    }
    minimizers
}

fn find_min(seq: &[u8], k:i32)-> u64 {
    let mut windows = seq.windows(k as usize);
    let mut minimizer = windows.next().unwrap();
    //println!("The sequence is {:?} with length {}", &seq, seq.len().to_string());
    //println!("{:?} has hash value {}", &minimizer, calculate_hash(&minimizer).to_string());
    for win in windows {
        //println!("{:?} has hash value {}", &win, calculate_hash(&win).to_string());
        if calculate_hash(&minimizer) > calculate_hash(&win){
            minimizer = win;
        }
    }
    calculate_hash(&minimizer)
}

fn calculate_hash<T: Hash>(t:&T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}