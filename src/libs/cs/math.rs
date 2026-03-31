// src/libs/cs/math.rs
use crate::libs::cs::model::{Cs, Dim};
use crate::libs::tolerance;
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::array;

pub mod d2;
pub mod d3;



// ===================================================================================
// MATEMATYKA WEKTOROWA
// ===================================================================================

impl<const N: usize> Cs<N> where Cs<N>: Dim {
    
    /// Odejmuje drugi wektor od obecnego (zwraca wektor między dwoma punktami: B - A).
    #[rustfmt::skip] #[inline] 
    pub fn sub(&self, other: &Self) -> Self { 
        // let mut result = [0.0; N];
        // for i in 0..N { result[i] = self.0[i] - other.0[i]; }
        // Cs(result)
        Cs(array::from_fn(|i| self.0[i] - other.0[i]))
    }
    
    /// Dodaje drugi wektor do obecnego (przesunięcie punktu o wektor).
    #[rustfmt::skip] #[inline] 
    pub fn add(&self, other: &Self) -> Self { 
        // let mut result = [0.0; N];
        // for i in 0..N { result[i] = self.0[i] + other.0[i]; }
        // Cs(result)
        Cs(array::from_fn(|i| self.0[i] + other.0[i]))
    }

    /// Iloczyn skalarny (Dot product) dla dowolnego wymiaru przestrzeni.
    #[rustfmt::skip] #[inline] 
    pub fn dot(&self, other: &Self) -> f64 { 
        let mut sum = 0.0;
        for i in 0..N { sum += self.0[i] * other.0[i]; }
        sum
    }

    /// Kwadrat pełnej długości wektora (wspólny dla 2D i 3D). 
    /// Zastępuje rxy_sq dla 2D oraz rxyz_sq dla 3D w operacjach ogólnych.
    #[rustfmt::skip] #[inline]
    pub fn r_sq(&self) -> f64 { self.dot(self) }

    /// Pełna długość wektora (uniwersalne R).
    #[rustfmt::skip] #[inline]
    pub fn r(&self) -> f64 { self.r_sq().sqrt() }

    /// Zwraca znormalizowany wektor (kierunek, długość = 1.0) dla danego wymiaru.
    /// Zwraca wektor zerowy, jeśli długość oryginału to 0.0.
    #[rustfmt::skip] #[inline]  
    pub fn normalize_r_projection(&self) -> Self {
        let radius = self.r();
        if tolerance::is_zero(radius) { 
            Cs([0.0; N]) 
        } else { 
            // let mut result = [0.0; N];
            // for i in 0..N { result[i] = self.0[i] / radius; }
            // Cs(result)
            Cs(array::from_fn(|i| self.0[i] / radius))
        }
    }

    /// Zwraca kąt między dwoma wektorami w radianach (uniwersalne dla 2D i 3D).
    #[rustfmt::skip] #[inline] 
    pub fn angle_between(&self, other: &Self) -> f64 {
        let r1 = self.r();
        let r2 = other.r();
        if tolerance::is_zero(r1) || tolerance::is_zero(r2) { 
            0.0 
        } else { 
            (self.dot(other) / (r1 * r2)).clamp(-1.0, 1.0).acos() 
        }
    }
}


// ===================================================================================
// PRZECIĄŻANIE OPERATORÓW DLA DOWOLNEGO N
// (Implementacja automatyczna dzięki użyciu tablic!)
// ===================================================================================

// --- DODAWANIE (Cs + Cs) ---
impl<const N: usize> Add for Cs<N> where Cs<N>: Dim {
    type Output = Cs<N>;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        // let mut result = [0.0; N];
        // for i in 0..N { result[i] = self.0[i] + rhs.0[i]; }
        // Cs(result)
        Cs(array::from_fn(|i| self.0[i] + rhs.0[i]))
    }
}

// --- ODEJMOWANIE (Cs - Cs) ---
impl<const N: usize> Sub for Cs<N> where Cs<N>: Dim {
    type Output = Cs<N>;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        // let mut result = [0.0; N];
        // for i in 0..N { result[i] = self.0[i] - rhs.0[i]; }
        // Cs(result)
        Cs(array::from_fn(|i| self.0[i] - rhs.0[i]))
    }
}

// --- NEGACJA (-Cs) ---
impl<const N: usize> Neg for Cs<N> where Cs<N>: Dim {
    type Output = Cs<N>;
    #[inline]
    fn neg(self) -> Self::Output {
        // let mut result = [0.0; N];
        // for i in 0..N { result[i] = -self.0[i]; }
        // Cs(result)
        Cs(array::from_fn(|i| -self.0[i]))
    }
}

// --- MNOŻENIE PRZEZ SKALAR (Cs * f64) ---
impl<const N: usize> Mul<f64> for Cs<N> where Cs<N>: Dim {
    type Output = Cs<N>;
    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        // let mut result = [0.0; N];
        // for i in 0..N { result[i] = self.0[i] * rhs; }
        // Cs(result)
        Cs(array::from_fn(|i| self.0[i] * rhs))
    }
}

// --- MNOŻENIE PRZEZ SKALAR (f64 * Cs) - ułatwia życie, gdy f64 jest po lewej ---
impl<const N: usize> Mul<Cs<N>> for f64 where Cs<N>: Dim {
    type Output = Cs<N>;
    #[inline]
    fn mul(self, rhs: Cs<N>) -> Self::Output {
        rhs * self // Wywołuje implementację wyżej
    }
}

// --- DZIELENIE PRZEZ SKALAR (Cs / f64) ---
impl<const N: usize> Div<f64> for Cs<N> where Cs<N>: Dim {
    type Output = Cs<N>;
    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        // let mut result = [0.0; N];
        // for i in 0..N { result[i] = self.0[i] / rhs; }
        // Cs(result)
        Cs(array::from_fn(|i| self.0[i] / rhs))
    }
}


/*
 *  --- DODATKOWE METODY KĄTÓW (Nie są potrzebne, bo można użyć arctan_y_x, arctan_z_x itd. -
 *  --- ale zostawiam je jako ciekawostkę dla map (Bearing/Heading)) 
 *  --- gdzie $0°$ to Północ (oś Y) i kąt rośnie zgodnie ze wskazówkami zegara (CW) w stronę Wschodu (X). 
 *
 *   #[rustfmt::skip] #[inline] fn arctan_x_y(&self) -> f64 { self.0[0].atan2(self.0[1])  }   
 *   #[rustfmt::skip] #[inline] fn arctan_x_z(&self) -> f64 { self.0[0].atan2(self.0[2])  }   
 *   #[rustfmt::skip] #[inline] fn arctan_y_z(&self) -> f64 { self.0[1].atan2(self.0[2])  }   
 */