# 📋 TODO: Projekt `cad-cs`

## 🚀 Plan na przyszłość

### Testy i Niezawodność

- [ ] **Testy jednostkowe (Unit Tests):** Aktualnie `cargo test` zgłasza brak testów (`running 0 tests`). Kluczowe będzie dopisanie modułu testowego dla głównych operacji matematycznych:
  - Konwersje układów współrzędnych (ECEF, DMS, sferyczne, biegunowe).
  - Weryfikacja działania `EPSILON_SPATIAL` oraz tolerancji numerycznej (`is_equal`).
  - Sprawdzenie operatorów na wektorach `Cs<N>`.
- [ ] **Testy dokumentacji (Doc Tests):** Dopisanie w dokumentacji (komentarze `///`) małych przykładów użycia każdej metody, co `cargo test` sam sprawdzi pod kątem poprawności.

### Dług technologiczny (Drobne poprawki)

- [*] **Zabezpieczenie tagu HTML w dokumentacji:** Składnia `Cs<N>` w pliku `src/libs/cs/model.rs` (linia 55) musi zostać ujęta w grawisy (`` `Cs<N>` ``), żeby `cargo doc` nie uznawał `<N>` za niezamknięty tag języka HTML.
- [ ] **Oczyszczenie importów:** Nieużywany import `AbstractHelperCs2` w `examples/basic_usage.rs` nie został naprawiony przez `cargo fix` i nadal wymaga ręcznego usunięcia.

### Rozwój Architektury

- [ ] **Eksploracja wasm64:** W miarę jak projekt zacznie operować na wielkich chmurach punktów (np. miliony wektorów 3D) rozważyć dodanie targetu `wasm64-unknown-unknown`, aby ominąć 4-gigabajtowy limit pamięci standardowego WebAssembly.

---

---

## Wdrożenie wzorca "Formatter Adapter Pattern"

**CEL:** Całkowita separacja warstwy prezentacji od logiki wyjścia (`stdout`).
Obecne rozwiązanie (`CsConsoleDebug`) jest wygodne, ale zbyt sztywno wiąże kod z makrem `println!`.

**KONCEPCJA:**
Zamiast metod `.print()`, stworzymy adaptery (lekkie struktury), które implementują trait `std::fmt::Display`. Dzięki temu punkt będzie można wypisać nie tylko na konsolę, ale też do logów, plików, czy przesłać jako tekst do UI.

**PRZEWIDYWANA ERGONOMIA:**

```rust
let pt = cs![1.0, 2.0, 3.0];
println!("Punkt kontrolny: {}", pt.display_as("P1", AngleFmt::Deg));
```

**ZALETY:**

1. **Wsparcie dla makr systemowych:** Pełna kompatybilność z `format!`, `write!`, `panic!`, `log!`.
2. **Leniwa ewaluacja (Lazy):** Formatowanie tekstu następuje dopiero w momencie rzeczywistego żądania zapisu, a nie w trakcie wywołania metody.
3. **Czystość kodu:** Całkowite usunięcie zależności od `std::io` z rdzenia matematycznego biblioteki.

**SZKIC IMPLEMENTACJI:**

```rust
struct CsFormatter<'a, const N: usize> {
    source: &'a Cs<N>,
    name: &'a str,
    fmt: AngleFmt
}

impl<'a, const N: usize> std::fmt::Display for CsFormatter<'a, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Tutaj przenosimy logikę z obecnych metod print, używając writeln!(f, ...)
        Ok(())
    }
}
```

---

---

## 🧩 Modularyzacja i Feature Flags (Planowana Optymalizacja)

**CEL:** Zapewnienie użytkownikom możliwości kompilacji tylko wybranych części biblioteki (np. samo `tolerance` bez ciężkiej trygonometrii).

### Konfiguracja `Cargo.toml`

Należy wdrożyć kaskadowy system zależności wewnętrznych:

```toml
[features]
default = ["cs"]

# Kotwice (Kaskada zależności):
cs        = ["angle", "cs_full"]
angle     = ["frac"]
frac      = ["tolerance"]
tolerance = []
cs_full   = [] # Prywatna kotwica dla pomocniczych modułów Cs
```

### Zmiany w strukturze `src/lib.rs` (lub `libs.rs`)

Wprowadzenie warunkowej kompilacji dla modułów pomocniczych:

```rust
// Przykład implementacji warunkowej w libs.rs
#[cfg(feature = "cs_full")]
pub(crate) mod cs_utils; // Dostępny tylko wewnątrz crate przy włączonym cs_full
```

**ZALETY:**

1. [cite_start]**Szybsza kompilacja:** Użytkownik potrzebujący tylko `is_zero()` nie kompiluje całej matematyki sferycznej[cite: 025].
2. [cite_start]**Mniejsze binarki:** Krytyczne przy projektach Embedded lub bardzo lekkich buildach WASM[cite: 006].
3. [cite_start]**Czystość dokumentacji:** Dzięki `all-features = true` w metadanych `docs.rs`, dokumentacja nadal będzie pokazywać pełne API[cite: 006].

