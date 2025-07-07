use std::f32::consts::{LOG10_2, PI};

use crate::filters::BiquadVars;

pub const fn prev_power_of_two(n: u32) -> u32 {
    // n = 0 gives highest_bit_set_idx = 0.
    let highest_bit_set_idx = 31 - (n | 1).leading_zeros();
    // Binary AND of highest bit with n is a no-op, except zero gets wiped.
    (1 << highest_bit_set_idx) & n
}

pub fn biquad_low_pass_params_generator(fs: i32, cutoff_frequency: f32)-> BiquadVars{ 
        let Q = 3.0;
        let omega = (2.0*PI*cutoff_frequency)/fs as f32;
        let alpha = omega.sin()*f32::sinh(LOG10_2/2.0*Q*omega/omega.sin());
        
        let a0 = 1.0+alpha;

        let vars = BiquadVars{
            x_1: 0.0,
            x_2: 0.0, 
            y_1: 0.0,
            y_2: 0.0, 
            //gain: 0.0809, 
            a1: (-2.0*omega.cos()) / a0,
            a2: (1.0 - alpha) / a0, 
            b0: ((1.0 - omega.cos())/2.0) / a0,
            b1: (1.0 - omega.cos()) / a0,
            b2: ((1.0 - omega.cos())/2.0) / a0
        };
        vars
}
