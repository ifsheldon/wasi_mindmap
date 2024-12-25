// Use a procedural macro to generate bindings for the world we specified in
// `wit/adder.wit`
wit_bindgen::generate!({
    // the name of the world in the `*.wit-files` input file
    world: "adder",
});

// Define a custom type and implement the generated `Guest` trait for it which
// represents implementing all the necessary exported interfaces for this
// component.
struct Adder;

impl Guest for Adder {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

// export! defines that the `Adder` struct defined below is going to define
// the exports of the `world`
export!(Adder);
