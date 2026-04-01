// 📃 src/libs/angle/model.rs

/// 📚 【 POL】: Izolator systemu typów przechowujący wartości kątowe w radianach. Zapobiega błędom jednostkowym.
/// 📚 【 ENG】: Type system insulator storing angular values in radians. Prevents unit mismatch errors.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle(pub f64); 

/// 📚 【 POL】: Definicja dostępnych formatów prezentacji kąta.
/// 📚 【 ENG】: Definition of available angle presentation formats.
#[derive(Debug, Clone, Copy, Default)]
pub enum AngleFmt {
    #[default]
    /// 📚 【 POL】: Stopnie (np. 90.00°)
    /// 📚 【 ENG】: Degrees (e.g., 90.00°)
    Deg,
    /// 📚 【 POL】: Radiany (np. 1.5708 rad)
    /// 📚 【 ENG】: Radians (e.g., 1.5708 rad)
    Rad,
    /// 📚 【 POL】: Ułamki liczby PI (np. 1/2 π)
    /// 📚 【 ENG】: PI Fractions (e.g., 1/2 π)
    PiFrac,
}