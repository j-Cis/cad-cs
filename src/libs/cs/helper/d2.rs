// src/libs/cs/helper/d2.rs

use crate::libs::angle::AngleFmt;
use crate::libs::cs::model::Cs;
use super::print_fmt_ang;

// ===================================================================================
// TRAIT DLA 2D
// ===================================================================================

/// 📚 【 POL】: Trait rozszerzający dla Cs<2>, umożliwiający formatowanie danych wyjściowych do konsoli.
/// 📚 【 ENG】: Extension trait for Cs<2>, enabling output formatting to the console.
pub trait Cs2ConsoleDebug {
	/// 📚 【 POL】: Wyświetla informację o ćwiartce i znakach składowych.
	/// 📚 【 ENG】: Displays quadrant information and component signs.
	fn print_q(&self, name: &str);

	/// 📚 【 POL】: Wyświetla współrzędne kartezjańskie (X, Y).
	/// 📚 【 ENG】: Displays Cartesian coordinates (X, Y).
	fn print_xy(&self, name: &str);

	/// 📚 【 POL】: Wyświetla współrzędne biegunowe (R, Φ).
	/// 📚 【 ENG】: Displays polar coordinates (R, Φ).
	fn print_rf(&self, name: &str, fmt: AngleFmt);

	/// 📚 【 POL】: Wyświetla zbiorczy raport debugowania dla wektora 2D.
	/// 📚 【 ENG】: Displays a summary debug report for the 2D vector.
	fn print(&self, name: &str, fmt: AngleFmt);
}

impl Cs2ConsoleDebug for Cs<2> {
	#[rustfmt::skip] #[inline]
	fn print_q(&self, name: &str) {
	let s = self.q_sign();
	println!(" {}    (Ćwiartka: {} [{}, {}])", name, self.q(), s[0], s[1]);
	}

	#[rustfmt::skip] #[inline]
	fn print_xy(&self, name: &str) {
		println!(" {} 🔷 (x: {:?}, y: {:?})", name, self.0[0], self.0[1]);
	}

	#[rustfmt::skip] #[inline]
	fn print_rf(&self, name: &str, fmt: AngleFmt) {
		let rf = self.to_rf_from_xy();
		println!(" {} 🟪 (R: {:?}, Φ: {})", name, rf[0], print_fmt_ang(rf[1], fmt));
	}

	#[rustfmt::skip] #[inline]
	fn print(&self, name: &str, fmt: AngleFmt) {
		let spacer = " ".repeat(name.chars().count());
		self.print_xy(name);
		self.print_q(&spacer);
		self.print_rf(&spacer, fmt);
		println!(" ");
	}
}