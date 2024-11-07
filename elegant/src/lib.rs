#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ptr;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn connectome_setup() {
        let mut connectome = Connectome {
            neuron_state: ptr::null_mut(),
            muscle_state: ptr::null_mut(),
            _neurons_tot: 0,
            _muscles_tot: 0,
            _neuron_current: ptr::null_mut(),
            _muscle_current: ptr::null_mut(),
            _neuron_next: ptr::null_mut(),
            _muscle_next: ptr::null_mut(),
            _meta: ptr::null_mut()
        };

        let chemotaxis:[u16; 8] = [
            N_ADFL as u16, N_ADFR as u16, N_ASGR as u16, N_ASGL as u16, N_ASIL as u16, N_ASIR as u16,
            N_ASJR as u16, N_ASJL as u16
        ];

        unsafe {
            ctm_init(&mut connectome);

            for _ in 0..1000 {
                ctm_neural_cycle(&mut connectome, chemotaxis.as_ptr(), 8);
            }

            println!("1000 round warmup complete");
        };
    }
}