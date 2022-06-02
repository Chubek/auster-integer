pub(crate) mod conversion {
    use crate::utils::gen::pad_with_zeros;

    use super::bitwise_ops::twos_complement;

    pub fn convert_from_decimal(decimal: i128, pad: usize) -> Vec<u8> {
        let mut remainders = Vec::<u8>::new();

        let mut dec_abs = i128::abs(decimal.clone()) as u128;

        let mut qoutient = 0u128;

        loop {
            qoutient = dec_abs / 2u128;
            let remainder = dec_abs % 2u128;
            dec_abs = qoutient;

            remainders.push(remainder as u8);

            if dec_abs == 0 {
                break;
            }
        }

        pad_with_zeros(&mut remainders, pad);

        remainders.reverse();

        if decimal < 0 {
            twos_complement(&mut remainders);
        }

        remainders
    }

    pub fn convert_to_decimal(bin_array: &Vec<u8>) -> i128 {
        let mut arr_clone = bin_array.clone();

        let mut res = 0u128;
        let mut fr = 0usize;

        if arr_clone[0] == 1 {
            super::bitwise_ops::twos_complement(&mut arr_clone);
            fr = 1;
        }

        let arr_u128 = bin_array
            .clone()
            .into_iter()
            .map(|x| x as u128)
            .collect::<Vec<u128>>();

        for (i, j) in (fr..arr_u128.len()).zip((0..arr_u128.len() - 1).rev()) {
            let exp = 2u128.pow(j.try_into().unwrap());
            res += arr_u128[i] * exp;
        }

        let ret = match fr {
            0 => res as i128,
            1 => -(res as i128),
            _ => 0i128,
        };

        ret
    }

    pub fn make_zero(size: usize) -> Vec<u8> {
        vec![0u8; size]
    }

    pub fn make_one(size: usize) -> Vec<u8> {
        vec![1u8; size]
    }
}

pub(crate) mod bitwise_ops {
    use crate::utils::math::{flip_one, replace_with};

    pub fn twos_complement(binary_arry: &mut Vec<u8>) {
        let mut switch = false;

        for i in (0..binary_arry.len()).rev() {
            if switch {
                binary_arry[i] ^= 1;
            }

            if binary_arry[i] == 1 {
                switch = true;
            }
        }
    }

    pub fn ones_complement(binary_array: &mut Vec<u8>) {
        (0..binary_array.len()).for_each(|u| flip_one(u, binary_array));
    }

    pub fn bitwise_and(array_a: &Vec<u8>, array_b: &Vec<u8>) -> Vec<u8> {
        array_a
            .iter()
            .zip(array_b.iter())
            .map(|(a, b)| a & b)
            .collect::<Vec<u8>>()
    }

    pub fn bitwise_or(array_a: &Vec<u8>, array_b: &Vec<u8>) -> Vec<u8> {
        array_a
            .iter()
            .zip(array_b.iter())
            .map(|(a, b)| a | b)
            .collect::<Vec<u8>>()
    }

    pub fn bitwise_xor(array_a: &Vec<u8>, array_b: &Vec<u8>) -> Vec<u8> {
        array_a
            .iter()
            .zip(array_b.iter())
            .map(|(a, b)| a ^ b)
            .collect::<Vec<u8>>()
    }

    pub fn bitwise_not(binary_array: &mut Vec<u8>) {
        (0..binary_array.len()).for_each(|u| flip_one(u, binary_array));
    }

    pub fn left_shift(binary_array: &mut Vec<u8>, num: usize) {
        binary_array
            .clone()
            .iter_mut()
            .enumerate()
            .filter(|(i, _)| *i > num - 1)
            .map(|(i, x)| x.clone())
            .chain(vec![0u8; num].into_iter())
            .enumerate()
            .for_each(|(i, x)| replace_with(binary_array, i, x));
    }

    pub fn right_shift(binary_array: &mut Vec<u8>, num: usize) {
        let len = binary_array.len();
        let zeroes = vec![0u8; num];
        let bin_vec = vec![zeroes, binary_array.clone()];
        bin_vec
            .into_iter()
            .flatten()
            .enumerate()
            .filter(|(i, _)| *i >= num && *i < len)
            .for_each(|(i, x)| replace_with(binary_array, i, x));
    }
}

