use gate_logic::{xor, and};

pub type Carry = u8;
pub type Sum = u8;

pub fn half_adder(a: u8, b: u8) -> (Option<Sum>, Option<Carry>) {
    (xor(a, b), and(a, b))
}

#[test]
fn one_bit_adder() {
    let test_data = vec![
     ((0, 0), (0, 0)),
     ((0, 1), (1, 0)),
     ((1, 0), (1, 0)),
     ((1, 1), (0, 1))
     ];

     for (inn, out) in test_data {
         let (a, b) = inn;
         let get = match half_adder(a, b) {
             (Some(v), Some(m)) => (v, m),
             _ => panic!("Test data wrong!")
         };
         assert_eq!(get, out);
     }
}