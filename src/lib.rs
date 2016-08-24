pub struct IntSorter {
  data : Vec<i64>,
  sorted: bool,
}

impl IntSorter {
  fn new() -> IntSorter {
    IntSorter{ data: vec![], sorted: true }
  }

  fn put(&mut self, i : i64) {
    self.data.push(i);
  }

  fn get(&mut self) -> i64 {
    self.data.pop().unwrap()
  }
}

#[no_mangle]
pub extern "C" fn sorter_new() -> Box<IntSorter> {
  Box::new( IntSorter::new() )
}

#[no_mangle]
pub extern "C" fn sorter_cleanup(s : Box<IntSorter>) {
  println!("Destroy IntSorter Instance");
  // Explicit drop removes warning
  drop(s);
}


#[no_mangle]
pub extern "C" fn sorter_put(s : &mut IntSorter, i : i64) {
  s.put(i);
  s.sorted = false;
}


#[no_mangle]
pub extern "C" fn sorter_get(s : &mut IntSorter) -> i64 {
  if s.sorted == false {
    s.data.sort();
    s.sorted = true;
  }
  s.get()
}

