mod matching_engine;

use matching_engine::orderbook::{Order, BidOrAsk, OrderBook};
use matching_engine::engine::{MatchingEngine, TradingPair};
fn main() {
    let buy_order_from_alice = Order::new(BidOrAsk::Bid, 5.5);
    let buy_order_from_bob = Order::new(BidOrAsk::Bid, 2.45);

    let mut orderBook = OrderBook::new();
    orderBook.add_order(4.4, buy_order_from_alice);
    orderBook.add_order(4.4, buy_order_from_bob);


  //  print!("{:?}", orderBook);

  let mut engine = MatchingEngine::new();
  let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
  engine.add_new_market(pair);
}
