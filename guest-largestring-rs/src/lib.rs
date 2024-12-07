wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "big-string",
});

struct LargeString;

impl Guest for LargeString {
    fn manipulate_large_string() -> String {
        print("hello from Rust guest!");
        let large_string = component::big_string::large_string::Largestring::new();
        large_string.push("a");
        large_string.push("b");
        let s = large_string.get();
        large_string.clear();
        s
    }
}

export!(LargeString);
