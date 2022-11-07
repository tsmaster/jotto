use std::fs;
use std::collections::HashMap;

fn overlap(w_1: &str, w_2: &str) -> bool {
    let b_1 = w_1.as_bytes();
    let b_2 = w_2.as_bytes();
    
    for idx_1 in 0 .. 5 {
	for idx_2 in 0 .. 5 {
	    if b_1[idx_1] == b_2[idx_2] {
		return true;
	    }
	}
    }
    return false;
}

fn word_to_int(w: &str) -> u32 {
    let mut i:u32 = 0;
    let bytes = w.as_bytes();
    
    for idx in 0 .. 5 {
	let c = bytes[idx] - 65;
	i = i | ((1 as u32) << c);
    }
    i
}

fn overlap_int(w_i_1: u32, w_i_2: u32) -> bool {
    (w_i_1 & w_i_2) != 0
}

/*
fn main() {
    println!("Hello, world!");

    let wordlist = fs::read_to_string("Data/merged.txt")
	.expect("failed to read file");

    let mut words_vec = vec!();
    
    for w in wordlist.split_whitespace() {
	//println!("word: {}", w);
	words_vec.push(w);
    }

    let num_words = words_vec.len();
    println!("found {} words", num_words);

    for a in 0..num_words {
	let w_1 = words_vec[a];
	for b in a+1 .. num_words {
	    let w_2 = words_vec[b];
	    if overlap(w_1, w_2) {
		continue;
	    }
	    for c in b+1 .. num_words {
		let w_3 = words_vec[c];
		if overlap(w_1, w_3) || overlap(w_2, w_3) {
		    continue;
		}
		for d in c+1 .. num_words {
		    let w_4 = words_vec[d];
		    if overlap(w_1, w_4) || overlap(w_2, w_4) || overlap(w_3, w_4) {
			continue;
		    }
		    for e in d+1 .. num_words {
			let w_5 = words_vec[e];
			if overlap(w_1, w_5) || overlap(w_2, w_5) || overlap(w_3, w_5) || overlap(w_4, w_5) {
			    continue;
			}
			
			println!("soln: {} {} {} {} {}", w_1, w_2, w_3, w_4, w_5);
			
		    }
		}
	    }
	}
    }
}
 */

fn main() {
    let wordlist = fs::read_to_string("Data/merged.txt")
	.expect("failed to read file");

    let mut words_vec = vec!();
    
    for w in wordlist.split_whitespace() {
	//println!("word: {}", w);
	words_vec.push(w);
    }

    let mut ints_vec = vec!();

    for w in &words_vec {
	let word_int = word_to_int(w);
	ints_vec.push(word_int);
    }

    let mut successors = HashMap::new();

    println!("making table");
    for word_index in 0 .. words_vec.len() {
	let word_int = ints_vec[word_index];

	let mut succ_vec = vec!();
	
	for succ_index in word_index + 1 .. words_vec.len() {
	    let succ_int = ints_vec[succ_index];

	    if !overlap_int(word_int, succ_int) {
		/*println!("no overlap from idx {} word {} and idx {} word {}",
			 word_index, words_vec[word_index],
			 succ_index, words_vec[succ_index]);*/
		succ_vec.push(succ_index);
	    }
	}

	successors.insert(word_index, succ_vec);
    }
    println!("table complete");

    let num_words = words_vec.len();
    println!("found {} words", num_words);

    for a in 0..num_words {
	let w_1 = words_vec[a];
	let w_i_1 = ints_vec[a];
	
	for b in &successors[&a] {
	    let w_2 = words_vec[*b as usize];
	    let w_i_2 = ints_vec[*b as usize];

	    for c in &successors[&b] {
		let w_3 = words_vec[*c as usize];
		let w_i_3 = ints_vec[*c as usize];
		
		if overlap_int(w_i_1, w_i_3) {
		    continue;
		}
		
		for d in &successors[&c] {
		    let w_4 = words_vec[*d as usize];
		    let w_i_4 = ints_vec[*d as usize];
		    
		    if overlap_int(w_i_1, w_i_4) || overlap_int(w_i_2, w_i_4) {
			continue;
		    }
		    for e in &successors[&d] {
			let w_5 = words_vec[*e as usize];
			let w_i_5 = ints_vec[*e as usize];
			if overlap_int(w_i_1, w_i_5) || overlap_int(w_i_2, w_i_5) || overlap_int(w_i_3, w_i_5) {
			    continue;
			}
			
			println!("soln: {} {} {} {} {}", w_1, w_2, w_3, w_4, w_5);
			
		    }
		}
	    }
	}
    }
}
