mod bindings;
use bindings::wasi_mindmap::interfaced_adder::add::add;

fn main() {
    let result = add(1, 2);
    println!("result: {result}");
}
