pub mod knapsack {
    use crate::reconstruction::reconstruction::reconstruct_chosen_items;
    use crate::structs::structs::Stock;
    use crate::tools::tools::float64_max;
    use crate::tools::tools::set_up_and_sort_data;

    fn check_all_combinations(
        combination_matrix: &mut Vec<Vec<f64>>,
        prices: &Vec<f64>,
        earnings: &Vec<f64>,
        rmb: usize,
        index: usize,
    ) -> f64 {
        if index == 0 || rmb == 0 {
            return 0.0;
        }
        if combination_matrix[index][rmb] != -1.0 {
            return combination_matrix[index][rmb];
        }
        if (prices[index - 1] * 100.0) as usize <= rmb {
            combination_matrix[index][rmb] = float64_max(
                earnings[index - 1]
                    + check_all_combinations(
                        combination_matrix,
                        prices,
                        earnings,
                        rmb - ((prices[index - 1] * 100.0) as usize),
                        index - 1,
                    ),
                check_all_combinations(combination_matrix, prices, earnings, rmb, index - 1),
            );
            return combination_matrix[index][rmb];
        } else {
            combination_matrix[index][rmb] =
                check_all_combinations(combination_matrix, prices, earnings, rmb, index - 1);
            return combination_matrix[index][rmb];
        }
    }

    pub fn run_algo(capacity: i64, filename: &str) -> Vec<Stock> {
        let stocks: Vec<Stock> = set_up_and_sort_data(filename);
        let n_stocks: usize = stocks.len();
        let rmb: usize = (capacity * 100) as usize;
        let mut combination_matrix: Vec<Vec<f64>> = Vec::with_capacity(n_stocks + 1);
        for _ in 0..=n_stocks {
            combination_matrix.push(vec![-1.0; rmb + 1]);
        }
        let price_col: Vec<f64> = stocks.iter().map(|stock: &Stock| stock.price).collect();
        let earnings_col: Vec<f64> = stocks.iter().map(|stock: &Stock| stock.earnings).collect();
        check_all_combinations(
            &mut combination_matrix,
            &price_col,
            &earnings_col,
            rmb,
            n_stocks,
        );
        let mut chosen_stocks: Vec<Stock> = reconstruct_chosen_items(&combination_matrix, &stocks);
        chosen_stocks.sort_by(|a: &Stock, b: &Stock| {
            b.profit
                .partial_cmp(&a.profit)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        return chosen_stocks;
    }
}
