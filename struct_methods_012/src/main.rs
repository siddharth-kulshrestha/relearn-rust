struct FruitBasket {
    apples: i32,
    bananas: i32,
}

impl FruitBasket {
    fn count_all(&self) -> i32 {
        return self.apples + self.bananas;
    }

    fn add_apples(&mut self) {
        self.apples += 1;
    }

    fn add_bananas(&mut self) {
        self.bananas += 1;
    }

    fn new(apples: i32, bananas: i32) -> FruitBasket {
        FruitBasket {
            apples: apples,
            bananas: bananas,
        }
    }
}

fn count_apples(fb: FruitBasket) {
    println!("Number of apples in the fruit basket: {}", fb.apples);
}

fn price_fruit_basket(fb: FruitBasket) {
    println!("Price of fruitbasket: {}", fb.apples * 5 + fb.bananas * 2);
}

fn main() {
    let mut fb = FruitBasket::new(10, 5);
    println!("Total fruits in fruit basket: {}", fb.count_all());
    fb.add_apples();
    println!("Total fruits in fruit basket: {}", fb.count_all());
    fb.add_bananas();
    println!("Total fruits in fruit basket: {}", fb.count_all());
    count_apples(fb);

    // price_fruit_basket(fb);
    /*
        36 |     let mut fb = FruitBasket::new(10, 5);
       |         ------ move occurs because `fb` has type `FruitBasket`, which does not implement the `Copy` trait
    ...
    42 |     count_apples(fb);
       |                  -- value moved here
    43 |     price_fruit_basket(fb);
       |                        ^^ value used here after move
        */
}
