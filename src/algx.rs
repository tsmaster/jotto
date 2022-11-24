use std::collections::HashSet;

struct AlgXColumn
{
    is_hidden: bool,
    row_indices: Vec<usize>,
    count: usize
}

impl AlgXColumn
{
    fn new() -> AlgXColumn {
	AlgXColumn {
	    is_hidden : false,
	    row_indices: vec!(),
	    count: 0
	}
    }

    fn hide(&mut self) {
	self.is_hidden = true;
    }

    fn unhide(&mut self) {
	self.is_hidden = false;
    }

    fn add_row(&mut self, row_index: usize) {
	self.row_indices.push(row_index);
	self.count = self.row_indices.len();
    }

    fn print(&self, row_keys:&Vec<String>) {
	for i in 0..self.count {
	    let idx = self.row_indices[i];
	    print!("{} ", row_keys[idx]);
	}
    }

    fn get_active_rows(&self) -> Vec<usize> {
	// todo return a slice?
	return self.row_indices[0..self.count].to_vec();
    }

    fn hide_value(&mut self, value: usize) {
	// find index of value
	let found_idx = self.row_indices.iter().position(|x| *x == value).unwrap();

	// if unhidden, swap with last value (unless last value is 0)
	if found_idx <= self.count - 1 {
	    let temp = self.row_indices[found_idx];
	    self.row_indices[found_idx] = self.row_indices[self.count - 1];
	    self.row_indices[self.count - 1] = temp;

	    // update count
	    self.count = self.count - 1;
	}
    }

    fn unhide_value(&mut self, value: usize) {
	// find index of value
	let found_idx = self.row_indices.iter().position(|x| *x == value).unwrap();

	// if hidden, swap with first hidden value (unless no hidden values)
	if found_idx >= self.count {
	    let temp = self.row_indices[found_idx];
	    self.row_indices[found_idx] = self.row_indices[self.count];
	    self.row_indices[self.count] = temp;

	    // update count
	    self.count = self.count + 1;
	}
    }
}

pub struct AlgXGridBuilder
{
    col_keys: Vec<String>,
    row_keys: Vec<String>,
    rows: Vec<Vec<usize>>,
    columns: Vec<AlgXColumn>,
}


impl AlgXGridBuilder {
    pub fn new(col_keys: Vec<String>) -> AlgXGridBuilder {
	AlgXGridBuilder {
	    col_keys: col_keys,
	    row_keys: vec!(),
	    rows: vec!(),
	    columns: vec!(),
	}
    }

    pub fn add_row(&mut self, row_key: String, cols: Vec<String>) {
	//println!("adding row key {}, row {:?}", row_key, cols);
	let mut row_data = vec!();

	for col_key in &cols {
	    let col_index = self.col_keys.iter().position(|x| x == col_key).unwrap();
	    row_data.push(col_index);
	}
	
	self.rows.push(row_data);
	self.row_keys.push(row_key);
    }

    pub fn add_row_str(&mut self, row_key: String, cols: Vec<&str>) {
	let mut row_data = vec!();
	for col_str in cols {
	    row_data.push(col_str.to_string());
	}
	self.add_row(row_key, row_data);
    }

    pub fn build(&self) -> AlgX {
	println!("building matrix");
	let mut col_vals = vec!();

	for _i in 0 .. self.col_keys.len() {
	    col_vals.push(AlgXColumn::new());
	}

	for (row_index, r) in self.rows.iter().enumerate() {
	    //println!("ri {} rk {} r {:?}", row_index, self.row_keys[row_index], r);
	    
	    for c_v in r {
		//println!("column value {}", c_v);
		//println!("column key {}", self.col_keys[*c_v]);

		col_vals[*c_v].add_row(row_index);
	    }
	}

	AlgX {
	    rows: self.rows.to_vec(),
	    cols: col_vals,

	    row_keys: self.row_keys.to_vec(),
	    col_keys: self.col_keys.to_vec(),
	}
    }
}



pub struct AlgX {
    rows: Vec<Vec<usize>>,
    cols: Vec<AlgXColumn>,

    row_keys: Vec<String>,
    col_keys: Vec<String>
}

impl AlgX {
    fn has_any_columns(&self) -> bool {
	for column in &self.cols {
	    if column.is_hidden {
		continue;
	    }
	    /*
	    if column.count > 0 {
		return true;
	}*/
	    return true;
	}
	return false;
    }
    
