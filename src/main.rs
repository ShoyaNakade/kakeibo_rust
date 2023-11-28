use std::io;
use kakeibo_rust::services;

const FILE_PATH: &str = "store/data.json";
fn main() {
    let mut service_type = String::new();
    println!("実行したい内容を入力してください。（０：登録、１：集計）");
    io::stdin().read_line(&mut service_type).unwrap();
    let service_type: u8 = service_type.trim().parse().expect("数字を入力してください。");
    services::validate::InputValidator::validate_service_type(service_type);

    if service_type == 0 {
        services::register::run(FILE_PATH);
    } else if service_type == 1 {
        services::summarize::run(FILE_PATH);
    }
}
