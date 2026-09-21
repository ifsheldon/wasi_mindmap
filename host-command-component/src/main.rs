mod bindings {
    wit_bindgen::generate!({
        // Load the imported package before the world that uses it.
        path: ["../guest-interfaced-adder-rs/wit", "wit"],
        world: "wasi-mindmap:host/host",
        generate_all,
    });
}

use bindings::wasi_mindmap::interfaced_adder::add::add;

fn main() {
    let result = add(1, 2);
    println!("result: {result}");
}
