# 🏛️ Architektura i Konwencje Nazewnicze `cad-cs`

Poniższy dokument stanowi legendę i klucz symboli używanych w całym ekosystemie struktur `Cs<N>` oraz operacji matematycznych. Przestrzeganie tej konwencji pozwala na natychmiastowe zrozumienie typu operacji po samej nazwie metody.

## 🧭 Ćwiartki i oktanty

Zmienna `q` (od ang. *quadrant*) określa numer ćwiartki (w 2D) lub oktantu (w 3D) w układzie kartezjańskim.

* **2D (Płaszczyzna XY):** Wartości od `1` do `4`
  * `1`: (+X, +Y) | `2`: (-X, +Y) | `3`: (-X, -Y) | `4`: (+X, -Y)
* **3D (Przestrzeń XYZ):** Wartości od `1` do `8`
  * Górna półprzestrzeń (+Z): `1` do `4` (jak w 2D)
  * Dolna półprzestrzeń (-Z): `5`: (+X, +Y, -Z) | `6`: (-X, +Y, -Z) | `7`: (-X, -Y, -Z) | `8`: (+X, -Y, -Z)

### Wizualizacja znakowa (`q_sign`)

Alternatywna reprezentacja logiczna dla ćwiartki/oktantu, zwracająca tablicę znaków kierunkowych.

* **2D:** `["+", "-"]` (np. dla punktu `-1.0, 2.0` → `q=2`, `q_sign=["-", "+"]`)
* **3D:** `["+", "-", "+"]` (np. dla punktu `-1.0, -2.0, 3.0` → `q=3`, `q_sign=["-", "-", "+"]`)

---

## 📐 Typy Współrzędnych i DTO

Sufiksy przy metodach (np. `to_rf_from_xy()`) oraz nazwy obiektów DTO (Domain Transfer Objects) bezpośrednio wskazują na rodzaj układu.

| Symbol | Klasa DTO | Opis Układu |
| :--- | :--- | :--- |
| **`xy`** | `Coords2dXy` | Współrzędne 2D kartezjańskie (x,y) na płaszczyźnie XY lub lub wektor (x,y)  lub płaszczyzna XY |
| **`xz`** | `Coords2dXz` | Współrzędne 2D kartezjańskie (x,z) na płaszczyźnie XZ lub lub wektor (x,z)  lub płaszczyzna XZ |
| **`yz`** | `Coords2dYz` | Współrzędne 2D kartezjańskie (y,z) na płaszczyźnie YZ lub lub wektor (y,z)  lub płaszczyzna YZ |
| **`rf`** | `Coords2dXyPolar` | Współrzędne 2D biegunowe/cylindryczne (R, Φ) |
| **`rf`** | `Coords2dXzPolar` | Współrzędne 2D biegunowe/cylindryczne (R, Φ) |
| **`rf`** | `Coords2dZyPolar` | Współrzędne 2D biegunowe/cylindryczne (R, Φ) |
| **`rfx`** | `Coords3dYzCylindricalX` | Współrzędne 3D cylindryczne względem osi X (R_yz, Φ, X) |
| **`rfy`** | `Coords3dXzCylindricalY` | Współrzędne 3D cylindryczne względem osi Y (R_xz, Φ, Y) |
| **`rfz`** | `Coords3dXyCylindricalZ` | Współrzędne 3D cylindryczne względem osi Z (R_xy, Φ, Z) |
| **`xyz`** | `Coords3dXyz` | Współrzędne 3D kartezjańskie (x,y,z) / Przestrzeń XYZ |
| **`rft`** | `Coords3dXyzSpherical` | Współrzędne 3D sferyczne (R, Φ, Θ) |

---

## 🔄 Kąty (Azymuty i Inklinacje)

Litery używane do oznaczania kątów różnią się w zależności od ich funkcji w przestrzeni:

* **`f` (Φ / Azymut):** Kąt obrotu w płaszczyźnie 2D
  * w płaszczyźnie XY/XZ/YZ lub
  * w układach sferycznych RΦΘ lub
  * w ukłądach cylindrycznych RΦX/RΦY/RΦZ.
* **`t` (Θ / Inklinacja):** Kąt odchylenia
  * (np. w układzie sferycznym od osi Z/Y/X).

