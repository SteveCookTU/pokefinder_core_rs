use autocxx::prelude::*;
use crate::ffi::Method;

include_cpp! {
    #include "Core/Enum/Method.hpp"
    #include "Core/Gen3/States/PIDToIVState.hpp"
    #include "Core/Gen3/Tools/PIDToIVCalculator.hpp"
    safety!(unsafe)
    generate!("Method")
    generate!("PIDToIVState")
    generate!("PIDToIVCalculator::calculateIVs")
}

pub struct PIDToIVState {
    seed: u32,
    ivs: [u8; 6],
    method: u8
}

pub fn calculate_ivs(pid: u32) -> Vec<PIDToIVState> {
    let test = ffi::PIDToIVCalculator::calculateIVs(c_uint(pid));
    test.iter().map(|state| PIDToIVState {
        seed: state.getSeed().0,
        ivs: [state.getIV(0), state.getIV(1), state.getIV(2), state.getIV(3), state.getIV(4), state.getIV(5)],
        method: state.getMethod() as u8,
    }).collect()
}