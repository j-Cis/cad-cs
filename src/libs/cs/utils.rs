
/// Trait rozszerzający dla f64, ułatwiający formatowanie znaków.
pub trait SignStrExt {
    /// Zwraca statyczny string "+" dla liczb >= 0.0 oraz "-" dla ujemnych.
    fn sign_str(self) -> &'static str;
}

impl SignStrExt for f64 {
    #[inline] fn sign_str(self) -> &'static str { if self >= 0.0 { "+" } else { "-" } }
}