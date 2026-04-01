// 📃 ./src/libs/cs/utils.rs

// 📚 【 POL】: Implementujemy trait dla f64 (rdzeń) oraz i8/i16 (DMS).
// 📚 【 ENG】: Implementing trait for f64 (core) and i8/i16 (DMS).
crate::impl_sign_str!(f64, 0.0);
crate::impl_sign_str!(i8, 0);
crate::impl_sign_str!(i16, 0);

// impl AbstractSignStrExt for f64 {
//     #[inline] fn sign_str(self) -> &'static str { if self >= 0.0 { "+" } else { "-" } }
// }
