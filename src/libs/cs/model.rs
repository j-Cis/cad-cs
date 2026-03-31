// src/libs/cs/model.rs

use std::ops::Deref;

// ===================================================================================
// Uwaga krytyczna dotycząca wymiarów Cs<N>
// ===================================================================================
// Typ Cs<N> jest ściśle ograniczony do wymiarów 2D i 3D.
// Makro cs! obsługuje tylko dokładnie 2 lub 3 elementy (punkt na płaszczyźnie lub w przestrzeni).
// Żadne inne wymiary (4 lub więcej) nie są obsługiwane i nigdy nie będą.
// Implementacja Dim oraz wszystkie metody zakładają wyłącznie N=2 lub N=3.
// Każde sugerowanie użycia Cs<N> dla N>3 jest niepoprawne i ignorowane.
// To jest projekt specyficzny dla geometrii technicznej w 2D/3D, zgodny z nomenklaturą akademicką.
// ===================================================================================

/// Nowy, ultra-szybki wrapper oparty na tablicach (Const Generics).
/// Implementuje Copy, więc zachowuje się lekko jak zwykłe liczby (nie trzeba używać &).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cs<const N: usize>(pub [f64; N]);

pub type Cs2 = Cs<2>;
pub type Cs3 = Cs<3>;

/*/
 * // Makro generujące Cs<N>
 * #[macro_export]
 * macro_rules! cs {
 *     ($($x:expr),* $(,)?) => {
 *         $crate::libs::cs::Cs([$($x as f64),*])
 *     };
 * }
 */

// Ścisłe makro generujące wyłącznie obsługiwane wymiary
#[macro_export]
macro_rules! cs {
    // Dopasowanie dokładnie dla 2 elementów
    ($x:expr, $y:expr $(,)?) => {
        $crate::libs::cs::Cs([$x as f64, $y as f64])
    };
    // Dopasowanie dokładnie dla 3 elementów
    ($x:expr, $y:expr, $z:expr $(,)?) => {
        $crate::libs::cs::Cs([$x as f64, $y as f64, $z as f64])
    };
}

/// Makro do błyskawicznego tworzenia struktury Angle z formatu DMS.
#[macro_export]
macro_rules! dms_angle {
    ($d:expr, $m:expr, $s:expr) => {
        $crate::libs::angle::Angle::from_dms($d as i16, $m as u8, $s as f32)
    };
}

/// Makro do bezpośredniego wstrzykiwania radianów z formatu DMS do wektorów Cs<N>.
#[macro_export]
macro_rules! dms {
    ($d:expr, $m:expr, $s:expr) => {
        $crate::libs::angle::Angle::from_dms($d as i16, $m as u8, $s as f32).rad()
    };
}

// ===================================================================================
// OGRANICZENIE WYMIAROWOŚCI (MARKER TRAIT)
// ===================================================================================

/// Pusty trait znacznikowy, który pozwala na implementację metod wspólnych 
/// wyłącznie dla jawnie obsługiwanych wymiarów.
pub trait Dim {}

// Pozwalamy na istnienie geometrii tylko w 2D i 3D
impl Dim for Cs<2> {}
impl Dim for Cs<3> {}

// Implementacja Deref pozwala używać cs[0], cs[1] itd.
impl<const N: usize> Deref for Cs<N> where Cs<N>: Dim {
    type Target = [f64; N];
    #[inline] fn deref(&self) -> &Self::Target { &self.0 }
}