use argmin::{
    core::{CostFunction, Error, Executor},
    solver::brent::BrentOpt,
};
use rand::prelude::*;

const SIMULATION_STEPS: usize = 1_000_000_000;

struct Simulation {}

impl CostFunction for Simulation {
    type Param = f64;
    type Output = f64;

    fn cost(&self, p: &Self::Param) -> Result<Self::Output, Error> {
        let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
        let mut total_score = 0.0;
        for _ in 0..SIMULATION_STEPS {
            let mut score = rng.gen::<f64>();
            while score < *p {
                score += rng.gen::<f64>();
            }
            if score <= 1.0 {
                score += rng.gen::<f64>();
                total_score += score;
            }
        }
        Ok(-total_score / SIMULATION_STEPS as f64)
    }
}

fn main() {
    let cost = Simulation {};
    let solver = BrentOpt::new(0.0, 1.0);
    let solver = solver.set_tolerance(f64::EPSILON.sqrt(), 1e-12);
    let res = Executor::new(cost, solver).run().unwrap();

    println!("{}", res);
}