---

---

## 🛠️ Poprawki i Refaktoryzacja

- [ ] **Stabilność DMS:** Rozważyć obsługę `-0` stopni w `Angle::from_dms` (obecna implementacja oparta na `i16` może gubić znak przy małych wartościach ujemnych).
- [ ] **Optymalizacja Trygonometrii:** W metodach konwersji (np. `to_ecef`) sprawdzić możliwość szerszego wykorzystania instrukcji `sin_cos()` dla redukcji kosztu obliczeniowego.
- [ ] **Implementacja Traitów:** Rozważyć implementację `std::ops::AddAssign` i `SubAssign` dla `Cs<N>` celem uniknięcia alokacji przy operacjach modyfikujących.

---

---

- [ ] **Macro compile_error:** Rozważyć użycie `compile_error!` w makrze `cs!` dla nieobsługiwanej liczby argumentów, aby dawać czytelniejsze komunikaty niż błąd dopasowania wzorca.
- [ ] **DerefMut:** Podjąć decyzję, czy dopuszczamy `impl DerefMut` dla `Cs<N>`. Obecnie struktura promuje niemutowalność, co jest bezpieczniejsze w obliczeniach wektorowych.

---

---

- [ ] **Shorthand API Consistency:** Rozważyć dodanie podobnych funkcji pomocniczych dla innych modułów (np. w `tolerance` dla szybkich porównań), jeśli ich częstotliwość użycia będzie wysoka.
- [ ] **Const context:** Sprawdzić, czy `from_deg` i `from_pi_frac` mogą stać się `const fn` (obecnie ograniczone przez wywołania `to_radians()`), co pozwoliłoby na używanie shorthandów w kontekstach statycznych.

---

---

- [ ] **Aproksymacja Symetryczna:** Przetestować zachowanie `as_frac` dla wartości ekstremalnie bliskich zeru, ale o różnym znaku, w celu uniknięcia anomalii w mianowniku.
- [ ] **Własny Typ Fraction:** Rozważyć wprowadzenie struktury `Fraction { num: f64, den: f64 }` zamiast krotki, aby umożliwić implementację traitu `Display`.

---

---

- [x] **Naprawa dokumentacji enum:** Naprawiono błąd `///` w `AngleFmt`.
- [ ] **Serde Support:** Rozważyć dodanie opcjonalnej zależności `serde` pod flagą feature, aby umożliwić automatyczną serializację struktur DTO do JSON/TOML.
- [ ] **Validation Logic:** Dodać metody walidujące zakresy dla DTO (np. czy `sn_lat_d` nie przekracza 90) przed konwersją do `Cs<N>`.

---

---

- [ ] **Symmetry Check:** Zweryfikować spójność kierunków kątów `f_z_x` oraz `f_z_y` względem reguły prawej dłoni w układach cylindrycznych osi Y i X.
- [ ] **Unified DTO Trait:** Rozważyć wprowadzenie traitu `Cylindrical` wspierającego różne osie obrotu (X, Y, Z) dla ujednolicenia interfejsu konwersji.
- [ ] **DMS Nz90Ex0 Altitude:** Podjąć decyzję o włączeniu pola `alt` (wysokość) do struktury `CoordsDmsNz90Ex0` w celu pełnej zgodności z ECEF.

---

---

- [ ] **AbstractSignStrExt - Obsługa -0.0:** Sprawdzić, czy dla `f64` należy uwzględniać "ujemne zero" w metodach `sign_*`. Obecna implementacja `self >= 0.0` traktuje `-0.0` jako dodatnie (N/E).
- [ ] **Lokalizacja:** Rozważyć, czy nazewnictwo kierunków (N/S/E/W) powinno być konfigurowalne (np. polskie P/P/W/Z) poprzez opcjonalny parametr lub feature flag.

---

---

- [ ] **Feature Gating:** Rozdzielić moduły `core::d2` i `core::d3` za pomocą atrybutów `#[cfg(feature = "dim2")]` / `#[cfg(feature = "dim3")]`.
- [ ] **TryFrom for DTO:** Rozważyć implementację `TryFrom<Cs<N>>` dla struktur DTO, co umożliwiłoby bezpieczny powrót z jądra obliczeniowego do modeli nazwanych z walidacją zakresów.
- [ ] **Cylindrical Rotation Check:** Wykonać testy jednostkowe sprawdzające, czy kierunki obrotu w `new_from_rfx` i `new_from_rfy` są zgodne z intuicją CAD (prawoskrętność).

---

---

