use polars::frame::DataFrame;
use reqwest::blocking::get;
use polars::prelude::*;
use std::io::Cursor;

pub trait CsvConverter {
    fn convert(&self, target: &str) -> PolarsResult<DataFrame>;

}

fn get_default_parse_options() -> CsvParseOptions {
    CsvParseOptions::default().with_try_parse_dates(true).with_separator(";".as_bytes()[0]).with_decimal_comma(true)
}

pub struct CsvUrlConverter {
  
}

pub struct CsvFileConverter {
  
}


impl CsvConverter for CsvUrlConverter {
    fn convert(&self, target: &str) -> PolarsResult<DataFrame> {
        let body = get(target).unwrap().text().unwrap();
        let reader = Cursor::new(body);
    
        let df = CsvReadOptions::default()
        .with_infer_schema_length(None)
        .with_has_header(true)
        .with_parse_options(get_default_parse_options())
        .into_reader_with_file_handle(reader).finish()?;
        Ok(df)
    }
 
}

impl CsvConverter for CsvFileConverter {
    fn convert(&self, target: &str) -> PolarsResult<DataFrame> {
        let df = CsvReadOptions::default()
        .with_infer_schema_length(None)
        .with_has_header(true)
        .with_parse_options(get_default_parse_options())
        .try_into_reader_with_file_path(Some(target.into())).unwrap()
        .finish()?;
    
        Ok(df)
    }
 
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_default_parse_options() {
        let options = get_default_parse_options();
        assert!(
            options.try_parse_dates,
            "try_parse_dates should be set to true"
        );
        assert_eq!(
            options.separator,
            b';',
            "separator should be the semicolon character"
        );
        assert!(
            options.decimal_comma,
            "decimal_comma should be enabled"
        );
    }

    #[test]
    fn test_convert_has_local_file_returns_content(){
        let file = "resources/file.csv";
        let converter = CsvFileConverter{};


        let result = converter.convert(file);

        assert!(result.is_ok());

        let df = result.unwrap();

        assert!(df.size() >= 6);
        assert_eq!(df.shape(), (6,4));

    }
}