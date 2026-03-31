// src/libs/cs/core.rs
use crate::libs::cs::model::{Cs, Cs2, Cs3};
use crate::libs::cs::model_coords::*;
// UWAGA: metody "from" używają sin_cos(), więc nic więcej matematycznego nie potrzebują,
// ale używają is_zero z modułu tolerance! Zatem:

// ===================================================================================
// KONWERSJE: Z BEZPIECZNYCH STRUKTUR (DTO) DO SZYBKIEGO JĄDRA (Cs<N>)
// ===================================================================================

// --- PŁASZCZYZNA XY ---

impl From<Coords2dXy> for Cs2 {
    #[inline] fn from(c: Coords2dXy) -> Self { Cs([c.x, c.y]) }
}

impl From<Coords2dXyPolar> for Cs2 {
    #[inline] fn from(c: Coords2dXyPolar) -> Self {
        let (sin_f, cos_f) = c.f_yx.sin_cos();
        Cs([c.r_xy * cos_f, c.r_xy * sin_f])
    }
}

impl From<Coords3dXyCylindricalZ> for Cs3 {
    #[inline] fn from(c: Coords3dXyCylindricalZ) -> Self {
        let (sin_f, cos_f) = c.f_yx.sin_cos();
        Cs([c.r_xy * cos_f, c.r_xy * sin_f, c.z])
    }
}

// --- PŁASZCZYZNA XZ ---

impl From<Coords2dXz> for Cs2 {
    #[inline] fn from(c: Coords2dXz) -> Self { Cs([c.x, c.z]) }
}

impl From<Coords2dXzPolar> for Cs2 {
    #[inline] fn from(c: Coords2dXzPolar) -> Self {
        let (sin_f, cos_f) = c.f_zx.sin_cos();
        Cs([c.r_xz * cos_f, c.r_xz * sin_f])
    }
}

impl From<Coords3dXzCylindricalY> for Cs3 {
    #[inline] fn from(c: Coords3dXzCylindricalY) -> Self {
        let (sin_f, cos_f) = c.f_zx.sin_cos();
        Cs([c.r_xz * cos_f, c.y, c.r_xz * sin_f])
    }
}

// --- PŁASZCZYZNA YZ ---

impl From<Coords2dYz> for Cs2 {
    #[inline] fn from(c: Coords2dYz) -> Self { Cs([c.y, c.z]) }
}

impl From<Coords2dYzPolar> for Cs2 {
    #[inline] fn from(c: Coords2dYzPolar) -> Self {
        let (sin_f, cos_f) = c.f_zy.sin_cos();
        Cs([c.r_yz * cos_f, c.r_yz * sin_f])
    }
}

impl From<Coords3dYzCylindricalX> for Cs3 {
    #[inline] fn from(c: Coords3dYzCylindricalX) -> Self {
        let (sin_f, cos_f) = c.f_zy.sin_cos();
        Cs([c.x, c.r_yz * cos_f, c.r_yz * sin_f])
    }
}

// --- PRZESTRZEŃ 3D (Sferyczna i Kartezjańska) ---

impl From<Coords3dXyz> for Cs3 {
    #[inline] fn from(c: Coords3dXyz) -> Self { Cs([c.x, c.y, c.z]) }
}

impl From<Coords3dXyzSpherical> for Cs3 {
    #[inline] fn from(c: Coords3dXyzSpherical) -> Self {
        let (sin_f, cos_f) = c.f_yx.sin_cos(); // Azymut XY
        let (sin_t, cos_t) = c.t_zr.sin_cos(); // Inklinacja Z do R
        Cs([
            c.r_xyz * sin_t * cos_f, 
            c.r_xyz * sin_t * sin_f, 
            c.r_xyz * cos_t
        ])
    }
}


impl Cs<2>{

/// Tworzy nowy wektor 2D z układu biegunowego (R, Φ) na płaszczyźnie XY.
    /// Kąt Phi (azymut) należy podać w radianach.
    #[rustfmt::skip] #[inline] 
    pub fn new_from_rf(r: f64, phi_rad: f64) -> Self { 
        let (sin_phi, cos_phi) = phi_rad.sin_cos();
        Cs([r * cos_phi, r * sin_phi]) 
    }

    // Projeksje 2D -> 3D (zwracają nową pamięć Cs<3>)
    #[rustfmt::skip] #[inline] pub fn new_as_xy(&self) -> Cs<3> { Cs([self.0[0], self.0[1], 0.0]) }
    #[rustfmt::skip] #[inline] pub fn new_as_xz(&self) -> Cs<3> { Cs([self.0[0], 0.0, self.0[1]]) }
    #[rustfmt::skip] #[inline] pub fn new_as_yz(&self) -> Cs<3> { Cs([0.0, self.0[0], self.0[1]]) }

    // ===================================================================================
    // KREACJA Z UKŁADÓW BIEGUNOWYCH I CYLINDRYCZNYCH (Źródło: R, Phi)
    // ===================================================================================
    // ⚠️ UWAGA DOTYCZĄCA ZAKRESU PONIŻSZYCH METOD:
    // Poniższe metody asymilują ("as"), że wywołujący obiekt `self` NIE JEST 
    // wektorem kartezjańskim [X, Y], lecz kontenerem na dane układu biegunowego:
    // self.0[0] = R (Promień)
    // self.0[1] = Φ (Kąt Azymutu w radianach)
    // Przykładowe wywołanie: let punkt = cs![promien, azymut_rad].new_as_xy_from_rf();

