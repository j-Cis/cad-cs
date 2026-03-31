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
    #[rustfmt::skip] #[inline]    pub fn rxy(&self)  -> f64 { self.0[0].hypot(self.0[1]) }
    #[rustfmt::skip] #[inline]    pub fn rxz(&self)  -> f64 { self.0[0].hypot(self.0[2]) }
    #[rustfmt::skip] #[inline]    pub fn ryz(&self)  -> f64 { self.0[1].hypot(self.0[2]) }
    #[rustfmt::skip] #[inline]    pub fn rxyz(&self) -> f64 { self.0[0].hypot(self.0[1]).hypot(self.0[2]) }

    // Kąty płaskie (azymut)
    #[rustfmt::skip] #[inline]    pub fn arctan_y_x(&self) -> f64 { self.0[1].atan2(self.0[0]) }
    #[rustfmt::skip] #[inline]    pub fn arctan_z_x(&self) -> f64 { self.0[2].atan2(self.0[0]) }
    #[rustfmt::skip] #[inline]    pub fn arctan_z_y(&self) -> f64 { self.0[2].atan2(self.0[1]) }
    // Kąty kompasowe / mapowe (0° to Y/Północ)
    #[rustfmt::skip] #[inline]    pub fn arctan_x_y(&self) -> f64 { self.0[0].atan2(self.0[1]) }  
    #[rustfmt::skip] #[inline]    pub fn arctan_x_z(&self) -> f64 { self.0[0].atan2(self.0[2]) }  
    #[rustfmt::skip] #[inline]    pub fn arctan_y_z(&self) -> f64 { self.0[1].atan2(self.0[2]) }

    // Kąty przestrzenne (inklinacja/odchylenie)
    #[rustfmt::skip] #[inline]    pub fn arccos_x_rxyz(&self) -> f64 { let r = self.rxyz(); if tolerance::is_zero(r) { 0.0 } else { (self.0[0] / r).clamp(-1.0, 1.0).acos() } }
    #[rustfmt::skip] #[inline]    pub fn arccos_y_rxyz(&self) -> f64 { let r = self.rxyz(); if tolerance::is_zero(r) { 0.0 } else { (self.0[1] / r).clamp(-1.0, 1.0).acos() } }
    #[rustfmt::skip] #[inline]    pub fn arccos_z_rxyz(&self) -> f64 { let r = self.rxyz(); if tolerance::is_zero(r) { 0.0 } else { (self.0[2] / r).clamp(-1.0, 1.0).acos() } }

    // Konwersje 3D -> 2D (Rzuty polarne)
    #[rustfmt::skip] #[inline]    pub fn to_rf_from_xy(&self) -> Cs<2> { Cs([self.rxy(), self.arctan_y_x()]) }
    #[rustfmt::skip] #[inline]    pub fn to_rf_from_xz(&self) -> Cs<2> { Cs([self.rxz(), self.arctan_z_x()]) }
    #[rustfmt::skip] #[inline]    pub fn to_rf_from_yz(&self) -> Cs<2> { Cs([self.ryz(), self.arctan_z_y()]) }

    // Konwersje 3D -> 3D (Cylindryczne i Sferyczne)
    #[rustfmt::skip] #[inline]    pub fn to_rfx_from_xyz(&self) -> Cs<3> { Cs([self.ryz(),  self.arctan_z_y(), self.0[0]]) }
    #[rustfmt::skip] #[inline]    pub fn to_rfy_from_xyz(&self) -> Cs<3> { Cs([self.rxz(),  self.arctan_z_x(), self.0[1]]) }
    #[rustfmt::skip] #[inline]    pub fn to_rfz_from_xyz(&self) -> Cs<3> { Cs([self.rxy(),  self.arctan_y_x(), self.0[2]]) }
    #[rustfmt::skip] #[inline]    pub fn to_rft_from_xyz(&self) -> Cs<3> { Cs([self.rxyz(), self.arctan_y_x(), self.arccos_z_rxyz()]) }

    // ===================================================================================
    // KONWERSJE GEODEZYJNE
    // ===================================================================================

    /// Bezpośredni konstruktor wektora ECEF (XYZ) z formatu DMS.
    /// Nazewnictwo `sn_we`: najpierw Szerokość (S/N), potem Długość (W/E).
    #[rustfmt::skip]
    pub fn to_ecef_from_dms_sn_we(
        sn_d: i16, sn_m: u8, sn_s: f32, 
        we_d: i16, we_m: u8, we_s: f32, 
        r: f64
    ) -> Self {
        use crate::libs::angle::Angle;
        let lat_rad = Angle::from_dms(sn_d, sn_m, sn_s).rad();
        let lon_rad = Angle::from_dms(we_d, we_m, we_s).rad();
        
        let (sin_lat, cos_lat) = lat_rad.sin_cos();
        let (sin_lon, cos_lon) = lon_rad.sin_cos();

        Cs([
            r * cos_lat * cos_lon,
            r * cos_lat * sin_lon,
            r * sin_lat
        ])
    }

    /// Przekształca wektor 3D ECEF (X, Y, Z) na współrzędne geograficzne w formacie DMS.
    /// Zwraca strukturę `CoordsDmsNz90Ex0` (najpierw Szerokość SN, potem Długość WE).
    #[rustfmt::skip]
    pub fn to_dms_sn_we_from_xyz(&self) -> crate::libs::cs::model_coords::CoordsDmsNz90Ex0 {
        use crate::libs::angle::Angle;
        use crate::libs::tolerance;

        let r = self.rxyz();
        
        // Zabezpieczenie przed osobliwością w samym jądrze Ziemi (r = 0)
        let (lat_rad, lon_rad) = if tolerance::is_zero(r) {
            (0.0, 0.0)
        } else {
            (
                (self.0[2] / r).clamp(-1.0, 1.0).asin(), // Szerokość z osi Z
                self.0[1].atan2(self.0[0])               // Długość z płaszczyzny XY
            )
        };

        // Rzutujemy radiany na nasz izolator Angle
        let lat_angle = Angle::from_rad(lat_rad);
        let lon_angle = Angle::from_rad(lon_rad);

        // Dekodujemy do składowych (Stopnie, Minuty, Sekundy)
        let (lat_d, lat_m, lat_s) = lat_angle.to_dms();
        let (lon_d, lon_m, lon_s) = lon_angle.to_dms();

        // Pakujemy w DTO
        crate::libs::cs::model_coords::CoordsDmsNz90Ex0 {
            sn_lat_d: lat_d as i8,
            sn_lat_m: lat_m,
            sn_lat_s: lat_s,
            we_lon_d: lon_d,
            we_lon_m: lon_m,
            we_lon_s: lon_s,
        }
    }

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
    #[rustfmt::skip] #[inline]    pub fn rxyz_sq(&self) -> f64 { self.0[0] * self.0[0] + self.0[1] * self.0[1] + self.0[2] * self.0[2] }

    // Kwadraty promieni cząstkowych
    #[rustfmt::skip] #[inline]    pub fn rxy_sq(&self) -> f64 { self.0[0] * self.0[0] + self.0[1] * self.0[1] }
    #[rustfmt::skip] #[inline]    pub fn rxz_sq(&self) -> f64 { self.0[0] * self.0[0] + self.0[2] * self.0[2] }
    #[rustfmt::skip] #[inline]    pub fn ryz_sq(&self) -> f64 { self.0[1] * self.0[1] + self.0[2] * self.0[2] }

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
