
struct Counter {
    value:i32,
}

impl Counter {
    fn new(value:i32) -> Counter {
        Counter{value}
    }

    fn increment(&mut self) {
        self.value = self.value + 1; 
    }
    fn get_value(&self) -> i32 {
        self.value
    }
    fn reset(&mut self) {
        self.value = 0;
    }
}

fn main() {
    let mut counter = Counter::new(0);
    counter.increment();
    counter.increment();
    counter.increment();
    println!("{}", counter.get_value());
    counter.reset();
    println!("{}", counter.get_value());

}





