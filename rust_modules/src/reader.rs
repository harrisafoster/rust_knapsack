pub mod csv_reader {
    use std::error::Error;
    use std::io::Cursor;

    pub type CsvColumns = (Vec<String>, Vec<f64>, Vec<f64>, Vec<f64>);

    pub fn read_csv(csv_data: &[u8]) -> Result<CsvColumns, Box<dyn Error>> {
        let cursor = Cursor::new(csv_data);
        let mut rdr = csv::Reader::from_reader(cursor);

        let mut name_col: Vec<String> = Vec::new();
        let mut price_col: Vec<f64> = Vec::new();
        let mut profit_col: Vec<f64> = Vec::new();
        let mut earnings_col: Vec<f64> = Vec::new();

        for result in rdr.records() {
            let record: csv::StringRecord = result?;

            let name = record.get(0).ok_or("Missing stock name")?;

            let price: f64 = record.get(1).ok_or("Missing stock price")?.parse()?;

            let profit: f64 = record.get(2).ok_or("Missing stock profit")?.parse()?;

            if price > 0.0 {
                name_col.push(name.to_owned());
                price_col.push(price);
                profit_col.push(profit);
                earnings_col.push(price * (profit / 100.0));
            }
        }

        Ok((name_col, price_col, profit_col, earnings_col))
    }
}