- [ ] **Unit Tests - Vector Ops:** Implementacja testów dla `dot`, `cross` (2D/3D) oraz `angle_between` z wykorzystaniem wartości referencyjnych.
- [ ] **SIMD Optimization:** Rozważyć użycie atrybutów `#[repr(simd)]` (wymaga nightly) dla `Cs<2>` i `Cs<3>` w celu przyspieszenia operacji masowych na chmurach punktów.
- [ ] **ECEF Accuracy:** Przetestować `to_dms_sn_we_from_xyz` pod kątem błędów zaokrągleń przy biegunach (osobliwość asin).
- [ ] **Assignment Operators:** Zaimplementować traity `AddAssign`, `SubAssign`, `MulAssign` oraz `DivAssign` dla `Cs<N>`, aby umożliwić modyfikację wektorów w miejscu (in-place).
- [ ] **Angle Between - Edge Cases:** Przetestować stabilność `angle_between` dla wektorów o skrajnie różnych rzędach wielkości.
- [ ] **ECEF Reference Ellipsoids:** Rozważyć dodanie predefiniowanych stałych dla elipsoid (np. WGS84, GRS80) jako parametrów wejściowych dla konwersji ECEF.
- [ ] **Angle Normalization:** Sprawdzić, czy metody azymutalne powinny automatycznie normalizować wynik do zakresu [0, 2π).
- [ ] **to_rf Consistency:** Zweryfikować, czy konwersja `to_rf_from_xy` nie powinna zwracać dedykowanej struktury DTO zamiast surowego `Cs<2>`, aby uniknąć pomyłek R/Phi z X/Y.
- [ ] **Coordinate Precision:** Zweryfikować błędy zaokrągleń przy konwersji `to_ecef` dla punktów znajdujących się w skrajnie dużej odległości od środka układu.
- [ ] **Angle Ambiguity:** Dodać testy sprawdzające stabilność kątów kompasowych (`arctan_x_y` etc.) na granicach przedziałów.
- [ ] **DMS Validation:** Dodać walidację wejściowych parametrów DMS (sekundy < 60, minuty < 60) przed konwersją do radianów.

---

---

- [ ] **ANSI Color Support:** Rozważyć użycie kodów kolorów ANSI zamiast (lub obok) emoji, aby poprawić czytelność w terminalach nieobsługujących pełnego zestawu Unicode.
- [ ] **Custom Formatter:** Wprowadzić strukturę `CsFormatter`, która pozwoli na globalne ustawienie precyzji wyświetlanych liczb (obecnie używane jest domyślne `{:?}`).
- [ ] **Log Integration:** Przygotować implementację traitu `Display` lub integrację z makrem `log::debug!`, aby uniknąć bezpośredniego używania `println!` wewnątrz biblioteki.

---

---

- [ ] **Eksport DTO z modelem Wildcard:** Przeanalizować, czy użycie `pub use model::*;` nie spowoduje zanieczyszczenia przestrzeni nazw (Namespace Pollution) w miarę dodawania nowych układów DTO. Rozważyć wprowadzenie modułu `prelude`.

---

---

- [ ] **Linter & Docs Atrybuty:** Rozważyć dodanie na górze pliku `libs.rs` (lub `lib.rs`) rygorystycznych atrybutów, np. `#![warn(missing_docs)]` oraz `#![deny(clippy::unwrap_used)]`, aby wymusić rygor dokumentacyjny i bezpieczeństwo (brak panik) w całym drzewie projektu.
- [ ] **no_std Support:** Przeanalizować możliwość dodania atrybutu `#![no_std]` (wymagałoby użycia biblioteki `libm` dla funkcji trygonometrycznych na rzecz wbudowanego `std`), co otworzyłoby bibliotekę na systemy Embedded i lekkie środowiska WASM.

---

---

- [ ] **Globalne Lintery:** Dodać na początku `src/lib.rs` dyrektywy `#![warn(missing_docs)]` oraz `#![deny(clippy::unwrap_used)]` w celu zablokowania możliwości wdrożenia nieudokumentowanego lub panikującego kodu.
- [ ] **Feature Flags w Crate Root:** Jeśli wprowadzimy system `features` (np. dla wyłączania modułu 3D), `src/lib.rs` będzie miejscem do warunkowego eksportowania (np. `#[cfg(feature = "full")] pub mod libs;`).

---

---

- [ ] **Fasada w przykładach:** Zaktualizować importy w `basic_usage.rs`, aby używały skróconych ścieżek z fasady `cs.rs` (np. `use cad_cs::CoordsPolar;` zamiast pełnej ścieżki do `model`), w celu testowania publicznego API tak, jak widzi to użytkownik zewnętrzny.
- [ ] **Rozszerzenie Examples:** Dodać osobny plik `examples/advanced_geodesy.rs` skupiający się wyłącznie na konwersjach ECEF i operacjach krzyżowych, odciążając tym samym `basic_usage.rs`.

---

---
