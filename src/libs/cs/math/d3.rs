// src/libs/cs/math/d3.rs
use crate::libs::cs::model::Cs;
use crate::libs::cs::utils::SignStrExt;
use crate::libs::tolerance;

// ===================================================================================
// SPECJALIZACJA 
// ===================================================================================


impl Cs<3> {
    // ===================================================================================
    // SPECJALIZACJA 3D (Tylko dla wektorów przestrzennych)
    // ===================================================================================
    
    // Promienie cząstkowe i pełne
    #[rustfmt::skip] #[inline] pub fn rxy(&self)  -> f64 { self.0[0].hypot(self.0[1]) }
    #[rustfmt::skip] #[inline] pub fn rxz(&self)  -> f64 { self.0[0].hypot(self.0[2]) }
    #[rustfmt::skip] #[inline] pub fn ryz(&self)  -> f64 { self.0[1].hypot(self.0[2]) }
    #[rustfmt::skip] #[inline] pub fn rxyz(&self) -> f64 { self.0[0].hypot(self.0[1]).hypot(self.0[2]) } 

    // Kąty płaskie (azymut)
    #[rustfmt::skip] #[inline] pub fn arctan_y_x(&self) -> f64 { self.0[1].atan2(self.0[0]) }
    #[rustfmt::skip] #[inline] pub fn arctan_z_x(&self) -> f64 { self.0[2].atan2(self.0[0]) }
    #[rustfmt::skip] #[inline] pub fn arctan_z_y(&self) -> f64 { self.0[2].atan2(self.0[1]) }

    // Kąty przestrzenne (inklinacja/odchylenie)
    #[rustfmt::skip] #[inline] pub fn arccos_x_rxyz(&self) -> f64 { let r = self.rxyz(); if tolerance::is_zero(r) { 0.0 } else { (self.0[0] / r).clamp(-1.0, 1.0).acos() } }
    #[rustfmt::skip] #[inline] pub fn arccos_y_rxyz(&self) -> f64 { let r = self.rxyz(); if tolerance::is_zero(r) { 0.0 } else { (self.0[1] / r).clamp(-1.0, 1.0).acos() } }
    #[rustfmt::skip] #[inline] pub fn arccos_z_rxyz(&self) -> f64 { let r = self.rxyz(); if tolerance::is_zero(r) { 0.0 } else { (self.0[2] / r).clamp(-1.0, 1.0).acos() } }

    // Konwersje 3D -> 2D (Rzuty polarne)
    #[rustfmt::skip] #[inline] pub fn to_rf_from_xy(&self) -> Cs<2> { Cs([self.rxy(), self.arctan_y_x()]) }
    #[rustfmt::skip] #[inline] pub fn to_rf_from_xz(&self) -> Cs<2> { Cs([self.rxz(), self.arctan_z_x()]) }
    #[rustfmt::skip] #[inline] pub fn to_rf_from_yz(&self) -> Cs<2> { Cs([self.ryz(), self.arctan_z_y()]) }

    // Konwersje 3D -> 3D (Cylindryczne i Sferyczne)
    #[rustfmt::skip] #[inline] pub fn to_rfx_from_xyz(&self) -> Cs<3> { Cs([self.ryz(),  self.arctan_z_y(), self.0[0]]) }
    #[rustfmt::skip] #[inline] pub fn to_rfy_from_xyz(&self) -> Cs<3> { Cs([self.rxz(),  self.arctan_z_x(), self.0[1]]) }
    #[rustfmt::skip] #[inline] pub fn to_rfz_from_xyz(&self) -> Cs<3> { Cs([self.rxy(),  self.arctan_y_x(), self.0[2]]) }
    #[rustfmt::skip] #[inline] pub fn to_rft_from_xyz(&self) -> Cs<3> { Cs([self.rxyz(), self.arctan_y_x(), self.arccos_z_rxyz()]) }

