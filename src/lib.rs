use std::collections::{HashMap, VecDeque};

type Price = i64;
type Size = u32;

#[derive(Debug, Clone)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Debug)]
struct Order {
    id: i32,
    initial_size: Size,
    remaining_size: Size,
}

impl Order {
    fn new(id: i32, size: Size) -> Self {
        Order {
            id,
            initial_size: size,
            remaining_size: size,
        }
    }

    fn fill(&mut self, quantity: u32) -> Result<u32, &str> {
        if quantity > self.remaining_size {
            return Err("Cannot fill order for quantity greater than remaining size.");
        }

        self.remaining_size -= quantity;

        Ok(self.remaining_size)
    }
}

#[derive(Debug)]
pub struct OrderInfo {
    pub id: i32,
    pub initial_size: Size,
    pub remaining_size: Size,
    pub side: Side,
    pub price: Price,
}

#[derive(Debug)]
pub struct OrderBook {
    next_id: i32,
    bids: HashMap<Price, VecDeque<Order>>,
    asks: HashMap<Price, VecDeque<Order>>,
    order_price_map: HashMap<i32, (Side, Price)>,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            next_id: 1,
            bids: HashMap::new(),
            asks: HashMap::new(),
            order_price_map: HashMap::new(),
        }
    }

    pub fn get_order(&self, id: &i32) -> Option<OrderInfo> {
        let (side, price) = self.order_price_map.get(&id)?;
        let price_levels = match side {
            Side::Bid => &self.bids,
            Side::Ask => &self.asks,
        };

        let order = price_levels.get(price)?.into_iter().find(|o| o.id == *id)?;

        Some(OrderInfo {
            id: order.id,
            initial_size: order.initial_size,
            remaining_size: order.remaining_size,
            side: side.to_owned(),
            price: price.to_owned(),
        })
    }

    pub fn add_order(&mut self, price: Price, side: Side, size: Size) -> i32 {
        let id = self.next_id;
        let price_levels = match side {
            Side::Bid => &mut self.bids,
            Side::Ask => &mut self.asks,
        };

        let order = Order::new(id, size);

        match price_levels.get_mut(&price) {
            Some(orders) => orders.push_back(order),
            None => {
                let mut orders = VecDeque::new();
                orders.push_back(order);
                price_levels.insert(price, orders);
            }
        }

        self.order_price_map.insert(id, (side, price));

        self.next_id += 1;

        id
    }

    pub fn cancel_order(&mut self, id: &i32) -> Result<(), &str> {
        const ERROR: &str = "";

        let (side, price) = self.order_price_map.get(&id).ok_or(ERROR)?;

        let price_levels = match side {
            Side::Bid => &mut self.bids,
            Side::Ask => &mut self.asks,
        };

        let orders = price_levels.get_mut(price).ok_or(ERROR)?;
        let order_index = orders.iter().position(|o| o.id == *id).ok_or(ERROR)?;

        orders.remove(order_index).ok_or(ERROR)?;

        if orders.len() == 0 {
            price_levels.remove(price);
        }

        self.order_price_map.remove(id);

        Ok(())
    }
}
