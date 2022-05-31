pub(crate) mod math {
    pub fn flip_one(u: usize, vec: &mut Vec<u8>) {
        let flipped = vec[u] ^ 1;
        vec[u] = flipped
    }
    
    pub fn rep_single(v: &mut Vec<u8>, rep: u8) {
        let last = v.len();
        v[last - 1]  = rep;
    }
}

pub(crate) mod gen {
    pub fn get_element_or_zero(u: &usize, v: &Vec<u8>) -> u8 {
        let num = ((*u !=0) & (1usize != 0)) as u8;

        v[*u] * num
    }

    pub fn pad_with_zeros(v: &mut Vec<u8>, pad: usize) {
        if v.len() % pad != 0 {
            let pad_amount = pad - (v.len() % pad);
            let padders = vec![0u8; pad_amount];

            v.extend(padders);
        }
    }
}