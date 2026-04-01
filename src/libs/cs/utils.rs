// src/libs/cs/utils.rs

/// 📚 【 POL】: Trait rozszerzający dla typów liczbowych, ułatwiający prezentację znaków kierunkowych.
/// 📚 【 ENG】: Extension trait for numerical types, facilitating the presentation of directional signs.
pub trait SignStrExt {
	/// 📚 【 POL】: Zwraca "+" dla wartości nieujemnych oraz "-" dla ujemnych.
    /// 📚 【 ENG】: Returns "+" for non-negative values and "-" for negative ones.
    fn sign_str(self) -> &'static str;

	/// 📚 【 POL】: Zwraca "N" (Północ) dla wartości nieujemnych oraz "S" (Południe) dla ujemnych.
    /// 📚 【 ENG】: Returns "N" (North) for non-negative values and "S" (South) for negative ones.
    fn sign_sn(self) -> &'static str;

	/// 📚 【 POL】: Zwraca "E" (Wschód) dla wartości nieujemnych oraz "W" (Zachód) dla ujemnych.
    /// 📚 【 ENG】: Returns "E" (East) for non-negative values and "W" (West) for negative ones.
    fn sign_we(self) -> &'static str;
}

/// 📚 【 POL】: Makro implementujące trait SignStrExt dla wskazanych typów liczbowych i zdefiniowanego punktu zera.
/// 📚 【 ENG】: Macro implementing the SignStrExt trait for specified numerical types and a defined zero point.
macro_rules! impl_sign_str {
	($t:ty, $zero:expr) => {
		impl SignStrExt for $t {
			#[inline]
			fn sign_str(self) -> &'static str {
				if self >= $zero { "+" } else { "-" }
			}
			#[inline]
			fn sign_sn(self) -> &'static str {
				if self >= $zero { "N" } else { "S" }
			}
			#[inline]
			fn sign_we(self) -> &'static str {
				if self >= $zero { "E" } else { "W" }
			}
		}
	};
}

// Implementujemy trait dla f64 (rdzeń matematyczny) oraz i8/i16 (struktury DMS)
impl_sign_str!(f64, 0.0);
impl_sign_str!(i8, 0);
impl_sign_str!(i16, 0);

// impl SignStrExt for f64 {
//     #[inline] fn sign_str(self) -> &'static str { if self >= 0.0 { "+" } else { "-" } }
// }
