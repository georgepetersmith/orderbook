use orderbook::{OrderBook, Side};

fn main() {
    let mut order_book = OrderBook::new();
    let id = order_book.add_order(3, Side::Bid, 5);
    let c = order_book.add_order(2, Side::Bid, 1);
    order_book.add_order(4, Side::Ask, 2);

    println!("{:#?}", order_book);

    order_book.cancel_order(&c).unwrap();

    println!("{:#?}", order_book);

    let order = order_book.get_order(&id);
    println!("{:#?}", order);
}