**Klucze metod konwersji kątowych:**

* `y_x` (czyli `f_yx`) → $\arctan(\frac{y}{x})$ – azymut w płaszczyźnie XY
* `z_x` (czyli `f_zx`) → $\arctan(\frac{z}{x})$ – azymut w płaszczyźnie XZ
* `z_y` (czyli `f_zy`) → $\arctan(\frac{z}{y})$ – azymut w płaszczyźnie YZ
* `x_rxyz` (czyli `t_xr`) → $\arccos(\frac{x}{rxyz})$ – inklinacja względem osi X
* `y_rxyz` (czyli `t_yr`) → $\arccos(\frac{y}{rxyz})$ – inklinacja względem osi Y
* `z_rxyz` (czyli `t_zr`) → $\arccos(\frac{z}{rxyz})$ – inklinacja względem osi Z

---

## 📏 Promienie, Długości i Projekcje

* **`{x, y, z}`** – Bazowe współrzędne 2D/3D kartezjańskie.
* **`r`** – Uniwersalny promień / długość wektora.
* **`rxy`** – Długość rzutu wektora na płaszczyznę 2D XY.
* **`rxz`** – Długość rzutu wektora na płaszczyznę 2D XZ.
* **`ryz`** – Długość rzutu wektora na płaszczyznę 2D YZ.
* **`rxyz`** – Pełna długość wektora w przestrzeni 3D.

---

## 🛠️ Podstawowe Operacje Wektorowe

Biblioteka implementuje matematykę przestrzenną zgodną z nazewnictwem algebraicznym:

* `add`, `sub` – Dodawanie i odejmowanie wektorów (Translacja).
* `dot` – Iloczyn skalarny (Dot product). Zwraca wartość `f64`.
* `cross` – Iloczyn wektorowy (Tylko 3D). Zwraca nowy wektor ortogonalny.
* `perp` – Wektor prostopadły w lewo/CCW (Tylko 2D).
* `angle_between` – Zwraca kąt między dwoma wektorami.
* `normalize_r<plaszczyzna>_projection` – Normalizuje wektor w taki sposób, aby jego rzut na daną płaszczyznę (np. XY) miał długość 1.0.

---

## ⚖️ Fundamenty Numeryczne

Biblioteka opiera się na stałej `EPSILON_SPATIAL = 1e-9`.
Wszelkie porównania logiczne (równość punktów, zerowanie wektorów) MUSZĄ przechodzić przez funkcje `is_zero` lub `is_equal`.
Wartości kątowe są zawsze procesowane w `f64` (radiany), a konwersja na stopnie/DMS służy wyłącznie do prezentacji lub I/O.

---

## 📐 Rygor Wymiarowości (`Cs<N>`)

Projekt `cad-cs` świadomie rezygnuje z obsługi geometrii wielowymiarowej (N > 3).

* **Cs2 (N=2):** Reprezentacja płaszczyzny XY.
* **Cs3 (N=3):** Reprezentacja przestrzeni XYZ.

Ograniczenie to jest wymuszone na poziomie kompilacji poprzez trait `Dim`. Próba użycia struktur dla innych wartości `N` skutkuje błędem kompilacji, co gwarantuje stabilność algorytmów geometrycznych.

### Inicjalizacja Makrowa

W celu uproszczenia składni i zapewnienia bezpieczeństwa typów, stosowane są makra:

* `cs!`: Wymusza podanie dokładnie 2 lub 3 argumentów, automatycznie rzutując je na `f64`.
* `dms! / dms_angle!`: Standardowe wejście dla danych geodezyjnych/kątowych, izolujące logikę przeliczeń DMS -> Radiany od reszty systemu.

---

## ⌨️ Ergonomia Programistyczna (Shorthand API)

Biblioteka udostępnia globalne funkcje pomocnicze w module `angle`, które skracają proces inicjalizacji typów opakowanych.

* Zamiast pełnej ścieżki `Angle::from_deg(90.0)`, programista może użyć bezpośredniej funkcji `deg(90.0)`.
* Wszystkie funkcje pomocnicze są oznaczone jako `#[inline]`, co eliminuje narzut wywołania funkcji (zero-cost abstraction).
* Wykorzystanie `#[rustfmt::skip]` przy tych funkcjach jest świadomym zabiegiem mającym na celu utrzymanie maksymalnej gęstości pionowej kodu API.

