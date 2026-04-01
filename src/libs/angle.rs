// 📃 ./src/libs/angle.rs

use std::f64::consts;

use super::frac;

/// 📚 【 POL】: Izolator systemu typów przechowujący wartości kątowe w radianach. Zapobiega błędom jednostkowym.
/// 📚 【 ENG】: Type system insulator storing angular values in radians. Prevents unit mismatch errors.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle(f64);

impl Angle {
	/// 📚 【 POL】: Tworzy instancję Angle bezpośrednio z wartości w radianach (Konstruktor bazowy).
	/// 📚 【 ENG】: Creates an Angle instance directly from a value in radians (Base constructor).
	#[inline]
	pub const fn from_rad(r: f64) -> Self { Self(r) }

	/// 📚 【 POL】: Tworzy instancję Angle z wartości w stopniach, dokonując konwersji na radiany.
	/// 📚 【 ENG】: Creates an Angle instance from a value in degrees, converting it to radians.
	#[inline]
	pub fn from_deg(d: f64) -> Self { Self(d.to_radians()) }

	/// 📚 【 POL】: Tworzy instancję Angle jako wielokrotność liczby PI.
	/// 📚 【 ENG】: Creates an Angle instance as a multiple of PI.
	#[inline]
	pub fn from_pi_frac(fraction: f64) -> Self { Self(fraction * consts::PI) }

	/// 📚 【 POL】: Dekoduje format DMS (Stopnie, Minuty, Sekundy) do reprezentacji radianowej.
	/// 📚 【 ENG】: Decodes DMS (Degrees, Minutes, Seconds) format to radian representation.
	///
	/// ⚠️ 【 POL】: Wykorzystanie i16 dla stopni uniemożliwia reprezentację "-0°" w formacie DMS.
	/// ⚠️ 【 ENG】: Using i16 for degrees prevents representation of "-0°" in DMS format.
	pub fn from_dms(d: i16, m: u8, s: f32) -> Self {
		// Uwaga: prosty 'sign' z d < 0 nie obsłuży "-0 stopni".
		// W GIS często używa się f64 z signum() dla stopnia, ale trzymając się i16:
		let sign = if d < 0 { -1.0_f64 } else { 1.0_f64 };
		let ddd = (d as f64) + sign * ((m as f64) / 60.0) + sign * ((s as f64) / 3600.0);
		Self::from_deg(ddd)
	}

	/// 📚 【 POL】: Dekomponuje kąt z radianów do formatu DMS (Stopnie, Minuty, Sekundy).
	/// 📚 【 ENG】: Decomposes angle from radians to DMS (Degrees, Minutes, Seconds) format.
	pub fn to_dms(self) -> (i16, u8, f32) {
		let ddd = self.deg();
		let sign = ddd.signum();
		let abs_ddd = ddd.abs();

		let d = (abs_ddd.floor() * sign) as i16;
		let m_float = (abs_ddd - abs_ddd.floor()) * 60.0;
		let m = m_float.floor() as u8;
		let s = ((m_float - m_float.floor()) * 60.0) as f32;

		(d, m, s)
	}

	/// 📚 【 POL】: Zwraca wartość kąta w radianach (f64). Operacja bezkosztowa.
	/// 📚 【 ENG】: Returns angle value in radians (f64). Zero-cost operation.
	#[inline]
	pub const fn rad(self) -> f64 { self.0 }

	/// 📚 【 POL】: Zwraca wartość kąta w stopniach (f64).
	/// 📚 【 ENG】: Returns angle value in degrees (f64).
	#[inline]
	pub fn deg(self) -> f64 { self.0.to_degrees() }

	/// 📚 【 POL】: Zwraca kąt jako ułamek liczby PI w postaci (licznik, mianownik).
	/// 📚 【 ENG】: Returns angle as a fraction of PI in (numerator, denominator) form.
	#[inline]
	pub fn pi_frac(self) -> (f64, f64) { frac::as_frac_pi(self.0) }

	/// 📚 【 POL】: Formatuje kąt jako ciąg znaków w radianach. Alokuje String.
	/// 📚 【 ENG】: Formats angle as a string in radians. Allocates String.
	pub fn print_rad(self) -> String { format!("{:.4} rad", self.rad()) }

