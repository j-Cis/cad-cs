// src/libs.rs

/// 📚 【 POL】: Izolator systemu kątowego. Bezpieczne zarządzanie radianami, stopniami i formatem DMS.
/// 📚 【 ENG】: Angular system insulator. Safe management of radians, degrees, and DMS format.
pub mod angle;

/// 📚 【 POL】: Jądro geometryczne. Wektory, układy współrzędnych i transformacje w przestrzeni 2D/3D.
/// 📚 【 ENG】: Geometric kernel. Vectors, coordinate systems, and transformations in 2D/3D space.
pub mod cs;

/// 📚 【 POL】: Silnik aproksymacji wymiernej. Ułamki łańcuchowe i reprezentacja wielokrotności liczby PI.
/// 📚 【 ENG】: Rational approximation engine. Continued fractions and PI multiples representation.
pub mod frac;

/// 📚 【 POL】: Fundament stabilności numerycznej. Globalne progi tolerancji dla operacji zmiennoprzecinkowych.
/// 📚 【 ENG】: Numerical stability foundation. Global tolerance thresholds for floating-point operations.
pub mod tolerance;