    fn print(&self) {
	let mut remaining_row_indices = HashSet::new();
	for column in &self.cols {
	    if column.is_hidden {
		continue;
	    }
	    for row_index in &column.get_active_rows() {
		remaining_row_indices.insert(*row_index);
	    }
	}

	let mut remaining_row_indices_vec:Vec<usize> = remaining_row_indices.into_iter().collect();
	remaining_row_indices_vec.sort();
	
	println!("rows");
	for row_idx in remaining_row_indices_vec {
	    print!(" {} - ", self.row_keys[row_idx]);
	    for col_idx in &self.rows[row_idx] {
		print!("{} ", self.col_keys[*col_idx]);
	    }
	    println!("");
	}
	println!("");
	println!("cols");
	for (c_index, column) in self.cols.iter().enumerate() {
	    if column.is_hidden {
		continue;
	    }
	    print!(" {} - ", self.col_keys[c_index]);
	    self.cols[c_index].print(&self.row_keys);
	    println!("");
	}	
    }
    
    fn solve_rec(&mut self, soln: &mut Vec<usize>) {
	//self.print();
	
	if !self.has_any_columns() {
	    println!("found solution {:?}", soln);

	    for row_index in soln {
		println!("row {}", self.row_keys[*row_index]);
	    }
	    return;
	}

	let mut best_col_idx:Option<usize> = None;
	let mut best_col_len:usize = 0;

	for (col_idx, column) in self.cols.iter().enumerate() {
	    if column.is_hidden {
		continue;
	    }
	    
	    if (best_col_idx.is_none()) ||
		(column.count < best_col_len) {
		    best_col_idx = Some(col_idx);
		    best_col_len = column.count;
		}
	}

	let selected_col_idx = best_col_idx.unwrap();
	let selected_col_key = self.col_keys[selected_col_idx].to_string();

	//println!("choosing column key {}", selected_col_key);

	let count = self.cols[selected_col_idx].count;
	
	for counter in 0..count {
	    let row_index = self.cols[selected_col_idx].row_indices[counter];
	    let row_key = &self.row_keys[row_index];
	    //println!("adding row {} to candidate soln", row_key);
	    soln.push(row_index);

	    let cols = self.select(row_index);
	    self.solve_rec(soln);
	    self.deselect(row_index, cols);
	    soln.pop();
	}
    }

    pub fn solve(&mut self) {
	let mut solns = vec!();

	self.solve_rec(&mut solns);
    }

    fn select(&mut self, row_index: usize) -> Vec<usize> {
	//println!("selecting row key: {}", self.row_keys[row_index]);
	let mut cols = vec!();

	for col_j_index in &self.rows[row_index] { // for columns specified by this row
	    //println!("removing all rows containing column key: {}", self.col_keys[*col_j_index]);

	    for row_i_index in &self.cols[*col_j_index].get_active_rows() { // i is a row key
		//println!("removing row {}", self.row_keys[*row_i_index]);

		for col_k_index in &self.rows[*row_i_index] { // k is a column index used by this row
		    //println!("hiding rows in column {}", self.col_keys[*col_k_index]);

		    if col_k_index != col_j_index {
			/*
			println!("want to remove key {} from column key {}",
				 self.row_keys[*row_i_index],
				 self.col_keys[*col_k_index]);*/

			let col = &mut self.cols[*col_k_index];

			/*
			print!("column to remove from: ");
			col.print(&self.row_keys);
			println!();*/

			col.hide_value(*row_i_index);

			/*
			print!("column after remove: ");
			col.print(&self.row_keys);
			println!();*/
		    }
		}
	    }

	    cols.push(*col_j_index);
	    self.cols[*col_j_index].hide();
	}

	cols
    }

    fn deselect(&mut self, row_idx: usize, mut columns: Vec<usize>) {
	//println!("deselecting row key {}", self.row_keys[row_idx]);

	let mut row = self.rows[row_idx].to_vec();
	row.reverse();
	
	for j_col_index in row {
	    let popped_idx = columns.pop().unwrap();
	    self.cols[popped_idx].unhide();

	    //println!("inserted column for {}", self.col_keys[j_col_index]);

	    for row_i_idx in self.cols[j_col_index].get_active_rows() {
		for col_k_idx in &self.rows[row_i_idx] {
		    if *col_k_idx != j_col_index {
			self.cols[*col_k_idx].unhide_value(row_i_idx);
		    }
		}
	    }
	}
    }
}
