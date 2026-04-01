// 📃 ./src/libs/cs/core.rs

/// 📚 【 POL】: Implementacje konwersji 2D. 📚 【 ENG】: 2D conversion impls.
pub mod d2;
/// 📚 【 POL】: Implementacje konwersji 3D. 📚 【 ENG】: 3D conversion impls.
pub mod d3;

use super::abstract_traits::AbstractModelCsGeneric;
use super::model::Cs;
use super::types::Dim;
use std::ops::Deref;

/// 📚 【 POL】: Implementacja Deref zapewniająca bezpośredni dostęp do wewnętrznej tablicy danych.
/// 📚 【 ENG】: Deref implementation providing direct access to the internal data array.
impl<const N: usize> Deref for Cs<N>
where
	Cs<N>: Dim,
{
	type Target = [f64; N];
	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

/// 📚 【 POL】: Implementacja kontraktu bazowego (przeniesiona z model.rs).
impl<const N: usize> AbstractModelCsGeneric<N> for Cs<N>
where
	Cs<N>: Dim,
{
	#[inline]
	fn new(data: [f64; N]) -> Self {
		Cs(data)
	}
	#[inline]
	fn origin() -> Self {
		Cs([0.0; N])
	}
	#[inline]
	fn as_slice(&self) -> &[f64] {
		&self.0
	}
}
