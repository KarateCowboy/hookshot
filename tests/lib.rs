use cucumber::{given, when, then, World};

// These `Cat` definitions would normally be inside your project's code, 
// not test code, but we create them here for the show case.
#[derive(Debug, Default)]
struct Cat {
    pub hungry: bool,
}

impl Cat {
    fn feed(&mut self) {
        self.hungry = false;
    }
}

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario. 
#[derive(Debug, Default, World)]
pub struct AnimalWorld {
    number: i32,
    new_number: i32
}

// Steps are defined with `given`, `when` and `then` attributes.
#[given("I have a number in a variable")]
fn hungry_cat(world: &mut AnimalWorld) {
    world.number = 3;
}
#[when("I add two to it")]
fn add_two_to_it(world: &mut AnimalWorld){
    world.new_number = world.number + 2;
}
#[then("the number sholud be two larger")]
fn number_should_be_two_larger(world: &mut AnimalWorld){
    assert_eq!((world.number + 2), world.new_number);
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(AnimalWorld::run("tests/features/lib/thing.feature"));
}
