use polars::frame::DataFrame;
use reqwest::blocking::get;
use polars::prelude::*;
use std::io::Cursor;

mod utils;
mod types;

use utils::date::extract_year_month_from_string;

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

        assert_eq!(df.size(), 24);
        assert_eq!(df.shape(), (6,4));

    }

    #[test]
    fn test_expected_contents_of_dataframe_from_csv_file(){
        let file = "resources/file.csv";
        let converter = CsvFileConverter{};
        let df = converter.convert(file).expect("Failed to convert CSV file");
    
        let expected_df = df![
            "konsumgruppe" => &["TOTAL Totalindeks", "TOTAL Totalindeks", "TOTAL Totalindeks", "TOTAL Totalindeks", "TOTAL Totalindeks", "TOTAL Totalindeks"],
            "måned" => &["2024M12", "2024M12", "2024M12", "2025M01", "2025M01", "2025M01"],
            "statistikkvariabel" => &["Konsumprisindeks (2015=100)", "Månedsendring (prosent)", "12-måneders endring (prosent)", "Konsumprisindeks (2015=100)", "Månedsendring (prosent)", "12-måneders endring (prosent)"],
            "03013: Konsumprisindeks, etter konsumgruppe, måned og statistikkvariabel" => &[134.8, -0.1, 2.2, 135.1, 0.2, 2.3]
        ].unwrap();

    
        assert_eq!(df, expected_df);
    }

}