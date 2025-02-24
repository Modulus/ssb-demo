use polars::frame::DataFrame;

pub struct DataPointKPI {
    pub consume_group: String,
    pub year_month: String,
    pub var_name: String,
    pub value: f64,
}

impl DataPointKPI {
    pub fn new(df: &DataFrame) -> Self {
        let data = df.select(["konsumgruppe", "måned", "statistikkvariabel", "03013: Konsumprisindeks, etter konsumgruppe, måned og statistikkvariabel"]).unwrap();
        DataPointKPI {
            consume_group: String::from(""),
            year_month: String::from(""),
            var_name: String::from(""),
            value: 0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_datapoint_kpi() {
        // let dp = DataPointKPI::new("TOTAL Totalindeks", "2024M12", "Konsumprisindeks (2015=100)", 134.8);
        // assert_eq!(dp.consume_group, "TOTAL Totalindeks");
        // assert_eq!(dp.year_month, "2024M12");
        // assert_eq!(dp.var_name, "Konsumprisindeks (2015=100)");
        // assert_eq!(dp.value, 134.8);
    }
}



