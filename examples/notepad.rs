use parselnk;

fn main() {
    let lnk_data = std::include_bytes!("../test_data/notepad.lnk");
    let lnk = parselnk::Lnk::new(&mut &lnk_data[0..]);
    println!("-> {:#?}", lnk);
}
