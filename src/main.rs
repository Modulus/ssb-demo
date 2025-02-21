use reqwest::blocking::get;
use polars::prelude::*;
use ssb_demo::CsvUrlConverter;
use ssb_demo::CsvConverter;
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


    let url = "https://data.ssb.no/api/v0/dataset/1088.csv?lang=no";
    let converter = CsvUrlConverter{};
    let df = converter.convert(url);
    println!("{:?}", df);
}
