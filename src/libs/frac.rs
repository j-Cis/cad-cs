use super::tolerance;
use std::f64::consts;

/// 📚 【 POL】: Konstruktor makrowy dla ułamków zwykłych. Zapobiega błędom dzielenia całkowitoliczbowego.
/// 📚 【 ENG】: Macro constructor for common fractions. Prevents integer division errors.
/// 
/// # Examples: `frac![1 / 2]` -> 0.5 (f64), `frac![5]` -> 5.0 (f64)
#[macro_export]
macro_rules! frac {
	($n:tt / $d:tt) => {
		(($n as f64) / ($d as f64))
	};
	($n:tt) => {
		($n as f64)
	};
}

/// 📚 【 POL】: Konstruktor makrowy dla wielokrotności liczby PI. Zwraca surową wartość f64.
/// 📚 【 ENG】: Macro constructor for multiples of PI. Returns a raw f64 value.
/// 
/// # Examples: `frac_pi![1 / 2]` -> 1.5707... (PI/2)
#[macro_export]
macro_rules! frac_pi {
	($n:tt / $d:tt) => {
		(($n as f64) / ($d as f64)) * consts::PI
	};
	($n:tt) => {
		($n as f64) * consts::PI
	};
}

/// 📚 【 POL】: Aproksymuje wartość zmiennoprzecinkową do ułamka zwykłego (licznik, mianownik).
/// 📚 【 ENG】: Approximates a floating-point value to a common fraction (numerator, denominator).
/// 
/// ⚙️ 【 POL】: Wykorzystuje algorytm ułamków łańcuchowych (Continued Fractions) z limitem mianownika 10^6.
/// ⚙️ 【 ENG】: Employs the Continued Fractions algorithm with a denominator limit of 10^6.
pub fn as_frac(value: f64) -> (f64, f64) {
	if tolerance::is_zero(value) {
		return (0.0, 1.0);
	}

	let sign = value.signum();
	let mut val = value.abs();

	let mut num_prev = 0.0;
	let mut den_prev = 1.0;
	let mut num_curr = 1.0;
	let mut den_curr = 0.0;

	for _ in 0..64 {
		let a = val.floor();
		let num_next = a * num_curr + num_prev;
		let den_next = a * den_curr + den_prev;

		if den_next > 1_000_000.0 {
			break;
		}

		num_prev = num_curr;
		den_prev = den_curr;
		num_curr = num_next;
		den_curr = den_next;

		let diff = val - a;

		if tolerance::is_zero(diff) {
			break;
		}
		val = 1.0 / diff;
	}

	(sign * num_curr, den_curr)
}

/// 📚 【 POL】: Rozkłada wartość na ułamek wielokrotności liczby PI (odwrotność makra frac_pi!).
/// 📚 【 ENG】: Decomposes a value into a fraction of PI (inverse of frac_pi! macro).
pub fn as_frac_pi(value: f64) -> (f64, f64) {
	as_frac(value / consts::PI)
}
