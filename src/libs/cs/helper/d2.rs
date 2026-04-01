// 📃 ./src/libs/cs/helper/d2.rs

use crate::libs::cs::abstract_traits::{AbstractHelperCs2, AbstractMathCs2};
use crate::libs::{angle::AngleFmt, cs::model::Cs};
// ===================================================================================
// TRAIT DLA 2D
// ===================================================================================

impl AbstractHelperCs2 for Cs<2> {
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
		println!(" {} 🟪 (R: {:?}, Φ: {})", name, rf[0], fmt.format(rf[1]));
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
