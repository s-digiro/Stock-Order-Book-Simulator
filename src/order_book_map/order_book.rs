use std::time::SystemTime;
use crate::transaction::Side;
use crate::transaction::New;

#[derive(PartialEq)]
#[derive(Debug)]
struct Order {
    user: i32,
    price: usize,
    quantity: usize,
    user_order_id: i32,
    time: SystemTime
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct OrderBook {
    enable_cross: bool,
    symbol: String,
    last_tob_buy: Option<(usize, usize)>,
    last_tob_sell: Option<(usize, usize)>,
    buys: Vec<Order>,
    sells: Vec<Order>,
}

impl OrderBook {
    pub fn new(symbol: &str, enable_cross: bool) -> OrderBook {
        OrderBook {
            enable_cross,
            symbol: symbol.to_string(),
            last_tob_buy: None,
            last_tob_sell: None,
            buys: Vec::new(),
            sells: Vec::new(),
        }
    }

    fn sort_books(&mut self) {
        self.buys.sort_by(|a, b| {
            use std::cmp::Ordering;

            let result = b.price.partial_cmp(&a.price).unwrap();
            if result == Ordering::Equal {
                a.time.partial_cmp(&b.time).unwrap()
            } else {
                result
            }
        });

        if let Some(o) = self.buys.first() {
            let tot_qty: usize = self.buys.iter()
                .filter(|x| x.price == o.price)
                .map(|x| x.quantity)
                .sum();

            if self.last_tob_buy != Some((o.price, tot_qty)) {
                println!("B, B, {}, {}", o.price, tot_qty);
                self.last_tob_buy = Some((o.price, tot_qty));
            }
        } else if self.last_tob_buy != None {
            println!("B, B, -, -");
            self.last_tob_buy = None;
        }


        self.sells.sort_by(|a, b| {
            use std::cmp::Ordering;

            let result = a.price.partial_cmp(&b.price).unwrap();
            if result == Ordering::Equal {
                a.time.partial_cmp(&b.time).unwrap()
            } else {
                result
            }
        });

        if let Some(o) = self.sells.first() {
            let tot_qty: usize = self.sells.iter()
                .filter(|x| x.price == o.price)
                .map(|x| x.quantity)
                .sum();

            if self.last_tob_sell != Some((o.price, tot_qty)) {
                println!("B, S, {}, {}", o.price, tot_qty);
                self.last_tob_sell = Some((o.price, tot_qty));
            }
        } else if self.last_tob_sell != None {
            println!("B, S, -, -");
            self.last_tob_sell = None;
        }
    }

    pub fn submit_order(&mut self, n: New) {
        if self.would_cross(&n.side, n.price) {
            if self.enable_cross {
                println!("A, {}, {}", n.user, n.user_order_id);
                self.trade(n.user, &n.side, n.price, n.quantity, n.user_order_id);
            } else {
                println!("R, {}, {}", n.user, n.user_order_id);
            }
        } else {
            println!("A, {}, {}", n.user, n.user_order_id);
            self.add(n.user, &n.side, n.price, n.quantity, n.user_order_id);
        }

        self.sort_books();
    }

    pub fn trade(
        &mut self,
        user: i32,
        side: &Side,
        price: usize,
        mut quantity: usize,
        user_order_id: i32
    ) {
        use std::cmp::min;
        use std::cmp::max;

        match side {
            Side::Buy => if !self.sells.is_empty() {
                for sell in self.sells.iter_mut().filter(|o| o.price <= price) {
                    let trade_qty = min(quantity, sell.quantity);
                    println!(
                        "T, {}, {}, {}, {}, {}, {}",
                        user, user_order_id,
                        sell.user, sell.user_order_id,
                        sell.price, trade_qty,
                    );

                    quantity = max(0, quantity - trade_qty);
                    sell.quantity = max(0, sell.quantity - trade_qty);

                    if quantity == 0 {
                        break;
                    }
                }

                self.sells.retain(|o| o.quantity > 0);
            },
            Side::Sell => if !self.buys.is_empty() {
                for buy in self.buys.iter_mut().filter(|o| o.price >= price) {
                    let trade_qty = min(quantity, buy.quantity);
                    println!(
                        "T, {}, {}, {}, {}, {}, {}",
                        buy.user, buy.user_order_id,
                        user, user_order_id,
                        buy.price, trade_qty,
                    );

                    quantity = max(0, quantity - trade_qty);
                    buy.quantity = max(0, buy.quantity - trade_qty);

                    if quantity == 0 {
                        break;
                    }
                }

                self.buys.retain(|o| o.quantity > 0);
            },
        }
    }

    pub fn add(
        &mut self,
        user: i32,
        side: &Side,
        mut price: usize,
        quantity: usize,
        user_order_id: i32
    ) {
        match side {
            Side::Sell => {
                // Set to market order if price is nothing
                if price == 0 {
                    price = self.last_tob_buy
                        .unwrap_or(
                            self.last_tob_sell
                                .unwrap_or((0,0))
                        ).0;

                }

                self.sells.push(
                    Order {
                        user,
                        price,
                        quantity,
                        user_order_id,
                        time: SystemTime::now(),
                    }
                );
            },
            Side::Buy => {
                // Set to market order if price is nothing
                if price == 0 {
                    price = self.last_tob_buy
                        .unwrap_or(
                            self.last_tob_sell
                                .unwrap_or((0,0))
                        ).0;

                }

                self.buys.push(
                    Order {
                        user,
                        price,
                        quantity,
                        user_order_id,
                        time: SystemTime::now(),
                    }
                )
            },
        }
    }

    fn would_cross(&self, side: &Side, price: usize) -> bool {
        match side {
            Side::Buy => {
                if self.sells.is_empty() {
                    false
                } else {
                    price >= self.min_sell().unwrap()
                }
            },
            Side::Sell => {
                if self.buys.is_empty() {
                    false
                } else {
                    price <= self.max_buy().unwrap()
                }
            },
        }
    }

    fn min_sell(&self) -> Option<usize> {
        self.sells.iter()
            .map(|order| order.price)
            .min()
    }

    fn max_buy(&self) -> Option<usize> {
        self.buys.iter()
            .map(|order| order.price)
            .max()
    }

    pub fn remove(&mut self, user: i32, user_order_id: i32) {
        let original_length = self.sells.len() + self.buys.len();

        self.sells
            .retain(|x| !(x.user == user && x.user_order_id == user_order_id));
        self.buys
            .retain(|x| !(x.user == user && x.user_order_id == user_order_id));

        let removed = original_length - (self.sells.len() + self.buys.len());

        if removed > 0 {
            println!("A, {}, {}", user, user_order_id)
        }

        self.sort_books();
    }
}
