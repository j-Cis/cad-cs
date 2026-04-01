// 📃 ./src/libs/cs/macros.rs

/*/
 * // Makro generujące Cs<N>
 * #[macro_export]
 * macro_rules! cs {
 *     ($($x:expr),* $(,)?) => {
 *         $crate::libs::cs::Cs([$($x as f64),*])
 *     };
 * }
 */

/// 📚 【 POL 】: Konstruktor makrowy dla `Cs<N>`. Obsługuje rzutowanie typów numerycznych na f64.
/// 📚 【 ENG 】: Macro constructor for `Cs<N>`. Handles casting of numerical types to f64.
///
/// # Examples: `cs![1, 2]` (Cs2), `cs![1, 2, 3]` (Cs3)
#[macro_export]
macro_rules! cs {
	// Dopasowanie dokładnie dla 2 elementów
	($x:expr, $y:expr $(,)?) => {
		$crate::libs::cs::model::Cs([$x as f64, $y as f64])
	};
	// Dopasowanie dokładnie dla 3 elementów
	($x:expr, $y:expr, $z:expr $(,)?) => {
		$crate::libs::cs::model::Cs([$x as f64, $y as f64, $z as f64])
	};
}

/// 📚 【 POL 】: Konstruktor makrowy tworzący instancję Angle z formatu DMS.
/// 📚 【 ENG 】: Macro constructor creating an Angle instance from DMS format.
#[macro_export]
macro_rules! dms_angle {
	($d:expr, $m:expr, $s:expr) => {
		$crate::libs::angle::Angle::from_dms($d as i16, $m as u8, $s as f32)
	};
}

/// 📚 【 POL 】: Konwertuje format DMS bezpośrednio do wartości f64 w radianach.
/// 📚 【 ENG 】: Converts DMS format directly to a f64 value in radians.
#[macro_export]
macro_rules! dms {
	($d:expr, $m:expr, $s:expr) => {
		$crate::libs::angle::Angle::from_dms($d as i16, $m as u8, $s as f32).rad()
	};
}

/// 📚 【 POL】: Makro implementujące trait AbstractSignStrExt dla wskazanych typów liczbowych i zdefiniowanego punktu zera.
/// 📚 【 ENG】: Macro implementing the AbstractSignStrExt trait for specified numerical types and a defined zero point.
#[macro_export]
macro_rules! impl_sign_str {
	($t:ty, $zero:expr) => {
		impl $crate::libs::cs::abstract_traits::AbstractSignStrExt for $t {
			#[inline]
			fn sign_str(self) -> &'static str { if self >= $zero { "+" } else { "-" } }
			#[inline]
			fn sign_sn(self) -> &'static str { if self >= $zero { "N" } else { "S" } }
			#[inline]
			fn sign_we(self) -> &'static str { if self >= $zero { "E" } else { "W" } }
		}
	};
}
