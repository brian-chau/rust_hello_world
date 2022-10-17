// Struct Definition
struct World {
    phrase: String,
}

// Impl block defining method
impl World {
    fn say_phrase(&self) {
        println!("{}", self.phrase);
    }
}

fn main() {
    // Instantiate a World
    let hello: World = World {
        phrase: "Hello world".to_string(),
    };

    // Call methods
    hello.say_phrase();
}
