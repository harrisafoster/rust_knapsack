pub mod tools {
    use crate::reader::csv_reader::read_csv;
    use crate::structs::structs::Stock;

    pub fn float64_max(v1: f64, v2: f64) -> f64 {
        if v1 > v2 {
            return v1;
        } else {
            return v2;
        }
    }

    pub fn set_up_and_sort_data(filename: &str) -> Vec<Stock> {
        let res: (Vec<String>, Vec<f64>, Vec<f64>, Vec<f64>) = read_csv(filename).unwrap();
        let mut stocks: Vec<Stock> = Vec::new();

        let num_rows: usize = res.0.len();
        for i in 0..num_rows {
            stocks.push(Stock {
                name: res.0[i].clone(),
                price: res.1[i],
                profit: res.2[i],
                earnings: res.3[i],
            });
        }
        stocks.sort_by(|a: &Stock, b: &Stock| {
            b.profit
                .partial_cmp(&a.profit)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        stocks
    }
}
