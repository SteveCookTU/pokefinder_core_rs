use pokefinder_core_rs::{calculate_ivs, PIDToIVState};

fn main() {
    let results = calculate_ivs(0);
    for PIDToIVState { seed, ivs, method } in results {
        println!("Seed: {seed} IVs: {ivs} Method: {method}");
    }
}