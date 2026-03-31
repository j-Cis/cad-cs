/// Absolutna tolerancja przestrzenna (np. dla sprawdzania długości wektora i koincydencji).
/// UWAGA: Ta wartość jest krytyczna dla stabilności numerycznej całego silnika.
pub const EPSILON_SPATIAL: f64 = 1e-9;

/// Sprawdza, czy wartość zmiennoprzecinkowa jest numerycznym zerem
/// w kontekście globalnej tolerancji przestrzennej jądra.
#[inline]
pub fn is_zero(value: f64) -> bool {
    value.abs() <= EPSILON_SPATIAL
}

/// Sprawdza, czy dwie wartości zmiennoprzecinkowe są sobie równe
/// z uwzględnieniem tolerancji numerycznej.
#[inline]
pub fn is_equal(a: f64, b: f64) -> bool {
    (a - b).abs() <= EPSILON_SPATIAL
}
