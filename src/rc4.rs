pub(crate) struct RC4<'a> {
    key: &'a [u8],
}

impl RC4<'_> {
    pub fn new_rc4(key: &[u8]) -> RC4 {
        RC4 { key }
    }
    /// TODO: multithraded https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.736.151&rep=rep1&type=pdf
    pub fn apply_cipher(&mut self, data: &mut Vec<u8>) {
        let mut state = [0u8; 256];
        // KSA part
        state.iter_mut().enumerate().for_each(|(i, x)| {
            *x = i as u8;
        });

        let i_iter = 0..256usize;
        let key_iter = self.key.iter().cycle();

        let mut j = 0u8;
        i_iter.zip(key_iter).for_each(|(i, k)| {
            j = j.wrapping_add(state[i]).wrapping_add(*k);

            state.swap(i, j.into());
        });

        // PRGA part

        let mut idx1 = 0;
        let mut i = 0u8;
        let mut j = 0u8;

        while idx1 < data.len() {
            i = i.wrapping_add(1);
            j = j.wrapping_add(state[i as usize]);

            state.swap(i as usize, j as usize);
            let index: usize = state[i as usize].wrapping_add(state[j as usize]).into();
            data[idx1] ^= state[index];
            idx1 += 1;
        }
    }
}
