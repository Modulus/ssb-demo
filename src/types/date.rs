#[derive(Debug, Clone, Copy)]
pub  struct YearMonth {
    pub year: u32,
    pub month: u32,
}


impl YearMonth {
    pub fn new(year: u32, month: u32) -> Self {
        YearMonth {
            year,
            month,
        }
    }
}