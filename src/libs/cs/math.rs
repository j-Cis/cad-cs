// 📃 ./src/libs/cs/math.rs

use std::{
	array,
	ops::{Add, Div, Mul, Neg, Sub},
};

use crate::libs::{
	cs::abstract_traits::{AbstractArithmeticCsGeneric, AbstractMathCsGeneric},
	cs::model::Cs,
	cs::types::Dim,
	tolerance,
};

/// 📚 【 POL】: Specjalizacja 2D. 📚 【 ENG】: 2D Specialization.
pub mod d2;
/// 📚 【 POL】: Specjalizacja 3D. 📚 【 ENG】: 3D Specialization.
pub mod d3;

impl<const N: usize> AbstractMathCsGeneric for Cs<N>
where
	Cs<N>: Dim,
{
	/// 📚 【 POL】: Operacja odejmowania wektorów. Zwraca wektor różnicy (B - A).
	/// 📚 【 ENG】: Vector subtraction. Returns the difference vector (B - A).
    #[rustfmt::skip] #[inline]
	fn sub(&self, other: &Self) -> Self {
		Cs(array::from_fn(|i| self.0[i] - other.0[i]))
	}

	/// 📚 【 POL】: Operacja dodawania wektorów. Zwraca wektor sumy (A + B).
	/// 📚 【 ENG】: Vector addition. Returns the sum vector (A + B).
    #[rustfmt::skip] #[inline]
	fn add(&self, other: &Self) -> Self {
		Cs(array::from_fn(|i| self.0[i] + other.0[i]))
	}

	/// 📚 【 POL】: Iloczyn skalarny (Dot product) dla dowolnego wymiaru N.
	/// 📚 【 ENG】: Dot product for any dimension N.
    #[rustfmt::skip] #[inline]
	fn dot(&self, other: &Self) -> f64 {
		let mut sum = 0.0;
		for i in 0..N { sum += self.0[i] * other.0[i]; }
		sum
	}

	/// 📚 【 POL】: Kwadrat pełnej długości wektora (R²). Eliminuje kosztowne pierwiastkowanie.
	/// 📚 【 ENG】: Squared full length of the vector (R²). Eliminates costly square root.
    #[rustfmt::skip] #[inline]
	fn r_sq(&self) -> f64 { self.dot(self) }

	/// 📚 【 POL】: Pełna długość wektora w przestrzeni (Norma Euklidesowa R).
	/// 📚 【 ENG】: Full length of the vector in space (Euclidean Norm R).
    #[rustfmt::skip] #[inline]
	fn r(&self) -> f64 { self.r_sq().sqrt() }

	/// 📚 【 POL】: Normalizuje wektor do długości jednostkowej (1.0). Zwraca wektor zerowy przy zerowej długości.
	/// 📚 【 ENG】: Normalizes the vector to unit length (1.0). Returns a zero vector for zero length.
    #[rustfmt::skip] #[inline]
	fn normalize_r_projection(&self) -> Self {
		let radius = self.r();
		if tolerance::is_zero(radius) {
			Cs([0.0; N])
		} else {
			Cs(array::from_fn(|i| self.0[i] / radius))
		}
	}

	/// 📚 【 POL】: Oblicza kąt między dwoma wektorami w radianach.
	/// 📚 【 ENG】: Calculates the angle between two vectors in radians.
    #[rustfmt::skip] #[inline]
	fn angle_between(&self, other: &Self) -> f64 {
		let r1 = self.r();
		let r2 = other.r();
		if tolerance::is_zero(r1) || tolerance::is_zero(r2) {
			0.0
		} else {
			(self.dot(other) / (r1 * r2)).clamp(-1.0, 1.0).acos()
		}
	}
}

// =============================================================================
// IMPLEMENTACJA OPERATORÓW
// =============================================================================

