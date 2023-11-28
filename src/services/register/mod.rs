use std::io;
use crate::services;
use crate::models;
use chrono::NaiveDate;
use std::str::FromStr;

pub fn run(file_path: &str) {
    println!("登録処理を実行します。");
    let register_type = input_register_type();
    let name = input_name();
    let category_type = input_category_type(register_type);
    let price = input_price();
    let date = input_date();
    let category = models::Item::get_category(register_type, category_type);

    let item = models::Item::new(name, category, price, date);
    println!("{:?}", item);

    let mut data = services::io::read_data_or_create_new_data(file_path);
    data.push(item);
    services::io::write_to_jason(&data, file_path);
}

fn input_register_type() -> u8 {
    println!("登録したい内容を入力してください。（０：収入、１：支出）");
    let mut register_type = String::new();
    io::stdin().read_line(&mut register_type).expect("登録種別の読み込みに失敗しました。");
    let register_type: u8 = register_type.trim().parse().expect("登録種別は数字を入力してください。");
    services::validate::InputValidator::validate_register_type(register_type);
    register_type
}

fn input_name() -> String {
    println!("登録したい名前を入力してください。");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("名前の読み込みに失敗しました。");
    name.trim().to_string()
}

fn input_category_type(register_type: u8) -> u8 {
    println!("カテゴリーを入力してください");
    if register_type == 0 {
        println!("（０：給与、１：一時所得、２：その他）");
    } else if register_type == 1 {
        println!("（０：食費、１：日用品、２：その他）");
    }
    let mut category_type = String::new();
    io::stdin().read_line(&mut category_type).expect("カテゴリーの読み込みに失敗しました。");
    let category_type: u8 = category_type.trim().parse().expect("カテゴリーは数字を入力してください。");
    services::validate::InputValidator::validate_category_type(register_type,category_type);
    category_type
}

fn input_price() -> u32 {
    println!("金額を入力してください。");
    let mut price = String::new();
    io::stdin().read_line(&mut price).expect("金額の読み込みに失敗しました。");
    let price: u32 = price.trim().parse().expect("金額は数字を入力してください。");
    price
}

fn input_date() -> NaiveDate {
    println!("日付を入力してください。(yyyy-mm-dd)");
    let mut date = String::new();
    io::stdin().read_line(&mut date).expect("日付の読み込みに失敗しました。");
    NaiveDate::from_str(&date).expect("日付はyyyy-mm-ddの形式で入力してください。")
}