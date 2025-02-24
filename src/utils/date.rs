

use crate::types::date::YearMonth;

//TODO: return result here, or option
pub fn extract_year_month_from_string(date: &str) -> YearMonth {
    let date = date.split("M").collect::<Vec<&str>>();
    let year = date[0].parse::<u32>().unwrap();
    let month = date[1].parse::<u32>().unwrap();
    YearMonth::new(year, month)
}




#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("2025M01", 2025, 1)]
    #[case("2023M11", 2023, 11)]
    #[case("1917M03", 1917, 3)]
    #[case("1658M12", 1658, 12)]
    #[case("22M01", 22, 1)]
    #[case("01M07", 1, 7)]
    fn test_extract_year_month_from_string_has_valid_input_date_from_ssb_data(#[case] date: &str, #[case] year: u32, #[case] month: u32) {
        let year_month = extract_year_month_from_string(date);
        assert_eq!(year_month.year, year);
        assert_eq!(year_month.month, month);
    }

}