// src/libs/cs/math/d2.rs
use crate::libs::cs::model::Cs;
use crate::libs::cs::utils::SignStrExt;

// ===================================================================================
// SPECJALIZACJA
// ===================================================================================

impl Cs<2> {
    // ===================================================================================
    // SPECJALIZACJA 2D (Tylko dla wektorów płaskich)
    // ===================================================================================

    // Matematyka i kąty 2D
    #[rustfmt::skip] #[inline]    pub fn rxy(&self) -> f64 { self.0[0].hypot(self.0[1]) }
    #[rustfmt::skip] #[inline]    pub fn arctan_y_x(&self) -> f64 { self.0[1].atan2(self.0[0]) }
    // Kąty kompasowe / mapowe (0° to Y/Północ, rośnie zgodnie z ruchem wskazówek zegara)
    #[rustfmt::skip] #[inline]    pub fn arctan_x_y(&self) -> f64 { self.0[0].atan2(self.0[1]) }

    // Konwersje w obrębie 2D
    #[rustfmt::skip] #[inline]    pub fn to_rf_from_xy(&self) -> Cs<2> { Cs([self.rxy(), self.arctan_y_x()]) }

    /// Przekształca wektor 2D [Szerokość_rad, Długość_rad] na wektor 3D (X, Y, Z)
    /// ze środkiem (0,0,0) w jądrze Ziemi.
    /// Parametr `r` to promień Ziemi (np. 6371000.0 m dla średniego promienia).
    pub fn to_ecef_from_rad_sn_we(&self, r: f64) -> Cs<3> {
        // Składowe trygonometryczne (Rust optymalizuje to sprzętowo jako jedną instrukcję)
        let (sin_lat, cos_lat) = self.0[0].sin_cos();
        let (sin_lon, cos_lon) = self.0[1].sin_cos();

        // Oś X: 0°N, 0°E (Greenwich na równiku)
        // Oś Y: 0°N, 90°E (Ocean Indyjski na równiku)
        // Oś Z: 90°N (Biegun Północny)
        Cs([
            r * cos_lat * cos_lon, // X
            r * cos_lat * sin_lon, // Y
            r * sin_lat,           // Z
        ])
    }

    /// Zwraca ćwiartkę na płaszczyźnie XY (1-4)
    #[rustfmt::skip] #[inline]
    pub fn q(&self) -> u8 {
        match (self.0[0] >= 0.0, self.0[1] >= 0.0) {
            (true, true) => 1, (false, true) => 2, (false, false) => 3, (true, false) => 4,
        }
    }
    /// Zwraca znaki kierunkowe osi X i Y w formie tablicy (np. ["+", "-"]).
    /// Alternatywa wizualna i logiczna dla numeru ćwiartki.
    #[rustfmt::skip] #[inline]
    pub fn q_sign(&self) -> [&'static str; 2] {
        [self.0[0].sign_str(), self.0[1].sign_str()]
    }

    /// Kwadrat długości wektora (szybsze od rxy, bo nie robi pierwiastkowania). Idealne do porównywania odległości.
    #[rustfmt::skip] #[inline]    pub fn rxy_sq(&self) -> f64 { self.0[0] * self.0[0] + self.0[1] * self.0[1] }

    /// 2D "Iloczyn wektorowy" (skalarny iloczyn wektorowy / wyznacznik 2x2).
    /// Zwraca SKALAR (f64). Znak wyniku informuje o orientacji:
    /// wartość dodatnia oznacza, że wektor 'other' skręca w lewo (CCW), ujemna - w prawo (CW).
    /// Wartość bezwzględna to pole równoległoboku rozpiętego przez te dwa wektory.
    #[rustfmt::skip] #[inline]
    pub fn cross(&self, other: &Cs<2>) -> f64 { self.0[0] * other.0[1] - self.0[1] * other.0[0] }

    /// Zwraca wektor prostopadły, obrócony o 90 stopni w lewo (CCW) na płaszczyźnie 2D.
    /// Często używany w parze z dot(), aby symulować operacje wektorowe 3D.
    #[rustfmt::skip] #[inline]
    pub fn perp(&self) -> Cs<2> { Cs([-self.0[1], self.0[0]]) }
}
