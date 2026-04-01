// 📃 src/libs/angle/abstracts.rs
use super::model::Angle;

pub trait AbstractAngle {
	fn from_rad(r: f64) -> Self;
	fn from_deg(d: f64) -> Self;
	fn from_pi_frac(fraction: f64) -> Self;
	fn from_dms(d: i16, m: u8, s: f32) -> Self;
	fn to_dms(self) -> (i16, u8, f32);
	fn rad(self) -> f64;
	fn deg(self) -> f64;
	fn pi_frac(self) -> (f64, f64);
	fn print_rad(self) -> String;
	fn print_deg(self) -> String;
	fn print_pi_frac(self) -> String;
}

/// 📚 【 POL】: Trait rozszerzający dla f64, umożliwiający bezpośrednią konwersję na typ Angle.
/// 📚 【 ENG】: Extension trait for f64, allowing direct conversion to Angle type.
pub trait AngleExt {
	fn deg(self) -> Angle;
	fn rad(self) -> Angle;
	fn pi_frac(self) -> Angle;
}
