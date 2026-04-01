// src/libs/cs/helper.rs

use crate::libs::angle::{AngleExt, AngleFmt};

pub mod d2;
pub mod d3;

/// 📚 【 POL】: Wewnętrzny helper do formatowania wartości kątowych zgodnie z zadanym enum AngleFmt.
/// 📚 【 ENG】: Internal helper for formatting angular values according to the specified AngleFmt enum.
#[rustfmt::skip] #[inline]
pub(crate) fn print_fmt_ang(val: f64, fmt: AngleFmt) -> String {
	match fmt {
		AngleFmt::Rad => val.rad().print_rad(),
		AngleFmt::Deg => val.rad().print_deg(),
		AngleFmt::PiFrac => val.rad().print_pi_frac(),
	}
}
