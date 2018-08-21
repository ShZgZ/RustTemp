pub mod gate;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn and_gate_tests() {
        assert_eq!(gate::and_gate(0, 0), 0);
        assert_eq!(gate::and_gate(1, 0), 0);
        assert_eq!(gate::and_gate(0, 1), 0);
        assert_eq!(gate::and_gate(1, 1), 1);
    }

    #[test]
    fn or_gate_tests() {
        assert_eq!(gate::or_gate(0, 0), 0);
        assert_eq!(gate::or_gate(1, 0), 1);
        assert_eq!(gate::or_gate(0, 1), 1);
        assert_eq!(gate::or_gate(1, 1), 1);
    }

    #[test]
    fn not_and_gate_tests(){
        assert_eq!(gate::not_and_gate(0, 0), 1);
        assert_eq!(gate::not_and_gate(1, 0), 1);
        assert_eq!(gate::not_and_gate(0, 1), 1);
        assert_eq!(gate::not_and_gate(1, 1), 0);
    }

    #[test]
    fn exclusive_or_gate_tests(){
        assert_eq!(gate::exclusive_or_gate(0, 0), 0);
        assert_eq!(gate::exclusive_or_gate(0, 1), 1);
        assert_eq!(gate::exclusive_or_gate(1, 0), 1);
        assert_eq!(gate::exclusive_or_gate(1, 1), 0);
    }

    #[test]
    fn nand_not_gate_tests() {
        assert_eq!(gate::nand_not_gate(0), 1);
        assert_eq!(gate::nand_not_gate(1), 0);
    }
    #[test]
    fn nand_and_gate_tests() {
        assert_eq!(gate::nand_and_gate(0, 0), 0);
        assert_eq!(gate::nand_and_gate(1, 0), 0);
        assert_eq!(gate::nand_and_gate(0, 1), 0);
        assert_eq!(gate::nand_and_gate(1, 1), 1);
    }
    #[test]
    fn nand_or_gate_tests() {
        assert_eq!(gate::nand_or_gate(0, 0), 0);
        assert_eq!(gate::nand_or_gate(1, 0), 1);
        assert_eq!(gate::nand_or_gate(0, 1), 1);
        assert_eq!(gate::nand_or_gate(1, 1), 1);
    }
/*
    #[test]
    fn another () {
        panic!("Make this test fail");
    }
*/
}
