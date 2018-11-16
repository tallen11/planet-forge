use std::num::Wrapping;

pub struct XoroshiroRNG {
    state: [u32; 4],
}

impl XoroshiroRNG {
    pub fn new() -> XoroshiroRNG {
        XoroshiroRNG {
            state: XoroshiroRNG::generate_seed(),
        }
    }

    pub fn seeded(state: [u32; 4]) -> XoroshiroRNG {
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

    fn generate_seed() -> [u32; 4] {
        let b1 = Box::new(1);
        let b2 = Box::new(2);
        let b3 = Box::new(3);
        let b4 = Box::new(4);
        let addr1 = &b1 as *const Box<i32>;
        let addr2 = &b2 as *const Box<i32>;
        let addr3 = &b3 as *const Box<i32>;
        let addr4 = &b4 as *const Box<i32>;
        
        [addr1 as u32, addr2 as u32, addr3 as u32, addr4 as u32]
    }
}
