// 📃 ./src/libs/cs.rs

// --- REJESTRACJA PODMODUŁÓW ---

/// 📚 【 POL】: Rdzeń konwersji pomiędzy strukturami DTO a jądrem wektorowym.
/// 📚 【 ENG】: Conversion core between DTO structures and the vector kernel.
pub mod core;

/// 📚 【 POL】: Warstwa prezentacji i diagnostyki konsolowej (odseparowana od matematyki).
/// 📚 【 ENG】: Presentation and console diagnostics layer (separated from mathematics).
pub mod helper;

/// 📚 【 POL】: Implementacje operacji wektorowych, rzutów i transformacji geodezyjnych.
/// 📚 【 ENG】: Implementations of vector operations, projections, and geodetic transformations.
pub mod math;

/// 📚 【 POL】: Definicja struktury bazowej `Cs<N>` oraz ograniczeń wymiarowych (trait Dim).
/// 📚 【 ENG】: Definition of the base `Cs<N>` structure and dimensional constraints (Dim trait).
/// 📚 【 POL】: Modele danych (DTO) reprezentujące punkty w różnych układach współrzędnych.
/// 📚 【 ENG】: Data models (DTO) representing points in various coordinate systems.
pub mod model;

/// 📚 【 POL】: Narzędzia pomocnicze, w tym rozszerzenia typów do formatowania znaków.
/// 📚 【 ENG】: Utility tools, including type extensions for sign formatting.
pub mod utils;

// --- FASADA API (RE-EKSPORTY) ---

/// 📚 【 POL】: Bezpośredni dostęp do jądra obliczeniowego i aliasów typów.
/// 📚 【 ENG】: Direct access to the computational kernel and type aliases.
pub use model::Cs;
/// 📚 【 POL】: Globalny eksport wszystkich struktur DTO dla wygody inicjalizacji.
/// 📚 【 ENG】: Global export of all DTO structures for initialization convenience.
pub use model::*;
pub use types::{Cs2, Cs3, Dim};

// --- ABSTRAKCJA ---

pub mod abstracts;
/// 📚 【 POL】: Eksport traitu formatowania, wymaganego dla metod takich jak `sign_sn()`.
/// 📚 【 ENG】: Export of the formatting trait, required for methods like `sign_sn()`.
pub use abstracts::AbstractSignStrExt;

// --- MAKRA ---

pub mod macros;

// --- TYPY / ALIASY ---

pub mod types;
