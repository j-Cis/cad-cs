// 📃 ./src/libs/angle.rs

pub mod abstracts;
pub mod math;
pub mod model;

// Udostępniamy użytkownikowi modele i kontrakty od razu
pub use abstracts::*;
pub use math::{deg, pi_frac, rad};
pub use model::*;
