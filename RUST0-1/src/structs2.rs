#[derive(Debug)]
enum TradeType { Buy, Sell }

#[derive(Debug)]
struct Trade {
    price: u64,
    quantity: u64,
    trade_type: TradeType,
}

impl Trade {
    fn new(price: u64, quantity: u64, trade_type: TradeType) -> Self {
        Self { price, quantity, trade_type }
    }

    fn value(&self) -> u64 {
        self.price * self.quantity
    }
}

fn main() {
    let t1 = Trade::new(100, 50, TradeType::Buy);
    println!("Trade: {:?}", t1);
    println!("Trade Value: {}", t1.value());
}