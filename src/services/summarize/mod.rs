use std::collections::BTreeSet;
use std::collections::BTreeMap;
use chrono::{NaiveDate, Datelike};
use crate::{models, services};

pub fn run(file_path: &str) {
    println!("集計処理を実行します。");
    let data = services::io::read_data_or_panic(file_path);
    let target_dates = get_target_dates(&data);
    let mut result_table = BTreeMap::new();
    for target_date in target_dates {
        let filterd_data = get_filterd_data(&data, target_date);
        let sum = summarize_data(&filterd_data);
        result_table.insert(target_date, sum);
    }
    print_table(result_table);
}

fn get_target_dates(data: &Vec<models::Item>) -> BTreeSet<NaiveDate> {
    let target_dates = data.iter().map(|item| item.get_first_day()).collect();
    target_dates
}

fn get_filterd_data(data: &Vec<models::Item>, filter_date: NaiveDate) -> Vec<&models::Item> {
    let filterd_data = data.iter().filter(|item| {
        (item.get_year() == filter_date.year()) && (item.get_month() == filter_date.month())
    }).collect();
    filterd_data
}

fn summarize_data(data: &Vec<&models::Item>) -> i32 {
    let mut sum = 0;
    for item in data {
        sum += item.get_price_for_summary();
    }
    sum
}

fn format_data(data: NaiveDate) -> String {
    format!("{}/{}", data.year(), data.month())
}
fn format_price(price: i32) -> String {
    if price >= 0 {
        format!("+{}", price)
    } else {
        format!("{}", price)
    }
}

fn print_table(result_table: BTreeMap<NaiveDate, i32>) {
    for result in result_table {
        let date:String = format_data(result.0);
        let price:String = format_price(result.1);
        println!("{}の収支は{}円でした", date, price);
    }
}

#[cfg(test)]
mod summarize_test {
    use crate::models;

    fn get_test_data() -> Vec<models::Item> {
        vec![
            super::models::Item::new(
                "給与".to_string(), 
                super::models::Category::Income(super::models::IncomeCategory::Salary),
                300000, 
                super::NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()
            ),
            super::models::Item::new(
                "給与".to_string(), 
                super::models::Category::Income(super::models::IncomeCategory::Salary),
                300000, 
                super::NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()
            ),
        ]
    }

    #[test]
    fn test_get_target_dates() {
        let test_data: Vec<models::Item> = get_test_data();
        let mut expected = std::collections::BTreeSet::new();
        expected.insert(super::NaiveDate::from_ymd_opt(2020, 1, 1).unwrap());
        expected.insert(super::NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
        
        assert_eq!(super::get_target_dates(&test_data), expected);
    }
}