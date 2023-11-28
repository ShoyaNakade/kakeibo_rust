use crate::models;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_data_or_create_new_data(file_path: &str) -> Vec<models::Item> {
    let file = File::open(file_path);
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).expect("ファイルの読み込みに失敗しました。")
        }
        Err(_) => {
            println!("ファイルが存在しないため、新規作成します。");
            Vec::new()
        }
    }
}

pub fn read_data_or_panic(file_path: &str) -> Vec<models::Item> {
    let file = File::open(file_path).expect("ファイルの読み込みに失敗しました。");
    let reader = BufReader::new(file);
    let data: Vec<_> = serde_json::from_reader(reader).expect("デシリアライズに失敗しました。");

    if data.len() == 0 {
        panic!("データが存在しません。");
    }

    data 
}

pub fn write_to_jason(data: &Vec<models::Item>,file_path: &str, ) {
    let json_data = serde_json::to_string_pretty(data).expect("JSONへのシリアライズに失敗しました。");
    let mut file = File::create(file_path).expect("書き込みファイルのオープンにに失敗しました。");
    writeln!(file, "{}", json_data).expect("ファイルへの書き込みに失敗しました。");
    println!("ファイルへの書き込みが完了しました。");
}