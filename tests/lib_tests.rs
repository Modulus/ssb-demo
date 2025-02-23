use ssb_demo::{CsvConverter, CsvUrlConverter};



#[test]
fn test_verify_convert_with_online_file_from_ssb(){
    let url = "https://data.ssb.no/api/v0/dataset/1088.csv?lang=no";
    let converter = CsvUrlConverter{};
    let result = converter.convert(url);

    assert!(result.is_ok());

    let df = result.unwrap();

    assert_eq!(df.size(), 24);
    assert_eq!(df.shape(), (6, 4));

}