	/// 📚 【 POL】: Formatuje kąt jako ciąg znaków w stopniach. Alokuje String.
	/// 📚 【 ENG】: Formats angle as a string in degrees. Allocates String.
	pub fn print_deg(self) -> String { format!("{:.2}°", self.deg()) }

	/// 📚 【 POL】: Formatuje kąt jako ułamek liczby PI. Alokuje String.
	/// 📚 【 ENG】: Formats angle as a fraction of PI. Allocates String.
	pub fn print_pi_frac(self) -> String {
		let (num, den) = self.pi_frac();
		if num == 0.0 {
			return "0".to_string();
		}
		if den == 1.0 {
			return format!("{} π", num);
		}
		format!("{} / {} π", num, den)
	}
}

/// 📚 【 POL】: Funkcja pomocnicza (Top-level API) tworząca instancję Angle z wartości w radianach.
/// 📚 【 ENG】: Helper function (Top-level API) creating an Angle instance from a value in radians.
#[rustfmt::skip] #[inline] pub const fn rad(r: f64) -> Angle { Angle::from_rad(r) }

/// 📚 【 POL】: Funkcja pomocnicza (Top-level API) tworząca instancję Angle z wartości w stopniach.
/// 📚 【 ENG】: Helper function (Top-level API) creating an Angle instance from a value in degrees.
#[rustfmt::skip] #[inline] pub fn deg(d: f64) -> Angle { Angle::from_deg(d) }

/// 📚 【 POL】: Funkcja pomocnicza (Top-level API) tworząca instancję Angle jako wielokrotność liczby PI.
/// 📚 【 ENG】: Helper function (Top-level API) creating an Angle instance as a multiple of PI.
#[rustfmt::skip] #[inline] pub fn pi_frac(fraction: f64) -> Angle { Angle::from_pi_frac(fraction) }

/// 📚 【 POL】: Trait rozszerzający dla f64, umożliwiający bezpośrednią konwersję na typ Angle.
/// 📚 【 ENG】: Extension trait for f64, allowing direct conversion to Angle type.
pub trait AngleExt {
	fn deg(self) -> Angle;
	fn rad(self) -> Angle;
	fn pi_frac(self) -> Angle;
}

impl AngleExt for f64 {
	#[rustfmt::skip] #[inline]	fn deg(self) -> Angle { Angle::from_deg(self) }
	#[rustfmt::skip] #[inline]	fn rad(self) -> Angle { Angle::from_rad(self) }
	#[rustfmt::skip] #[inline]	fn pi_frac(self) -> Angle { Angle::from_pi_frac(self) }
}

/// 📚 【 POL】: Definicja dostępnych formatów prezentacji kąta.
/// 📚 【 ENG】: Definition of available angle presentation formats.
#[derive(Debug, Clone, Copy, Default)]
pub enum AngleFmt {
	#[default]
	/// 📚 【 POL】: Stopnie (np. 90.00°)
	/// 📚 【 ENG】: Degrees (e.g., 90.00°)
	Deg,
	/// 📚 【 POL】: Radiany (np. 1.5708 rad)
	/// 📚 【 ENG】: Radians (e.g., 1.5708 rad)
	Rad,
	/// 📚 【 POL】: Ułamki liczby PI (np. 1/2 π)
	/// 📚 【 ENG】: PI Fractions (e.g., 1/2 π)
	PiFrac,
}

impl AngleFmt {
	/// 📚 【 POL】: Formatuje wartość f64 (interpretowaną jako radiany) zgodnie z wybranym formatem.
	/// 📚 【 ENG】: Formats a f64 value (interpreted as radians) according to the selected format.
    #[rustfmt::skip] #[inline]
	pub fn format(&self, val: f64) -> String {
        match self {
            Self::Rad    => Angle::from_rad(val).print_rad(),
			Self::Deg    => Angle::from_rad(val).print_deg(),
			Self::PiFrac => Angle::from_rad(val).print_pi_frac(),
        }
    }
}
