// src/libs/cs.rs

// 1. Rejestrujemy wszystkie prywatne/publiczne podmoduły
pub mod core;
pub mod debug_print;
pub mod math;
pub mod model;
pub mod model_coords;
pub mod utils;

// 2. Wyciągamy na wierzch to, co ma być widoczne dla użytkownika końcowego
// (to "spłaszcza" Twoją skomplikowaną architekturę folderów na zewnątrz)
pub use model::{Cs, Cs2, Cs3, Dim};
pub use model_coords::*;
pub use utils::SignStrExt;

// (Metody przypięte przez bloki 'impl Cs' z core.rs, math.rs, d2.rs, d3.rs
// ładują się automatycznie dla kompilatora, gdy tylko załadujemy moduł.
// Nie trzeba ich eksportować przez 'pub use'!)
