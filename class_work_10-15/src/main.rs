#[derive(PartialEq, Debug)]
enum Fruit {
    Apple(String),
    Banana(String),
    Tomato(String),
}

struct Inventory {
    fruit: Vec<Fruit>,
}

impl Inventory {
    fn available_fruits(&self) {
        for f in &self.fruit {
            print!("{:?}: ", f);
            Self::tell_me_joke(f);
        }
    }

    fn tell_me_joke(fruit: &Fruit) {
        match fruit {
            Fruit::Apple(_) => println!("Why did the apple stop in the middle of the road? Because it ran out of juice!"),
            Fruit::Banana(_) => println!("What do bananas say when they pick up the phone? Yellow!"),
            Fruit::Tomato(_) => println!("Why did the tomato turn red? Because it saw the salad dressing!"),
        }
    }
}

fn main(){
    let a = "An apple a day keeps the doctor away.".to_string();
    let b = "A banana boosts energy in a peel.".to_string();
    let t = "A tomato a day keeps the sunburn away.".to_string();
    let fruits = vec![Fruit::Banana(b),Fruit::Apple(a),Fruit::Tomato(t)];
    let grocery_store = Inventory {
        fruit:fruits,
    };
   grocery_store.available_fruits();

}
