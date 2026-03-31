/// Trait rozszerzający dla f64, ułatwiający formatowanie znaków.
pub trait SignStrExt {
    /// Zwraca statyczny string "+" dla liczb >= 0 oraz "-" dla ujemnych.
    fn sign_str(self) -> &'static str;

    /// Zwraca "N" dla wartości >= 0 (Północ) oraz "S" dla ujemnych (Południe).
    fn sign_sn(self) -> &'static str;

    /// Zwraca "E" dla wartości >= 0 (Wschód) oraz "W" dla ujemnych (Zachód).
    fn sign_we(self) -> &'static str;
}

// Makro do błyskawicznej implementacji dla różnych typów liczbowych
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
