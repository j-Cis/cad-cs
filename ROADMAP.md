# 🗺️ ROADMAP (Plany Rozwoju)

Ten dokument określa strategiczne cele i planowane funkcjonalności dla biblioteki `cad-cs` na przyszłe wydania (v0.2.0 i wyżej).

---

## 1. Rozszerzenie Systemów Kątowych i Geodezyjnych

Obecnie biblioteka natywnie wspiera format DMS (Stopnie, Minuty, Sekundy) oraz Radiany. Planujemy rozbudowę o bardziej nowoczesne standardy zapisu.

* [ ] **Wsparcie dla formatu DDD (Decimal Degrees):**
  * **📚 [POL]:** Dodanie pełnej obsługi formatu stopni dziesiętnych (np. `50.2489°`). Jest to krytyczne dla natywnej współpracy z nowoczesnymi interfejsami API (np. Google Maps, GeoJSON).
  * **📚 [ENG]:** Add full support for Decimal Degrees format. Critical for native interoperability with modern web APIs (e.g., Google Maps, GeoJSON).
  * *Implementacja:* Wymaga dodania struktur DTO dla DDD, makra `ddd!` oraz funkcji przejścia pomiędzy jądrem `Cs` a nowym izolatem.

## 2. Zaawansowane Rzutowanie Przestrzenne (Polyhedron Projections)

Wychodzimy poza standardowe rzuty na płaszczyzny kartezjańskie (XY, XZ, YZ) oraz ECEF.

* [ ] **Rzutowanie Lat/Lon na Wielościany (Polyhedron):**
  * **📚 [POL]:** Implementacja algorytmów rzutowania współrzędnych sferycznych (Szerokość/Długość) na zdefiniowane, płaskie ściany wielościanów foremnych (np. Dwudziestościan/Icosahedron, Sześcian/Cube).
  * **📚 [ENG]:** Implementation of projection algorithms from spherical coordinates (Lat/Lon) onto defined, flat faces of regular polyhedrons.
  * *Cel:* Umożliwienie tworzenia map topologicznych (np. mapy Dymaxion Fullera) i rozwijanie sfery na płaskie siatki badawcze.

## 3. Topologia Sferyczna (Spherical Spatial Analysis)

Rozszerzenie obecnych, prostych analiz podziału przestrzeni (ćwiartki 2D, oktanty 3D) na zaawansowaną geometrię wielościenną.

* [ ] **Identyfikacja w Wielościanach Sferycznych (Spherical Polyhedrons):**
  * **📚 [POL]:** Zamiast ograniczania się do 8 oktanów, system będzie w stanie określić, do której "ściany" (obszaru) zdefiniowanego wielościanu sferycznego należy dany wektor 3D / punkt LatLon.
  * **📚 [ENG]:** Instead of limiting to 8 octants, the system will determine which "face" (region) of a defined spherical polyhedron a given 3D vector / LatLon point belongs to.
  * *Mechanizm:* Konstrukcja brył wpisanych w sferę bazową. Funkcja (np. `spherical_face()`) zwróci identyfikator regionu zdefiniowanego wielościanu (np. 1 z 20 trójkątów sferycznych dla icosahedronu), zastępując tradycyjne wywołanie `q()`.
