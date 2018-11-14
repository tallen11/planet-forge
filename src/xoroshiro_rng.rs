use std::num::Wrapping;

pub struct XoroshiroRNG {
    state: [u32; 4],
}

impl XoroshiroRNG {
    pub fn new(state: [u32; 4]) -> XoroshiroRNG {
        XoroshiroRNG {
            state: state,
        }
    }

    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        let result_plus = (Wrapping(self.state[0]) + Wrapping(self.state[3])).0;
        let t = self.state[1] << 9;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;

        self.state[3] = XoroshiroRNG::rotl_u32(self.state[3], 11);

        result_plus
    }

    #[inline]
    pub fn next_f32(&mut self) -> f32 {
        (self.next_u32() & 0xFFFFFF) as f32 / 16777216.0
    }

    #[inline]
    fn rotl_u32(x: u32, k: u32) -> u32 {
        (x << k) | (x >> (32 - k))
    }
}
