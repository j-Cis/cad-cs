// 📃 ./src/libs/cs/types.rs

use super::model::Cs;

/// 📚 【 POL】: Alias typu dla współrzędnych dwuwymiarowych (płaszczyzna).
/// 📚 【 ENG】: Type alias for two-dimensional coordinates (plane).
pub type Cs2 = Cs<2>;

/// 📚 【 POL】: Alias typu dla współrzędnych trójwymiarowych (przestrzeń).
/// 📚 【 ENG】: Type alias for three-dimensional coordinates (space).
pub type Cs3 = Cs<3>;

/// 📚 【 POL】: Marker trait ograniczający implementację metod wyłącznie do wymiarów N=2 lub N=3.
/// 📚 【 ENG】: Marker trait restricting method implementation exclusively to dimensions N=2 or N=3.
pub trait Dim {}

// Pozwalamy na istnienie geometrii tylko w 2D i 3D
impl Dim for Cs<2> {}
impl Dim for Cs<3> {}
