pub mod csv_reader {
    use std::error::Error;
    use std::fs::File;
    use std::path::Path;
    
    pub fn read_csv<P: AsRef<Path>>(
        filename: P,
    ) -> Result<(Vec<String>, Vec<f64>, Vec<f64>, Vec<f64>), Box<dyn Error>> {
        let file: File = File::open(filename)?;
        let mut rdr: csv::Reader<File> = csv::Reader::from_reader(file);

        let mut name_col: Vec<String> = Vec::new();
        let mut price_col: Vec<f64> = Vec::new();
        let mut profit_col: Vec<f64> = Vec::new();
        let mut earnings_col: Vec<f64> = Vec::new();

        for result in rdr.records() {
            let record: csv::StringRecord = result?;
            let price: f64 = record[1].parse::<f64>()?;
            let profit: f64 = record[2].parse::<f64>()?;
            if price > 0.0 {
                name_col.push(record[0].to_string());
                price_col.push(price);
                profit_col.push(profit);
                earnings_col.push(price * (profit / 100.0));
            }
        }
        Ok((name_col, price_col, profit_col, earnings_col))
    }
}