---

## ➗ Reprezentacja Wymierna i Aproksymacja

W celu poprawy czytelności danych wyjściowych (np. w logach i interfejsach użytkownika), biblioteka oferuje mechanizm konwersji liczb zmiennoprzecinkowych na ułamki zwykłe.

* **Algorytm:** Wykorzystujemy ułamki łańcuchowe, co pozwala na znalezienie najlepszego przybliżenia wymiernego dla danej liczby `f64`.
* **Precyzja:** Proces aproksymacji jest sterowany przez jądro `tolerance`. Gdy różnica między wartością a ułamkiem spadnie poniżej `EPSILON_SPATIAL`, algorytm kończy pracę.
* **Limity:** Mianownik jest ograniczony do wartości 1 000 000, co zapobiega generowaniu skomplikowanych ułamków dla liczb zaszumionych numerycznie.

---

## 📦 Modele Koordynacji (DTO)

Biblioteka separuje surowe obliczenia na `Cs<N>` od struktur wejściowych/wyjściowych (DTO).

* **Separacja:** Struktury w `model_coords.rs` służą do czytelnej inicjalizacji danych i komunikacji z zewnętrznymi modułami (np. UI lub bazą danych).
* **Konwersja:** Każda struktura DTO posiada implementację `Into<Cs<N>>`, co pozwala na bezproblemowe przejście do wydajnych operacji wektorowych.
* **DMS Standard:** Przyjęto system `Nz90Ex0`, gdzie szerokość geograficzna (N/S) jest procesowana przed długością (E/W), co jest zgodne ze standardami ISO 6709.

---

## 📐 Płaszczyzny Robocze i Układy Cylindryczne

Biblioteka `cad-cs` wspiera pełną ortogonalność układów współrzędnych. Poza standardową płaszczyzną XY, zaimplementowano natywne wsparcie dla płaszczyzn XZ i YZ:

* **XY / Oś Z:** Standardowa orientacja "z góry". Układ cylindryczny bazuje na promieniu `r_xy`.
* **XZ / Oś Y:** Orientacja "z boku" (front/back). Układ cylindryczny bazuje na promieniu `r_xz`.
* **YZ / Oś X:** Orientacja "z boku" (left/right). Układ cylindryczny bazuje na promieniu `r_yz`.

Taka struktura pozwala na bezpośrednie rzutowanie punktów 3D na dowolną z trzech głównych płaszczyzn kartezjańskich bez konieczności ręcznej zamiany składowych wektora przed obliczeniami biegunowymi.

---

## 🔡 Semantyka Znaków (SignStrExt)

Biblioteka mapuje wartości liczbowe na reprezentacje tekstowe zgodnie ze standardami inżynieryjnymi i geodezyjnymi:

* **Matematyka Ogólna:** Wartości `>= 0` otrzymują znak `+`.
* **Geodezja (Szerokość):** Wartości `>= 0` mapowane są na `N` (Północ), wartości `< 0` na `S` (Południe).
* **Geodezja (Długość):** Wartości `>= 0` mapowane są na `E` (Wschód), wartości `< 0` na `W` (Zachód).

Implementacja oparta na traitach rozszerzających (`Extension Traits`) pozwala na wywoływanie tych metod bezpośrednio na typach prymitywnych (`f64`, `i8`, `i16`), co upraszcza kodowanie warstwy UI i logowania.

---

## 🔄 Rdzeń Konwersji (Core)

Struktura `core/` zarządza przepływem danych między modelami bezpiecznymi (DTO) a jądrem obliczeniowym `Cs<N>`.

* **Wzorzec Shorthand:** Metody `new_as_..._from_...` pozwalają na używanie struktury `Cs<N>` jako tymczasowego kontenera na parametry (np. R, Φ), co redukuje potrzebę tworzenia wielu małych alokacji na stosie przed konwersją na wektor końcowy.
* **Efektywność:** Wszystkie konwersje trygonometryczne wykorzystują metodę `sin_cos()`, co na większości nowoczesnych procesorów pozwala na obliczenie obu wartości w jednym cyklu jednostki FPU.

