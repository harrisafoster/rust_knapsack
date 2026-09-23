pub mod tools {
    use crate::reader::csv_reader::{ read_csv, CsvColumns };
    use crate::structs::structs::Stock;

    pub fn float64_max(v1: f64, v2: f64) -> f64 {
        if v1 > v2 { v1 } else { v2 }
    }

    pub fn set_up_and_sort_data(csv_data: &[u8]) -> Vec<Stock> {
        let res: CsvColumns = read_csv(csv_data).expect("Failed to read embedded CSV data");

        let mut stocks: Vec<Stock> = Vec::with_capacity(res.0.len());

        for i in 0..res.0.len() {
            stocks.push(Stock {
                name: res.0[i].clone(),
                price: res.1[i],
                profit: res.2[i],
                earnings: res.3[i],
            });
        }

        stocks.sort_by(|a: &Stock, b: &Stock| {
            b.profit.partial_cmp(&a.profit).unwrap_or(std::cmp::Ordering::Equal)
        });

        stocks
    }
}
