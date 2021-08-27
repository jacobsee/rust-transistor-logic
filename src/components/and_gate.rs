use crate::components::transistor::Transistor;
use crate::components::wire::wiring;

pub struct AndGate {
    t1: Transistor,
    t2: Transistor,
    pub input1: wiring::Wire,
    pub input2: wiring::Wire,
    pub output: wiring::Wire,
}

impl Default for AndGate {
    fn default() -> Self {
        let mut t1 = Transistor::default();
        let mut t2 = Transistor::default();
        wiring::set_high(&mut t1.collector);
        wiring::connect(&mut t2.collector, t1.emitter.clone());
        AndGate {
            input1: t1.base.clone(),
            input2: t2.base.clone(),
            output: t2.emitter.clone(),
            t2: t2,
            t1: t1,
        }
    }
}

impl AndGate {
    pub fn settle(&mut self) {
        self.t1.tick();
        self.t2.tick();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn false_and_false() {
        let mut and_gate = AndGate::default();
        wiring::set_low(&mut and_gate.input1);
        wiring::set_low(&mut and_gate.input2);
        and_gate.settle();
        let output_lock = and_gate.output.lock().unwrap();
        assert_eq!(*output_lock, false);
    }

    #[test]
    fn true_and_false() {
        let mut and_gate = AndGate::default();
        wiring::set_high(&mut and_gate.input1);
        wiring::set_low(&mut and_gate.input2);
        and_gate.settle();
        let output_lock = and_gate.output.lock().unwrap();
        assert_eq!(*output_lock, false);
    }

    #[test]
    fn false_and_true() {
        let mut and_gate = AndGate::default();
        wiring::set_low(&mut and_gate.input1);
        wiring::set_high(&mut and_gate.input2);
        and_gate.settle();
        let output_lock = and_gate.output.lock().unwrap();
        assert_eq!(*output_lock, false);
    }

    #[test]
    fn true_and_true() {
        let mut and_gate = AndGate::default();
        wiring::set_high(&mut and_gate.input1);
        wiring::set_high(&mut and_gate.input2);
        and_gate.settle();
        let output_lock = and_gate.output.lock().unwrap();
        assert_eq!(*output_lock, true);
    }
}
