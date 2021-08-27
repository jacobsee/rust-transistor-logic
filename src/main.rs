mod components;

pub use crate::components::{AndGate, wiring};

fn main() {
    let mut and_gate = AndGate::default();
    and_gate.settle();
    println!("Inputs: {}, {} Output: {}", and_gate.input1.lock().unwrap(), and_gate.input2.lock().unwrap(), and_gate.output.lock().unwrap());

    wiring::set_high(&mut and_gate.input1);
    and_gate.settle();
    println!("Inputs: {}, {} Output: {}", and_gate.input1.lock().unwrap(), and_gate.input2.lock().unwrap(), and_gate.output.lock().unwrap());

    wiring::set_high(&mut and_gate.input2);
    and_gate.settle();
    println!("Inputs: {}, {} Output: {}", and_gate.input1.lock().unwrap(), and_gate.input2.lock().unwrap(), and_gate.output.lock().unwrap());
}
