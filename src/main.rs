mod components;

pub use crate::components::AndGate;

fn main() {
    let mut and_gate = AndGate::default();
    and_gate.settle();
    println!("{}", and_gate.output.lock().unwrap());
    {
        let mut input1_lock = and_gate.input1.lock().unwrap();
        *input1_lock = true;
    }
    and_gate.settle();
    println!("{}", and_gate.output.lock().unwrap());
    {
        let mut input2_lock = and_gate.input2.lock().unwrap();
        *input2_lock = true;
    }
    and_gate.settle();
    println!("{}", and_gate.output.lock().unwrap());
}
