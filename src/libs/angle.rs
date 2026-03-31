use super::frac;
use std::f64::consts;

/// Izolator systemu typów dla wartości kątowych (wewnętrzna reprezentacja zawsze w radianach).
/// Zapobiega błędom algebraicznym wynikającym z pomyłkowego przekazania kąta jako wektora liniowego.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle(f64);

impl Angle {
    // --- KONSTRUKTORY (Normalizują wszystko do wewnętrznych radianów) ---
    #[inline] pub const fn from_rad(r: f64) -> Self { Self(r) }
    #[inline] pub fn from_deg(d: f64) -> Self { Self(d.to_radians()) }
    #[inline] pub fn from_pi_frac(fraction: f64) -> Self { Self(fraction * consts::PI) }

    /// DEKODER: Zamienia (D, M, S) bezpośrednio na strukturę Angle (wewnętrznie radiany).
    pub fn from_dms(d: i16, m: u8, s: f32) -> Self {
        // Uwaga: prosty 'sign' z d < 0 nie obsłuży "-0 stopni". 
        // W GIS często używa się f64 z signum() dla stopnia, ale trzymając się i16:
        let sign = if d < 0 { -1.0_f64 } else { 1.0_f64 };
        let ddd = (d as f64) + sign * ((m as f64) / 60.0) + sign * ((s as f64) / 3600.0);
        Self::from_deg(ddd)
    }

    /// ENKODER: Zamienia wewnętrzne radiany z powrotem na krotkę (D, M, S).
    pub fn to_dms(self) -> (i16, u8, f32) {
        let ddd = self.deg();
        let sign = ddd.signum();
        let abs_ddd = ddd.abs();
        
        let d = (abs_ddd.floor() * sign) as i16;
        let m_float = (abs_ddd - abs_ddd.floor()) * 60.0;
        let m = m_float.floor() as u8;
        let s = ((m_float - m_float.floor()) * 60.0) as f32;
        
        (d, m, s)
    }
    
    // --- GETTERY MATEMATYCZNE (Zero-cost, zwracają prymitywy) ---
    #[inline] pub const fn rad(self) -> f64 { self.0 }
    #[inline] pub fn deg(self) -> f64 { self.0.to_degrees() }
    #[inline] pub fn pi_frac(self) -> (f64, f64) { frac::as_frac_pi(self.0) }

    // --- GETTERY PREZENTACYJNE (Alokują String, używać do UI/Logów) ---
    pub fn print_rad(self) -> String {
        format!("{:.4} rad", self.rad())
    }

    pub fn print_deg(self) -> String {
        format!("{:.2}°", self.deg())
    }

    pub fn print_pi_frac(self) -> String {
        let (num, den) = self.pi_frac();
        
        if num == 0.0 {
            return "0".to_string();
        }
        if den == 1.0 {
            return format!("{} π", num);
        }
        
        format!("{} / {} π", num, den)
    }
}

// --- FUNKCJE POMOCNICZE (Bezpośrednie API dla programisty) ---
#[rustfmt::skip] #[inline] pub const fn rad(r: f64) -> Angle { Angle::from_rad(r) }
#[rustfmt::skip] #[inline] pub fn deg(d: f64) -> Angle { Angle::from_deg(d) }
#[rustfmt::skip] #[inline] pub fn pi_frac(fraction: f64) -> Angle { Angle::from_pi_frac(fraction) }

// --- TRAIT ROZSZERZAJĄCY (Składnia z kropką np. 90.0.deg()) ---
pub trait AngleExt {
    fn deg(self) -> Angle;
    fn rad(self) -> Angle;
    fn pi_frac(self) -> Angle;
}

impl AngleExt for f64 {
    #[rustfmt::skip] #[inline] fn deg(self) -> Angle { Angle::from_deg(self) }
    #[rustfmt::skip] #[inline] fn rad(self) -> Angle { Angle::from_rad(self) }
    #[rustfmt::skip] #[inline] fn pi_frac(self) -> Angle { Angle::from_pi_frac(self) }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum AngleFmt {
    #[default]
    Deg,    // Stopnie (np. 90.00°)
    Rad,    // Radiany (np. 1.5708 rad)
    PiFrac, // Ułamki PI (np. 1/2 π)
}