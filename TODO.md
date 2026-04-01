# 📋 TODO: Projekt `cad-cs`

## 🚀 Plan na przyszłość

### Testy i Niezawodność

- [ ] **Testy jednostkowe (Unit Tests):** Aktualnie `cargo test` zgłasza brak testów (`running 0 tests`). Kluczowe będzie dopisanie modułu testowego dla głównych operacji matematycznych:
  - Konwersje układów współrzędnych (ECEF, DMS, sferyczne, biegunowe).
  - Weryfikacja działania `EPSILON_SPATIAL` oraz tolerancji numerycznej (`is_equal`).
  - Sprawdzenie operatorów na wektorach `Cs<N>`.
- [ ] **Testy dokumentacji (Doc Tests):** Dopisanie w dokumentacji (komentarze `///`) małych przykładów użycia każdej metody, co `cargo test` sam sprawdzi pod kątem poprawności.

### Dług technologiczny (Drobne poprawki)

- [ ] **Zabezpieczenie tagu HTML w dokumentacji:** Składnia `Cs<N>` w pliku `src/libs/cs/model.rs` (linia 55) musi zostać ujęta w grawisy (`` `Cs<N>` ``), żeby `cargo doc` nie uznawał `<N>` za niezamknięty tag języka HTML.
- [ ] **Oczyszczenie importów:** Nieużywany import `Cs2ConsoleDebug` w `examples/basic_usage.rs` nie został naprawiony przez `cargo fix` i nadal wymaga ręcznego usunięcia.

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
