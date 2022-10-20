use rand::Rng;
use std::collections::HashMap;
use std::collections::*;
use std::fmt::Result;
use std::io::Result as IoResult;
use std::io::{self, Write};
use std::{cmp::Ordering, io};

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    let secret_number = rand::thread_rng().gen_range(1..=100);
}

// fn function1() -> Result {}

// fn function2() -> IoResult<()> {}
