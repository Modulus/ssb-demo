use ssb_demo::CsvFileConverter;
use ssb_demo::CsvUrlConverter;
use ssb_demo::CsvConverter;
mod utils;
mod types;

use utils::date::extract_year_month_from_string;
use types::ssb::DataPointKPI;

fn main() {
    println!("Hello, world!");

    let file = "resources/file.csv";
    let converter = CsvFileConverter{};
    let df = converter.convert(file);

    println!("{:?}", df.unwrap());


    let url = "https://data.ssb.no/api/v0/dataset/1088.csv?lang=no";
    let converter = CsvUrlConverter{};
    let df = converter.convert(url);
    println!("{:?}", df.unwrap());

    

    // let datapoints : Vec<DataPointKPI> = df.iter().for_each(|s| DataPointKPI::new(s)).collect();
}
