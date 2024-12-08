mod bindings; // ISSUE: This line needs to be added manually, see https://github.com/bytecodealliance/cargo-component/issues/361
use bindings::component::interfaced_adder::add::add;
// ISSUE: cargo-component cannot find exported functions, see https://github.com/bytecodealliance/cargo-component/issues/360

fn main() {
    let result = add(1, 2);
    println!("result: {}", result);
}