    /// Interpretuje obecny wektor jako układ biegunowy [R, Φ] i zwraca kartezjański wektor 2D [X, Y].
    #[rustfmt::skip] #[inline] 
    pub fn new_as_xy_from_rf(&self) -> Cs<2> {
        let (sin_phi, cos_phi) = self.0[1].sin_cos();
        Cs([self.0[0] * cos_phi, self.0[0] * sin_phi])
    }

    /// Interpretuje obecny wektor jako dane rzutu walcowego względem osi Z [R_xy, Φ] 
    /// i po dodaniu wysokości 'z' zwraca kartezjański wektor 3D [X, Y, Z].
    #[rustfmt::skip] #[inline] 
    pub fn new_as_xyz_from_rf_with_z(&self, z: f64) -> Cs<3> {
        let (sin_phi, cos_phi) = self.0[1].sin_cos();
        Cs([self.0[0] * cos_phi, self.0[0] * sin_phi, z])
    }

    /// Interpretuje obecny wektor jako dane rzutu walcowego względem osi X [R_yz, Φ] 
    /// (kąt od osi Y do Z) i po dodaniu głębokości 'x' zwraca kartezjański wektor 3D [X, Y, Z].
    #[rustfmt::skip] #[inline] 
    pub fn new_as_xyz_from_rf_with_x(&self, x: f64) -> Cs<3> {
        let (sin_phi, cos_phi) = self.0[1].sin_cos();
        Cs([x, self.0[0] * cos_phi, self.0[0] * sin_phi])
    }

    /// Interpretuje obecny wektor jako dane rzutu walcowego względem osi Y [R_xz, Φ] 
    /// (kąt od osi X do Z) i po dodaniu szerokości 'y' zwraca kartezjański wektor 3D [X, Y, Z].
    #[rustfmt::skip] #[inline] 
    pub fn new_as_xyz_from_rf_with_y(&self, y: f64) -> Cs<3> {
        let (sin_phi, cos_phi) = self.0[1].sin_cos();
        Cs([self.0[0] * cos_phi, y, self.0[0] * sin_phi])
    }
}

impl Cs<3> {
    /// Tworzy nowy wektor 3D z układu sferycznego (R, Φ, Θ).
    /// Kąty Phi (azymut) i Theta (inklinacja od osi Z) należy podać w radianach.
    #[rustfmt::skip] #[inline] 
    pub fn new_from_rft(r: f64, phi_rad: f64, theta_rad: f64) -> Self {
        let (sin_phi, cos_phi) = phi_rad.sin_cos();
        let (sin_theta, cos_theta) = theta_rad.sin_cos();
        Cs([
            r * sin_theta * cos_phi, 
            r * sin_theta * sin_phi, 
            r * cos_theta
        ])
    }

    /// Tworzy nowy wektor 3D z układu cylindrycznego względem osi Z: (R_xy, Φ, Z).
    #[rustfmt::skip] #[inline] 
    pub fn new_from_rfz(r_xy: f64, phi_rad: f64, z: f64) -> Self {
        let (sin_phi, cos_phi) = phi_rad.sin_cos();
        Cs([r_xy * cos_phi, r_xy * sin_phi, z])
    }

    /// Tworzy nowy wektor 3D z układu cylindrycznego względem osi X: (R_yz, Φ, X).
    /// Kąt mierzony jest od osi Y do Z na płaszczyźnie YZ.
    #[rustfmt::skip] #[inline] 
    pub fn new_from_rfx(r_yz: f64, phi_rad: f64, x: f64) -> Self {
        let (sin_phi, cos_phi) = phi_rad.sin_cos();
        Cs([x, r_yz * cos_phi, r_yz * sin_phi])
    }

    /// Tworzy nowy wektor 3D z układu cylindrycznego względem osi Y: (R_xz, Φ, Y).
    /// Kąt mierzony jest od osi X do Z na płaszczyźnie XZ.
    #[rustfmt::skip] #[inline] 
    pub fn new_from_rfy(r_xz: f64, phi_rad: f64, y: f64) -> Self {
        let (sin_phi, cos_phi) = phi_rad.sin_cos();
        Cs([r_xz * cos_phi, y, r_xz * sin_phi])
    }

    // ===================================================================================
    // KREACJA Z UKŁADÓW SFERYCZNYCH (Źródło: R, Phi, Theta)
    // ===================================================================================

    /// Interpretuje obecny wektor jako układ sferyczny (R, Φ, Θ) i zwraca kartezjański wektor 3D (X, Y, Z).
    #[rustfmt::skip] #[inline] 
    pub fn new_as_xyz_from_rft(&self) -> Cs<3> {
        let (sin_phi, cos_phi) = self.0[1].sin_cos();
        let (sin_theta, cos_theta) = self.0[2].sin_cos();
        Cs([
            self.0[0] * sin_theta * cos_phi,
            self.0[0] * sin_theta * sin_phi,
            self.0[0] * cos_theta
        ])
    }
}