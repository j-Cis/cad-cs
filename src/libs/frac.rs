use super::tolerance;
use std::f64::consts;

// Makra pomocnicze do czystego zapisu ułamków i wielokrotności PI.
// Eliminują problem dzielenia całkowitoliczbowego (gdzie `1/2` daje `0`)
// oraz konieczność dodawania `.0` do literałów.

/// Makro do czystego zapisu ułamków zwykłych.
/// Zwraca surowe `f64`.
#[macro_export]
macro_rules! frac {
    ($n:tt / $d:tt) => {
        (($n as f64) / ($d as f64))
    };
    ($n:tt) => {
        ($n as f64)
    };
}

/// Makro do czystego zapisu wielokrotności PI.
/// Zwraca surowe `f64`. Nie zakłada, że wynik jest kątem!
#[macro_export]
macro_rules! frac_pi {
    ($n:tt / $d:tt) => {
        (($n as f64) / ($d as f64)) * consts::PI
    };
    ($n:tt) => {
        ($n as f64) * consts::PI
    };
}

/// Zamienia wartość zmiennoprzecinkową na ułamek zwykły (licznik, mianownik).
/// Wykorzystuje algorytm ułamków łańcuchowych (Continued Fractions) dopasowany do tolerancji jądra.
pub fn as_frac(value: f64) -> (f64, f64) {
    if tolerance::is_zero(value) {
        return (0.0, 1.0);
    }

    let sign = value.signum();
    let mut val = value.abs();

    // POPRAWKA: Właściwe warunki startowe macierzy ułamków łańcuchowych!
    let mut num_prev = 0.0;
    let mut den_prev = 1.0;
    let mut num_curr = 1.0;
    let mut den_curr = 0.0;

    // Limit iteracji zapobiega pętli w przypadku liczb niewymiernych
    for _ in 0..64 {
        let a = val.floor();
        let num_next = a * num_curr + num_prev;
        let den_next = a * den_curr + den_prev;

        // Ograniczenie mianownika
        if den_next > 1_000_000.0 {
            break;
        }

        num_prev = num_curr;
        den_prev = den_curr;
        num_curr = num_next;
        den_curr = den_next;

        let diff = val - a;

        // Przerywamy, gdy trafimy z dokładnością naszego inżynieryjnego jądra
        if tolerance::is_zero(diff) {
            break;
        }
        val = 1.0 / diff;
    }

    (sign * num_curr, den_curr)
}

/// Odwrotność makra `frac_pi!`. Przyjmuje surową wartość (np. kąt w radianach)
/// i zwraca go jako ułamek wielokrotności PI (licznik, mianownik).
pub fn as_frac_pi(value: f64) -> (f64, f64) {
    as_frac(value / consts::PI)
}