pub(crate) mod arithmetic_ops {
    use crate::utils::{
        gen::{get_element_or_zero, pad_with_zeros},
        math::rep_single,
    };

    use super::{
        bitwise_ops::{bitwise_and, left_shift, right_shift},
        conversion::{make_one, make_zero},
    };

    pub enum CompareOp {
        GreaterThanEq,
        LessThanEq,
        GreaterThan,
        LessThan,
        Eq,
    }

    pub fn binary_add(a: &Vec<u8>, b: &Vec<u8>, pad: usize) -> Vec<u8> {
        let mut carry = 0u8;

        let mut res: Vec<u8> = vec![];

        let mut ai = a.len() - 1;
        let mut bi = b.len() - 1;

        loop {
            let el_a = get_element_or_zero(&ai, &a);
            let el_b = get_element_or_zero(&bi, &b);

            let mut val = el_a + el_b + carry;

            carry = match val > 1 {
                true => {
                    val %= 2;
                    1
                }
                false => 0,
            };

            res.push(val);

            ai -= 1;
            bi -= 1;

            if ai == 0 || bi == 0 {
                break;
            }
        }

        pad_with_zeros(&mut res, pad);

        res.reverse();

        res
    }

    pub fn binary_subtract(a: &Vec<u8>, b: &Vec<u8>, pad: usize) -> Vec<u8> {
        let mut b_clone = b.clone();

        super::bitwise_ops::twos_complement(&mut b_clone);

        binary_add(a, &b_clone, pad)
    }

    pub fn binary_multipy(a: &Vec<u8>, b: &Vec<u8>, pad: usize) -> Vec<u8> {
        let mut sums: Vec<Vec<u8>> = vec![];

        let size = a.len();
        let zeros = super::conversion::make_zero(size);

        for (i, d) in b.iter().rev().enumerate() {
            if d == &0 {
                sums.push(zeros.clone());
            } else {
                let mut a_clone = a.clone();
                super::bitwise_ops::left_shift(&mut a_clone, i);
                sums.push(a_clone);
            }
        }

        let mut res = zeros.clone();

        sums.iter().for_each(|x| res = binary_add(&res, x, pad));

        res
    }

    pub fn binary_divide(n: &Vec<u8>, d: &Vec<u8>, pad: usize) -> (Vec<u8>, Vec<u8>) {
        let mut q = make_zero(n.len());
        let mut r = make_zero(n.len());

        let mut i = n.len() - 1;

        loop {
            left_shift(&mut r, 1);

            rep_single(&mut r, n[i]);

            if compare(&r, d, pad, CompareOp::GreaterThanEq) {
                r = binary_subtract(&r, d, pad);

                q[n.len() - 1] = 1
            }

            i -= 1;

            if i == 0 {
                break;
            }
        }

        (q, r)
    }

    pub fn binary_expontent(a: &Vec<u8>, p: &Vec<u8>, pad: usize) -> Vec<u8> {
        let mut clone_a = a.clone();
        let mut clone_p = p.clone();

        let mut res = make_zero(a.len());

        let ones = make_one(p.len());

        while clone_p.iter().any(|x| x == &1) {
            if bitwise_and(&clone_p, &ones).iter().all(|x| x == &1) {
                res = binary_multipy(&res, &clone_a, pad);
            }

            clone_a = binary_multipy(&clone_a, &clone_a, pad);

            right_shift(&mut clone_p, 1);
        }

        res
    }

    pub fn compare(a: &Vec<u8>, b: &Vec<u8>, pad: usize, op: CompareOp) -> bool {
        let diff = binary_subtract(&a, &b, pad);

        let res = match op {
            CompareOp::Eq => diff.iter().all(|x| x == &0),
            CompareOp::GreaterThan | CompareOp::GreaterThanEq => diff[0] == 0,
            CompareOp::LessThan => diff[0] == 1,
            CompareOp::LessThanEq => diff[0] == 1 || diff.iter().all(|x| x == &0),
        };

        res
    }
}
