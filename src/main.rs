#![allow(dead_code)]

use std::collections::HashMap;

const SCALER: u64 = 100_000;

#[derive(Debug)]
enum Side {
    Bid,
    Ask,
}

#[derive(Debug)]
struct Order {
    side: Side,
    size: f64,
}

impl Order {
    fn new(side: Side, size: f64) -> Self {
        Order { side, size }
    }
}

#[derive(Debug)]
struct PriceLevel {
    price: Price,
    orders: Vec<Order>,
}

impl PriceLevel {
    fn new(price: f64) -> Self {
        PriceLevel {
            price: Price::new(price),
            orders: Vec::new(),
        }
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Price {
    integral: u64,
    fractional: u64,
}

impl Price {
    fn new(price: f64) -> Self {
        let integral = price as u64;

        let fractional = (price.fract() * (SCALER as f64)) as u64;

        Price {
            integral,
            fractional,
        }
    }
}

#[derive(Debug)]
struct OrderBook {
    bids: HashMap<Price, PriceLevel>,
    asks: HashMap<Price, PriceLevel>,
}

impl OrderBook {
    fn new() -> Self {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    fn add_order(&mut self, price: f64, order: Order) {
        let price_levels = match order.side {
            Side::Bid => &mut self.bids,
            Side::Ask => &mut self.asks,
        };

        let price_struct = Price::new(price);

        match price_levels.get_mut(&price_struct) {
            Some(price_level) => price_level.add_order(order),
            None => {
                let mut price_level = PriceLevel::new(price);
                price_level.add_order(order);
                price_levels.insert(price_struct, price_level);
            }
        };
    }
}

fn main() {
    let mut order_book = OrderBook::new();
    order_book.add_order(6.5, Order::new(Side::Bid, 1.0));
    order_book.add_order(6.5, Order::new(Side::Bid, 3.0));
    order_book.add_order(1.2, Order::new(Side::Bid, 3.0));
    order_book.add_order(4.9, Order::new(Side::Ask, 0.7));

    println!("{:#?}", order_book);
}
