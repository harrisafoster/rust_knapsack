pub mod reconstruction {
    use crate::structs::structs::Stock;

    pub fn reconstruct_chosen_items(combination_matrix: &Vec<Vec<f64>>, stocks: &Vec<Stock>) -> Vec<Stock> {
        let mut chosen_items: Vec<Stock> = Vec::new();
        let mut current_capacity: usize = combination_matrix[0].len() - 1;
        let mut current_index: usize = stocks.len();
    
        while current_index > 0 && current_capacity > 0 {
            if combination_matrix[current_index][current_capacity]
                != combination_matrix[current_index - 1][current_capacity]
            {
                let chosen_stock = &stocks[current_index - 1];
                chosen_items.push(Stock {
                    name: chosen_stock.name.clone(),
                    price: chosen_stock.price,
                    profit: chosen_stock.profit,
                    earnings: chosen_stock.earnings,
                });
                current_capacity -= (chosen_stock.price * 100.0) as usize;
            }
            current_index -= 1;
        }
    
        chosen_items
    }
}