/// 📚 【 POL】: Implementacja operatora dodawania (Cs + Cs).
/// 📚 【 ENG】: Implementation of the addition operator (Cs + Cs).
impl<const N: usize> Add for Cs<N>
where
	Cs<N>: Dim,
{
	type Output = Cs<N>;
	#[inline]
	fn add(self, rhs: Self) -> Self::Output {
		// 📌 Cs(array::from_fn(|i| self.0[i] + rhs.0[i]))
		AbstractMathCsGeneric::add(&self, &rhs)
	}
}

/// 📚 【 POL】: Implementacja operatora odejmowania (Cs - Cs).
/// 📚 【 ENG】: Implementation of the subtraction operator (Cs - Cs).
impl<const N: usize> Sub for Cs<N>
where
	Cs<N>: Dim,
{
	type Output = Cs<N>;
	#[inline]
	fn sub(self, rhs: Self) -> Self::Output {
		// 📌 Cs(array::from_fn(|i| self.0[i] - rhs.0[i]))
		AbstractMathCsGeneric::sub(&self, &rhs)
	}
}

/// 📚 【 POL】: Implementacja operatora negacji (-Cs).
/// 📚 【 ENG】: Implementation of the negation operator (-Cs).
impl<const N: usize> Neg for Cs<N>
where
	Cs<N>: Dim,
{
	type Output = Cs<N>;
	#[inline]
	fn neg(self) -> Self::Output {
		Cs(array::from_fn(|i| -self.0[i]))
		// ⁉️ AbstractMathCsGeneric::
	}
}

/// 📚 【 POL】: Implementacja mnożenia wektora przez skalar (Cs * f64).
/// 📚 【 ENG】: Implementation of vector multiplication by a scalar (Cs * f64).
impl<const N: usize> Mul<f64> for Cs<N>
where
	Cs<N>: Dim,
{
	type Output = Cs<N>;
	#[inline]
	fn mul(self, rhs: f64) -> Self::Output {
		Cs(array::from_fn(|i| self.0[i] * rhs))
		// ⁉️ AbstractMathCsGeneric::
	}
}

/// 📚 【 POL】: Implementacja mnożenia skalara przez wektor (f64 * Cs).
/// 📚 【 ENG】: Implementation of scalar multiplication by a vector (f64 * Cs).
impl<const N: usize> Mul<Cs<N>> for f64
where
	Cs<N>: Dim,
{
	type Output = Cs<N>;
	#[inline]
	fn mul(self, rhs: Cs<N>) -> Self::Output {
		rhs * self
	}
}

/// 📚 【 POL】: Implementacja dzielenia wektora przez skalar (Cs / f64).
/// 📚 【 ENG】: Implementation of vector division by a scalar (Cs / f64).
impl<const N: usize> Div<f64> for Cs<N>
where
	Cs<N>: Dim,
{
	type Output = Cs<N>;
	#[inline]
	fn div(self, rhs: f64) -> Self::Output {
		// let mut result = [0.0; N];
		// for i in 0..N { result[i] = self.0[i] / rhs; }
		// Cs(result)
		Cs(array::from_fn(|i| self.0[i] / rhs))
		// ⁉️ AbstractMathCsGeneric::
	}
}
// =============================================================================
// PODPIĘCIE ABSTRAKTU ARYTMETYCZNEGO POD OPERATORY POWYŻSZE
// =============================================================================

impl<const N: usize> AbstractArithmeticCsGeneric for Cs<N> 
where Cs<N>: Dim 
{
    #[inline] fn add_cs(&self, rhs: &Self) -> Self { AbstractMathCsGeneric::add(self, rhs) }
    #[inline] fn sub_cs(&self, rhs: &Self) -> Self { AbstractMathCsGeneric::sub(self, rhs) }
    #[inline] fn neg_cs(&self) -> Self { -*self }
    #[inline] fn mul_scalar(&self, rhs: f64) -> Self { *self * rhs }
    #[inline] fn div_scalar(&self, rhs: f64) -> Self { *self / rhs }
}