// src/libs/cs.rs

// 1. Rejestrujemy wszystkie prywatne/publiczne podmoduły
pub mod model;
pub mod model_coords;
pub mod utils;
pub mod core;
pub mod math;
pub mod debug_print;

// 2. Wyciągamy na wierzch to, co ma być widoczne dla użytkownika końcowego
// (to "spłaszcza" Twoją skomplikowaną architekturę folderów na zewnątrz)
pub use model::{Cs, Cs2, Cs3, Dim};
pub use model_coords::*;
pub use utils::SignStrExt;

// (Metody przypięte przez bloki 'impl Cs' z core.rs, math.rs, d2.rs, d3.rs 
// ładują się automatycznie dla kompilatora, gdy tylko załadujemy moduł. 
// Nie trzeba ich eksportować przez 'pub use'!)


// ===================================================================================
// LEGENDA / KLUCZE SYMBOLI W Cs<N>
// ===================================================================================
//
// 📖 Ćwiartki i oktanty:
//
// - q       : numer ćwiartki (2D) lub oktantu (3D) w układzie kartezjańskim
//             2D: płaszczyzna XY, wartości 1-4:
//                  1: (+X, +Y), 2: (-X, +Y), 3: (-X, -Y), 4: (+X, -Y)
//             3D: przestrzeń XYZ, wartości 1-8:
//                  1: (+X, +Y, +Z), 2: (-X, +Y, +Z), 3: (-X, -Y, +Z), 4: (+X, -Y, +Z),
//                  5: (+X, +Y, -Z), 6: (-X, +Y, -Z), 7: (-X, -Y, -Z), 8: (+X, -Y, -Z)
//
// - q_sign  : alternatywna reprezentacja wizualna i logiczna dla ćwiartki/oktantu.
//             Zwraca tablicę znaków kierunkowych osi ["+", "-"] w 2D lub ["+", "-", "+"] w 3D
//             np. dla 2D punktu (-1.0, 2.0) → q = 2, q_sign = ["-", "+"]
//             np. dla 3D punktu (-1.0, -2.0, 3.0) → q = 3, q_sign = ["-", "-", "+"]
//
// 📖 Współrzędne (Oraz ich bezpieczne odpowiedniki DTO):
//
// - xy  (Coords2dXy)      : współrzędne 2D kartezjańskie (x,y) lub lub wektor (x,y) lub płaszczyzna XY 
// - xz  (Coords2dXz)      : współrzędne 2D kartezjańskie (x,z) lub lub wektor (x,z) lub płaszczyzna XZ 
// - yz  (Coords2dYz)      : współrzędne 2D kartezjańskie (y,z) lub lub wektor (y,z) lub płaszczyzna YZ 
// - rf  (Coords2dXyPolar) : współrzędne 2D biegunowe/cylindryczne (R, Φ)
// - rf  (Coords2dXzPolar) : współrzędne 2D biegunowe/cylindryczne (R, Φ)
// - rf  (Coords2dZyPolar) : współrzędne 2D biegunowe/cylindryczne (R, Φ)
// - rfx (Coords3dYzCylindricalX) : współrzędne 3D cylindryczne względem osi X (R_yz, Φ, X) 
// - rfy (Coords3dXzCylindricalY) : współrzędne 3D cylindryczne względem osi Y (R_xz, Φ, Y) 
// - rfz (Coords3dXyCylindricalZ) : współrzędne 3D cylindryczne względem osi Z (R_xy, Φ, Z) 
// - xyz (Coords3dXyz)            : współrzędne 3D kartezjańskie (x,y,z) lub lub wektor (x,y,z) lub przestrzeń XYZ
// - rft (Coords3dXyzSpherical)   : współrzędne 3D sferyczne (R, Φ, Θ)
//
// 📖 Współrzędne / Kąty (azymuty / inklinacje):
//
// - f (f_yx/f_zx/f_zy) : współrzędna 2D/3D (Φ) – kąt azymutu 
//                                                  w płaszczyźnie XY/XZ/YZ lub 
//                                                  w układach sferycznych RΦΘ lub
//                                                  w ukłądach cylindrycznych RΦX/RΦY/RΦZ.
// - t (t_zr) : współrzędna 3D (Θ) – kąt inklinacji w układzie sferycznym od osi Z/Y/X
//
// - y_x    (f_yx) : arctan(y/x) – kąt/azymut w płaszczyźnie XY
// - z_x    (f_zx) : arctan(z/x) – kąt/azymut w płaszczyźnie XZ
// - z_y    (f_zy) : arctan(z/y) – kąt/azymut w płaszczyźnie YZ
// - x_rxyz (t_xr) : arccos(x/rxyz) – kąt/inklinacja względem osi X
// - y_rxyz (t_yr) : arccos(y/rxyz) – kąt/inklinacja względem osi Y
// - z_rxyz (t_zr) : arccos(z/rxyz) – kąt/inklinacja względem osi Z
//
// 📖 Współrzędne / Wektory, promienie, długości cząstkowe (projekcja na płaszczyzne):
//
// - {x, y, z}: współrzędna 2D/3D kartezjańska
// - r    : współrzędna 2D/3D biegunowe/sferyczne/kartezjańkie - długość/wektor (promień) w 2D lub 3D
// - rxy  : długość rzutu wektora na 2D płaszczyznę XY
// - rxz  : długość rzutu wektora na 2D płaszczyznę XZ
// - ryz  : długość rzutu wektora na 2D płaszczyznę YZ
// - rxyz : długość wektor w 3D przestrzeni XYZ
//
//
// 📖 Operacje wektorowe / pomocnicze:
//
// - add, sub : dodawanie / odejmowanie wektorów
// - dot      : iloczyn skalarny
// - cross    : iloczyn wektorowy (tylko w 3D)
// - perp     : wektor prostopadły (tylko w 2D)
// - normalize_r_projection    : normalizacja rzutu wektora na płaszczyznę
// - angle_between             : kąt między dwoma wektorami
//
// ===================================================================================


