fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {
        return 0
    }

    let mut buy = prices[0];
    let mut sell = prices[1];
    let mut profit = sell - buy;
    let len = prices.len();

    for (i, price) in prices.iter().enumerate() {
        if *price < buy {
            if i + 1 < len {
                buy = *price;
                sell = prices[i + 1];
            }
        } else if *price > sell {
            sell = *price;
        }

        if sell - buy > profit {
            profit = sell - buy;
        }
    }

    if profit < 0 { 0 } else { profit }
}

// AI refactored / Kadane's algorithm:
//
// pub fn max_profit(prices: Vec<i32>) -> i32 {
    // let mut min_price = i32::MAX;
    // let mut max_profit = 0;

    // for price in prices {
        // min_price = min_price.min(price);
        // max_profit = max_profit.max(price - min_price);
    // }

    // max_profit
// }

#[cfg(test)]
mod tests {
    use crate::solutions::solution_121::max_profit;

    #[test]
    fn test_max_profit_1() {
        let prices = vec![7,1,5,3,6,4];
        assert_eq!(5, max_profit(prices));
    }

    #[test]
    fn test_max_profit_2() {
        let prices = vec![7,6,4,3,1];
        assert_eq!(0, max_profit(prices));
    }

    #[test]
    fn test_max_profit_3() {
        let prices = vec![2,4,1];
        assert_eq!(2, max_profit(prices));
    }

    #[test]
    fn test_max_profit_4() {
        let prices = vec![3,2,6,5,0,3];
        assert_eq!(4, max_profit(prices));
    }

    #[test]
    fn test_max_profit_5() {
        let prices = vec![2,1,2,1,0,1,2];
        assert_eq!(2, max_profit(prices));
    }
}
