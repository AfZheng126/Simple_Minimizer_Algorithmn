use std::collections::HashMap;

use array_tool::vec::Intersect;

pub fn match_query_by_common_minimizers(query: (String, Vec<u64>), 
    references: &HashMap<String, Vec<u64>>, min_count:i32) -> (String, String) {
    let mut counter = HashMap::new();
    for seq in references{
        let sequence = seq.1.to_owned();
        //println!("{:?} are the minimizers and are the sequences", query);
        let matching = query.1.intersect(sequence);
        counter.insert(seq.0, matching.len());
        //println!("sequence is {} and there are {} number of hits", seq.0, matching.len());
    }
    let result = counter.iter().max_by_key(|entry | entry.1).unwrap().clone();
    let mut id = result.0.to_owned().to_owned();
    id.push_str(" with hits: ");
    id.push_str(&(*result.1 as i32).to_string());
    let final_result = if (*result.1 as i32) >= min_count {id} else {String::from("None")};
    (query.0, final_result)
}
