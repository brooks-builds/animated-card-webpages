pub mod js;

use js::random;

pub fn random_between(min: f64, max: f64) -> f64 {
    (random() * (max - min)) + min
}
