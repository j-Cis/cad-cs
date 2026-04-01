// 📃 ./src/libs/cs/model.rs

use std::ops::Deref;

/// 📚 【 POL】: Generyczny wrapper dla tablic o stałym rozmiarze, ograniczony do wymiarów 2D i 3D.
/// 📚 【 ENG】: Generic wrapper for fixed-size arrays, restricted to 2D and 3D dimensions.
///
/// ⚠️ 【 POL】: Implementacja ściśle ograniczona do N=2 lub N=3. Inne wymiary nie są wspierane.
/// ⚠️ 【 ENG】: Implementation strictly limited to N=2 or N=3. Other dimensions are not supported.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cs<const N: usize>(pub [f64; N]);

/// 📚 【 POL】: Alias typu dla współrzędnych dwuwymiarowych (płaszczyzna).
/// 📚 【 ENG】: Type alias for two-dimensional coordinates (plane).
pub type Cs2 = Cs<2>;

/// 📚 【 POL】: Alias typu dla współrzędnych trójwymiarowych (przestrzeń).
/// 📚 【 ENG】: Type alias for three-dimensional coordinates (space).
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

/// 📚 【 POL】: Konstruktor makrowy dla `Cs<N>`. Obsługuje rzutowanie typów numerycznych na f64.
/// 📚 【 ENG】: Macro constructor for `Cs<N>`. Handles casting of numerical types to f64.
///
/// # Examples: `cs![1, 2]` (Cs2), `cs![1, 2, 3]` (Cs3)
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

/// 📚 【 POL】: Konstruktor makrowy tworzący instancję Angle z formatu DMS.
/// 📚 【 ENG】: Macro constructor creating an Angle instance from DMS format.
#[macro_export]
macro_rules! dms_angle {
	($d:expr, $m:expr, $s:expr) => {
		$crate::libs::angle::Angle::from_dms($d as i16, $m as u8, $s as f32)
	};
}

/// 📚 【 POL】: Konwertuje format DMS bezpośrednio do wartości f64 w radianach.
/// 📚 【 ENG】: Converts DMS format directly to a f64 value in radians.
#[macro_export]
macro_rules! dms {
	($d:expr, $m:expr, $s:expr) => {
		$crate::libs::angle::Angle::from_dms($d as i16, $m as u8, $s as f32).rad()
	};
}

/// 📚 【 POL】: Marker trait ograniczający implementację metod wyłącznie do wymiarów N=2 lub N=3.
/// 📚 【 ENG】: Marker trait restricting method implementation exclusively to dimensions N=2 or N=3.
pub trait Dim {}

// Pozwalamy na istnienie geometrii tylko w 2D i 3D
impl Dim for Cs<2> {}
impl Dim for Cs<3> {}

/// 📚 【 POL】: Implementacja Deref zapewniająca bezpośredni dostęp do wewnętrznej tablicy danych.
/// 📚 【 ENG】: Deref implementation providing direct access to the internal data array.
impl<const N: usize> Deref for Cs<N>
where Cs<N>: Dim
{
	type Target = [f64; N];
	#[inline]
	fn deref(&self) -> &Self::Target { &self.0 }
}
