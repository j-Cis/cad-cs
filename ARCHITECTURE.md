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
