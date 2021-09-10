fn main() {
    let d = Dog{
        age: 2,
        name: "dog1".to_string()
    };
    println!("{} is {} years old", d.name, d.age());
}

trait Animal {
    fn age(&self) -> u32;
}

struct Dog {
    age: u32,
    name: String
}

impl Animal for Dog {
    fn age(&self) -> u32 {
        self.age
    }
}
