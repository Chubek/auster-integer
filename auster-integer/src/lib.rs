#![allow(unused)]

pub(crate) mod bin_utils;
pub(crate) mod utils;

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::bin_utils::{
        bitwise_ops::{
            bitwise_and, bitwise_not, bitwise_or, bitwise_xor, 
            ones_complement, twos_complement, right_shift, left_shift,
        },
        conversion::convert_from_decimal,
    };

    #[test]
    fn test_ones_complement() {
        let mut num = vec![1u8, 0u8, 1u8, 1u8, 1u8];
        let swapped = vec![0u8, 1u8, 0u8, 0u8, 0u8];

        ones_complement(&mut num);

        assert_eq!(swapped, num);
    }

    #[test]
    fn test_twos_complement() {
        let mut num = vec![0u8, 0u8, 1u8, 0u8, 1u8];
        let swapped = vec![1u8, 1u8, 0u8, 1u8, 1u8];

        twos_complement(&mut num);

        assert_eq!(swapped, num);
    }

    #[test]
    fn test_positive_conversion() {
        let num = 100i128;
        let bin_should_be = vec![0, 1, 1, 0, 0, 1, 0, 0];
        let bin = convert_from_decimal(num, 8usize);

        assert_eq!(bin, bin_should_be);
    }

    #[test]
    fn test_negative_conversion() {
        let num = -100i128;
        let bin_should_be = vec![1, 0, 0, 1, 1, 1, 0, 0];
        let bin = convert_from_decimal(num, 8usize);

        assert_eq!(bin, bin_should_be);
    }

    #[test]
    fn test_bitwise_operators() {
        let array_a = vec![0u8, 1u8, 1u8, 1u8, 0u8];
        let array_b = vec![1u8, 1u8, 0u8, 1u8, 0u8];

        let nums_anded = vec![0u8, 1u8, 0u8, 1u8, 0u8];
        let nums_oreded = vec![1u8, 1u8, 1u8, 1u8, 0u8];
        let nums_xoreded = vec![1u8, 0u8, 1u8, 0u8, 0u8];
        let num_a_notted = vec![1u8, 0u8, 0u8, 0u8, 1u8];

        let and = bitwise_and(&array_a, &array_b);
        let or = bitwise_or(&array_a, &array_b);
        let xor = bitwise_xor(&array_a, &array_b);

        let mut not = array_a.clone();

        bitwise_not(&mut not);

        assert_eq!(and, nums_anded);
        assert_eq!(or, nums_oreded);
        assert_eq!(xor, nums_xoreded);
        assert_eq!(not, num_a_notted);
    }

    #[test]
    fn test_shift_operators() {
        let array = vec![0u8, 1u8, 1u8, 1u8, 0u8];

        let sl_1 =  vec![1u8, 1u8, 1u8, 0u8, 0u8];
        let sl_2 =  vec![1u8, 0u8, 0u8, 0u8, 0u8];

        let sr_1 = vec![0u8, 0u8, 1u8, 1u8, 1u8];
        let sr_2 = vec![0u8, 0u8, 0u8, 0u8, 1u8];

        let mut arr_tsl = array.clone();
        let mut arr_tsr = array.clone();

        left_shift(&mut arr_tsl, 1);
        assert_eq!(arr_tsl, sl_1);
        left_shift(&mut arr_tsl, 2);
        assert_eq!(arr_tsl, sl_2);


        right_shift(&mut arr_tsr, 1);
        assert_eq!(arr_tsr, sr_1);
        right_shift(&mut arr_tsr, 2);
        assert_eq!(arr_tsr, sr_2);

    }
}