    /// Zwraca oktant w przestrzeni XYZ (1-8)
    #[rustfmt::skip] #[inline]
    pub fn q(&self) -> u8 {
        match (self.0[0] >= 0.0, self.0[1] >= 0.0, self.0[2] >= 0.0) {
            (true, true, true)   => 1, (false, true, true)  => 2, (false, false, true) => 3, (true, false, true)  => 4,
            (true, true, false)  => 5, (false, true, false) => 6, (false, false, false)=> 7, (true, false, false) => 8,
        }
    }
    /// Zwraca znaki kierunkowe osi X, Y i Z w formie tablicy (np. ["+", "-", "+"]).
    /// Alternatywa wizualna i logiczna dla numeru oktantu.
    #[rustfmt::skip] #[inline]
    pub fn q_sign(&self) -> [&'static str; 3] {
        [self.0[0].sign_str(), self.0[1].sign_str(), self.0[2].sign_str()]
    }

    /// Kwadrat pełnej długości wektora 3D.
    #[rustfmt::skip] #[inline] pub fn rxyz_sq(&self) -> f64 { self.0[0] * self.0[0] + self.0[1] * self.0[1] + self.0[2] * self.0[2] }
    
    // Kwadraty promieni cząstkowych
    #[rustfmt::skip] #[inline] pub fn rxy_sq(&self) -> f64 { self.0[0] * self.0[0] + self.0[1] * self.0[1] }
    #[rustfmt::skip] #[inline] pub fn rxz_sq(&self) -> f64 { self.0[0] * self.0[0] + self.0[2] * self.0[2] }
    #[rustfmt::skip] #[inline] pub fn ryz_sq(&self) -> f64 { self.0[1] * self.0[1] + self.0[2] * self.0[2] }

    /// Zwraca wektor 3D, którego rzut na płaszczyznę XY jest znormalizowany (długość w XY = 1.0).
    /// Współrzędna Z pozostaje niezmieniona.
    #[rustfmt::skip] #[inline] 
    pub fn normalize_rxy_projection(&self) -> Cs<3> {
        let r = self.rxy();
        if tolerance::is_zero(r) { Cs([0.0, 0.0, self.0[2]]) } else { Cs([self.0[0] / r, self.0[1] / r, self.0[2]]) }
    }

    /// Zwraca wektor 3D, którego rzut na płaszczyznę XZ jest znormalizowany (długość w XZ = 1.0).
    /// Współrzędna Y pozostaje niezmieniona.
    #[rustfmt::skip] #[inline] 
    pub fn normalize_rxz_projection(&self) -> Cs<3> {
        let r = self.rxz();
        if tolerance::is_zero(r) { Cs([0.0, self.0[1], 0.0]) } else { Cs([self.0[0] / r, self.0[1], self.0[2] / r]) }
    }

    /// Zwraca wektor 3D, którego rzut na płaszczyznę YZ jest znormalizowany (długość w YZ = 1.0).
    /// Współrzędna X pozostaje niezmieniona.
    #[rustfmt::skip] #[inline] 
    pub fn normalize_ryz_projection(&self) -> Cs<3> {
        let r = self.ryz();
        if tolerance::is_zero(r) { Cs([self.0[0], 0.0, 0.0]) } else { Cs([self.0[0], self.0[1] / r, self.0[2] / r]) }
    }
    
    /// Iloczyn wektorowy (Cross product) - ABSOLUTNY FUNDAMENT W 3D (Zwraca wektor prostopadły do dwóch innych).
    #[rustfmt::skip] #[inline] 
    pub fn cross(&self, other: &Cs<3>) -> Cs<3> {
        Cs([
            self.0[1] * other.0[2] - self.0[2] * other.0[1],
            self.0[2] * other.0[0] - self.0[0] * other.0[2],
            self.0[0] * other.0[1] - self.0[1] * other.0[0]
        ])
    }
  }