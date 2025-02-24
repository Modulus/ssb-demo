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

    #[test]
    fn test_extract_year_month_from_string_has_valid_input_date_from_ssb_data() {
        let date = "2025M01";
        let year_month = extract_year_month_from_string(date);
        assert_eq!(year_month.year, 2025);
        assert_eq!(year_month.month, 1);
    }

}