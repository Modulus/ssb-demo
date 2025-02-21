use reqwest::blocking::get;
use polars::prelude::*;
use std::io::Cursor;

fn main() {
    println!("Hello, world!");

    let df = CsvReadOptions::default()
    .with_infer_schema_length(None)
    .with_has_header(true)
    .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true).with_separator(";".as_bytes()[0]).with_decimal_comma(true))
    .try_into_reader_with_file_path(Some("resources/file.csv".into())).unwrap()
    .finish().unwrap();

    println!("{:?}", df);


}
