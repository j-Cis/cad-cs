<!-- markdownlint-disable MD024 -->

# usuwanie redundancji typów

## usuwam w `// 📃 ./src/libs/cs/model_coords.rs`

```rust
/*
 * ❌ pub struct CoordsXz2d {
 * ❌  pub x: f64,
 * ❌  pub z: f64,
 * ❌ }
 * ❌ pub struct CoordsYz2d {
 * ❌  pub y: f64,
 * ❌  pub z: f64,
 * ❌ }
 * ❌ pub struct CoordsPolarXz2d {
 * ❌  pub r_xz: f64,
 * ❌  pub f_zx: f64, // w radianach
 * ❌ }
 * ❌ pub struct CoordsPolarYz2d {
 * ❌  pub r_yz: f64,
 * ❌  pub f_zy: f64, // w radianach
 * ❌ }
 */
```

## usuwam w `// 📃 ./src/libs/cs/core/d2.rs`

```rust

impl From<CoordsXz2d> for Cs2 {
 /// 📚 【 POL】: Konwertuje kartezjańskie DTO XZ na wektor Cs2.
 /// 📚 【 ENG】: Converts Cartesian XZ DTO to a Cs2 vector.
 #[inline]
 fn from(c: CoordsXz2d) -> Self { Cs([c.x, c.z]) }
}

impl From<CoordsYz2d> for Cs2 {
 /// 📚 【 POL】: Konwertuje kartezjańskie DTO YZ na wektor Cs2.
 /// 📚 【 ENG】: Converts Cartesian YZ DTO to a Cs2 vector.
 #[inline]
 fn from(c: CoordsYz2d) -> Self { Cs([c.y, c.z]) }
}


impl From<CoordsPolarXz2d> for Cs2 {
 /// 📚 【 POL】: Konwertuje współrzędne biegunowe XZ na wektor kartezjański Cs2.
 /// 📚 【 ENG】: Converts XZ polar coordinates to a Cs2 Cartesian vector.
 #[inline]
 fn from(c: CoordsPolarXz2d) -> Self {
  let (sin_f, cos_f) = c.f_zx.sin_cos();
  Cs([c.r_xz * cos_f, c.r_xz * sin_f])
 }
}

impl From<CoordsPolarYz2d> for Cs2 {
 /// 📚 【 POL】: Konwertuje współrzędne biegunowe YZ na wektor kartezjański Cs2.
 /// 📚 【 ENG】: Converts YZ polar coordinates to a Cs2 Cartesian vector.
 #[inline]
 fn from(c: CoordsPolarYz2d) -> Self {
  let (sin_f, cos_f) = c.f_zy.sin_cos();
  Cs([c.r_yz * cos_f, c.r_yz * sin_f])
 }
}
```

## usuwam w ``

```rust

```
