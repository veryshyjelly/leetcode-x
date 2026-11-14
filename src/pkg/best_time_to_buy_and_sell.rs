#[allow(dead_code)]
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = i32::MAX;
    let mut max_profit = 0;

    prices.into_iter().for_each(|price| {
        min_price = i32::min(min_price, price);
        max_profit = i32::max(price - min_price, max_profit);
    });

    max_profit
}