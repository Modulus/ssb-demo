use polars::frame::DataFrame;
use reqwest::blocking::get;
use polars::prelude::*;
use std::io::Cursor;

pub trait CsvConverter {
    fn convert(&self, target: &str) -> DataFrame;

}

fn get_default_parse_options() -> CsvParseOptions {
    CsvParseOptions::default().with_try_parse_dates(true).with_separator(";".as_bytes()[0]).with_decimal_comma(true)
}

pub struct CsvUrlConverter {
  
}


impl CsvConverter for CsvUrlConverter {
    fn convert(&self, target: &str) -> DataFrame {
        let body = get(target).unwrap().text().unwrap();
        let reader = Cursor::new(body);
    
        let df = CsvReadOptions::default()
        .with_infer_schema_length(None)
        .with_has_header(true)
        .with_parse_options(get_default_parse_options())
        .into_reader_with_file_handle(reader).finish().unwrap();
        df
    }
 
}