### Orientacja w rzutach bocznych

W celu zachowania spójności z prawoskrętnym układem współrzędnych, definicje kątów w układach cylindrycznych są następujące:

* **Oś X (YZ):** Kąt zero leży na dodatniej osi Y, rośnie w stronę dodatniej osi Z.
* **Oś Y (XZ):** Kąt zero leży na dodatniej osi X, rośnie w stronę dodatniej osi Z.
* **Oś Z (XY):** Kąt zero leży na dodatniej osi X, rośnie w stronę dodatniej osi Y.

Wszystkie operacje bazują na strukturze tablicowej, co pozwala na optymalizację pętli przez kompilator (unrolling).

---

## 🧮 Operacje Wektorowe i Norma

Wszystkie operacje bazują na strukturze tablicowej, co pozwala na optymalizację pętli przez kompilator (unrolling).

* **Dot Product:** Bazowy mechanizm dla rzutowania i obliczeń kątowych.
* **2D Cross:** Realizowany jako wyznacznik macierzy 2x2, zwraca skalar reprezentujący zorientowane pole powierzchni.
* **3D Cross:** Zwraca wektor prostopadły, kluczowy dla obliczeń normalnych powierzchni w CAD.
* **Normalizacja:** Implementujemy `normalize_r_projection`, która chroni przed błędem dzielenia przez zero (Division by Zero) dzięki integracji z `tolerance::is_zero`.

## 📐 Algebra Wektorowa (Generic Math)

Moduł `math.rs` implementuje operacje wektorowe przy użyciu stałych generycznych (`const generics`).

* **Wspólna Implementacja:** Dzięki ograniczeniu `where Cs<N>: Dim`, metody takie jak `dot()`, `add()` czy `normalize_r_projection()` są definiowane raz dla wszystkich wspieranych wymiarów.
* **Wykorzystanie tablic:** Operacje bazują na `std::array::from_fn`, co pozwala kompilatorowi na agresywną optymalizację pętli i potencjalną automatyczną wektoryzację.
* **Przeciążanie Operatorów:** Biblioteka implementuje standardowe traity z `std::ops`, umożliwiając intuicyjny zapis równań matematycznych (`a + b * 2.0`).

## 🗺️ Geodezja i Rzuty 2D

Moduł `math/d2.rs` wykracza poza standardową geometrię euklidesową, wprowadzając funkcje dedykowane dla systemów informacji geograficznej (GIS):

* **Model ECEF (Earth-Centered, Earth-Fixed):** Metoda `to_ecef_from_rad_sn_we` pozwala na bezpośrednią konwersję współrzędnych sferycznych (szerokość/długość) na trójwymiarowy wektor kartezjański, co stanowi pomost między danymi GPS a silnikiem CAD.
* **Dualizm Azymutalny:** Biblioteka rozróżnia azymut matematyczny (mierzony od osi X) od azymutu geodezyjnego (mierzony od osi Y/Północy), co eliminuje błędy interpretacji danych wejściowych.
* **Analiza Kwadrantowa:** Wbudowana obsługa ćwiartek i znaków kierunkowych (`q()`, `q_sign()`) ułatwia logikę decyzyjną w algorytmach przycinania i nawigacji.

## 🌐 Geometria Przestrzenna i Geodezja (3D)

Moduł `math/d3.rs` stanowi fundament dla zaawansowanych operacji CAD i GIS:

* **Analiza rzutowa:** System natywnie wspiera obliczenia promieni (`rxy`, `rxz`, `ryz`) oraz azymutów na trzech głównych płaszczyznach kartezjańskich. Pozwala to na płynne przechodzenie między widokami roboczymi (góra/front/bok).
* **Normalizacja rzutowa:** Unikalne funkcje `normalize_*_projection` umożliwiają operacje na wektorach kierunkowych rzutowanych na płaszczyzny, co jest kluczowe w algorytmach oświetlenia i rzutowania prostopadłego.
* **Model ECEF/DMS:** Pełna implementacja dwukierunkowej konwersji między kartezjańskim układem odśrodkowym (ECEF) a geodezyjnym formatem DMS (Stopnie, Minuty, Sekundy). System uwzględnia osobliwość numeryczną w jądrze Ziemi (r=0).
* **Podział Oktantowy:** System `q()` rozszerza analizę ćwiartek do przestrzeni 3D, identyfikując jeden z ośmiu obszarów przestrzeni.

