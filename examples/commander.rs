use parselnk;
use std::convert::TryFrom;
use std::path::Path;

fn main() {
    let lnk = parselnk::Lnk::try_from(Path::new("./test_data/commander.lnk")).unwrap();
    println!("-> {:#?}", lnk);
}
