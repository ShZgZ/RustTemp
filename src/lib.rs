pub mod gate;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn and_gate_test() {
        assert_eq!(gate::and_gate(0, 0), 0);
        assert_eq!(gate::and_gate(1, 0), 0);
        assert_eq!(gate::and_gate(0, 1), 0);
        assert_eq!(gate::and_gate(1, 1), 1);
    }
/*
    #[test]
    fn another () {
        panic!("Make this test fail");
    }
*/
}