---

## 📝 Prezentacja i Diagnostyka (Debug Print)

Moduł `debug_print.rs` separuje logikę wyświetlania od matematycznego jądra, implementując traity rozszerzające `Cs2ConsoleDebug` i `Cs3ConsoleDebug`.

* **Wizualizacja Układów:** Każdy typ współrzędnych posiada unikalny identyfikator emoji, co ułatwia analizę logów w złożonych procesach konwersji (np. z rzutu walcowego na sferyczny).
* **Semantyka DMS:** Wyświetlanie współrzędnych geograficznych naśladuje standardowe interfejsy GIS, wykorzystując zera wiodące dla minut i sekund oraz automatyczne mapowanie znaków na litery kierunkowe (N, S, E, W).
* **Elastyczność Kątowa:** Dzięki integracji z `AngleFmt`, diagnostyka może odbywać się w radianach, stopniach lub ułamkach PI, zależnie od kontekstu debugowanego problemu.

---

## 🚪 Architektura API (Facade Pattern)

Mimo głębokiego i rygorystycznego podziału wewnętrznego plików (np. separacja `math/d2.rs` i `core/d3.rs`), biblioteka stosuje wzorzec Fasady (Facade) w głównym pliku `cs.rs`.

* **Spłaszczenie (Flattening):** Za pomocą instrukcji `pub use` krytyczne struktury (`Cs`, `Cs2`, moduły DTO) są wyciągane na najwyższy poziom modułu. Użytkownik wywołuje `cad_cs::Cs2`, nie wiedząc o istnieniu `cad_cs::cs::model::Cs2`.
* **Niejawne Ładowanie Implementacji:** Ze względu na specyfikę kompilatora Rusta, metody przypięte przez `impl Cs<N>` w podmodułach (np. z `math.rs`) są automatycznie dostępne dla każdego obiektu `Cs<N>` bez konieczności ich ręcznego importowania przez użytkownika końcowego.

---

## 🗂️ Top-Level Architektura (libs.rs)

Biblioteka została podzielona na cztery fundamentalne, ortogonalne filary, zdefiniowane w `src/libs.rs`:

* **`tolerance`:** Baza całego systemu. Zapewnia bezpieczne porównywanie wartości zmiennoprzecinkowych `f64`. Pozostałe moduły bezwzględnie na nim polegają.
* **`frac` i `angle`:** Moduły pomocnicze, izolujące logikę specyficzną dla ułamków oraz jednostek miar kątowych (Type Safety).
* **`cs` (Coordinate System):** Główne jądro wektorowe i geodezyjne (CAD/GIS). Konsumuje usługi pozostałych trzech modułów w celu dostarczenia bezpiecznego interfejsu numerycznego.

---

## 🚪 Punkt Wejścia (Crate Root)

Plik `src/lib.rs` jest celowo zredukowany do absolutnego minimum.

* **Delegacja Logiki:** Cała architektura i logika domenowa została wyeksportowana do katalogu `src/libs/` oraz zdefiniowana w `src/libs.rs`.
* **Czystość Przestrzeni Nazw:** Zabezpiecza to główny obszar roboczy przed bałaganem i ułatwia ewentualne wydzielenie części kodu do osobnych crate'ów (np. w przypadku monorepo workspace) w przyszłości.

---

## 💡 Wzorce Użycia (Usage Patterns)

Biblioteka `cad-cs` promuje dwa główne wzorce inicjalizacji i transferu danych, co zostało udokumentowane w katalogu `examples/`:

1. **Szybka Inicjalizacja (Zero-cost):** Użycie makr `cs!` oraz `dms!` do bezpośredniego wstrzykiwania surowych danych numerycznych w postaci wektorów. Zoptymalizowane pod kątem wydajności.
2. **Bezpieczna Inicjalizacja (Type-safe):** Wykorzystanie struktur DTO (np. `Coords3dXyzSpherical`) z implementacją traitów `Into/From`. Zabezpiecza przed pomyleniem kolejności parametrów wejściowych (np. R z Azymutem).

---
