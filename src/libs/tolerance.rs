// 📃 ./src/libs/tolerance.rs

/// 📚 【 POL】: Absolutna tolerancja przestrzenna używana do weryfikacji koincydencji punktów i długości wektorów.
/// 📚 【 ENG】: Absolute spatial tolerance used for point coincidence and vector length verification.
pub const EPSILON_SPATIAL: f64 = 1e-9;

/// 📚 【 POL】: Weryfikuje, czy wartość mieści się w zakresie błędu numerycznego bliskiego zeru.
/// 📚 【 ENG】: Checks if the value falls within the numerical error range close to zero.
#[inline]
pub fn is_zero(value: f64) -> bool { value.abs() <= EPSILON_SPATIAL }

/// 📚 【 POL】: Porównuje dwie wartości zmiennoprzecinkowe z uwzględnieniem progu tolerancji jądra.
/// 📚 【 ENG】: Compares two floating-point values considering the kernel's tolerance threshold.
#[inline]
pub fn is_equal(a: f64, b: f64) -> bool { (a - b).abs() <= EPSILON_SPATIAL }
