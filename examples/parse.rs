use parselnk;
use std::{convert::TryFrom, path::Path};

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Please provide a lnk to parse");
    let lnk = parselnk::Lnk::try_from(Path::new(&path))
        .map_err(|e| e.to_string())
        .expect("Could not parse lnk: ");

    println!("-> {:#?}", lnk);
}
