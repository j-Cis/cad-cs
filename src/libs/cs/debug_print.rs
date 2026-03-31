// src/libs/cs/debug_print.rs
use crate::libs::cs::model::Cs;
use crate::libs::angle::{AngleExt, AngleFmt};
// UWAGA: Potrzebuje też dostępu do metod matematycznych z innych bloków impl!
// Ponieważ w Ruście metody rozszerzające (traitowe) muszą być w scope, a Twoje
// metody to bezposrednie "impl Cs", będą widoczne o ile tylko typ Cs jest znany.

// ===================================================================================
// HELPERY PRYWATNE DLA MODUŁU DEBUG
// ===================================================================================


/// Prywatny helper do dynamicznego formatowania kątów.
/// Żyje tylko w warstwie prezentacji, z dala od czystej matematyki.
#[rustfmt::skip] #[inline]
fn print_fmt_ang(val: f64, fmt: AngleFmt) -> String {
    match fmt {
        AngleFmt::Rad => val.rad().print_rad(),
        AngleFmt::Deg => val.rad().print_deg(),
        AngleFmt::PiFrac => val.rad().print_pi_frac(),
    }
}

// ===================================================================================
// TRAIT DLA 2D
// ===================================================================================

pub trait Cs2ConsoleDebug {
    fn print_q(&self, name: &str);
    fn print_xy(&self, name: &str);
    fn print_rf(&self, name: &str, fmt: AngleFmt);
    fn print(&self, name: &str, fmt: AngleFmt);
}

impl Cs2ConsoleDebug for Cs<2> {
    #[rustfmt::skip] #[inline]
    fn print_q(&self, name: &str) {
      let s = self.q_sign();
      println!(" {}    (Ćwiartka: {} [{}, {}])", name, self.q(), s[0], s[1]);
    }

    #[rustfmt::skip] #[inline]
    fn print_xy(&self, name: &str) {
        println!(" {} 🔷 (x: {:?}, y: {:?})", name, self.0[0], self.0[1]);
    }

    #[rustfmt::skip] #[inline]
    fn print_rf(&self, name: &str, fmt: AngleFmt) {
        let rf = self.to_rf_from_xy();
        println!(" {} 🟪 (R: {:?}, Φ: {})", name, rf[0], print_fmt_ang(rf[1], fmt));
    }

    #[rustfmt::skip] #[inline]
    fn print(&self, name: &str, fmt: AngleFmt) {
        let spacer = " ".repeat(name.chars().count());
        self.print_xy(name);
        self.print_q(&spacer);
        self.print_rf(&spacer, fmt);
        println!(" ");
    }
}

// ===================================================================================
// TRAIT DLA 3D
// ===================================================================================

pub trait Cs3ConsoleDebug {
    fn print_q(&self, name: &str);
    fn print_xyz(&self, name: &str);
    fn print_rft(&self, name: &str, fmt: AngleFmt);
    fn print_rfx(&self, name: &str, fmt: AngleFmt);
    fn print_rfy(&self, name: &str, fmt: AngleFmt);
    fn print_rfz(&self, name: &str, fmt: AngleFmt);
    fn print(&self, name: &str, fmt: AngleFmt);
}

impl Cs3ConsoleDebug for Cs<3> {
    #[rustfmt::skip] #[inline]
    fn print_q(&self, name: &str) {
      let s = self.q_sign();
      println!(" {}    (Oktant: {} [{}, {}, {}])", name, self.q(), s[0], s[1], s[2]);
    }

    #[rustfmt::skip] #[inline]
    fn print_xyz(&self, name: &str) {
        println!(" {} 🔶 (x: {:?}, y: {:?}, z: {:?})", name, self.0[0], self.0[1], self.0[2]);
    }
    
    #[rustfmt::skip] #[inline]
    fn print_rft(&self, name: &str, fmt: AngleFmt) {
        let rft = self.to_rft_from_xyz();
        println!(" {} 🟫 (R: {:?}, Φ: {}, Θ: {})", name, rft[0], print_fmt_ang(rft[1], fmt), print_fmt_ang(rft[2], fmt));
    }

    #[rustfmt::skip] #[inline]
    fn print_rfx(&self, name: &str, fmt: AngleFmt) {
        let rfx = self.to_rfx_from_xyz();
        println!(" {} 🟧 (R: {:?}, Φ: {}, x: {:?})", name, rfx[0], print_fmt_ang(rfx[1], fmt), rfx[2]);
    }

    #[rustfmt::skip] #[inline]
    fn print_rfy(&self, name: &str, fmt: AngleFmt) {
        let rfy = self.to_rfy_from_xyz();
        println!(" {} 🟨 (R: {:?}, Φ: {}, y: {:?})", name, rfy[0], print_fmt_ang(rfy[1], fmt), rfy[2]);
    }

    #[rustfmt::skip] #[inline]
    fn print_rfz(&self, name: &str, fmt: AngleFmt) {
        let rfz = self.to_rfz_from_xyz();
        println!(" {} 🟥 (R: {:?}, Φ: {}, z: {:?})", name, rfz[0], print_fmt_ang(rfz[1], fmt), rfz[2]);
    }

    #[rustfmt::skip] #[inline]
    fn print(&self, name: &str, fmt: AngleFmt) {
        let spacer = " ".repeat(name.chars().count());
        self.print_xyz(name);
        self.print_q(&spacer);
        self.print_rft(&spacer, fmt);
        self.print_rfz(&spacer, fmt);
        self.print_rfy(&spacer, fmt);
        self.print_rfx(&spacer, fmt);
        println!(" ");
    }
}



// TODO: Wdrożenie Metody D - "Formatter Adapter Pattern" (Zatwierdzona ścieżka rozwoju)
// ---------------------------------------------------------------------------------------
// CEL: Całkowita separacja warstwy prezentacji od logiki wyjścia (stdout). 
// Obecna Metoda A (CsConsoleDebug) jest wygodna, ale "sztywno" wiąże nas z println!.
//
// KONCEPCJA:
// Zamiast metod .print(), stworzymy adaptery (lekkie struktury), które implementują 
// trait `std::fmt::Display`. Dzięki temu punkt będzie można wypisać nie tylko na konsolę, 
// ale też do logów, plików, czy przesłać jako String do UI.
//
// PRZEWIDYWANA ERGONOMIA:
// let pt = cs![1.0, 2.0, 3.0];
// println!("Punkt kontrolny: {}", pt.display_as("P1", AngleFmt::Deg));
// 
// ZALETY:
// 1. Wsparcie dla makr systemowych: format!, write!, panic!, log!.
// 2. Leniwa ewaluacja (Lazy): Formantowanie tekstu następuje dopiero w momencie 
//    rzeczywistego żądania zapisu, a nie w trakcie wywołania metody.
// 3. Czystość: Usunięcie bezpośrednich zależności od `std::io` z rdzenia matematycznego.
//
// SZKIC IMPLEMENTACJI:
// struct CsFormatter<'a, const N: usize> { 
//     source: &'a Cs<N>, 
//     name: &'a str, 
//     fmt: AngleFmt 
// }
//
// impl<'a, const N: usize> std::fmt::Display for CsFormatter<'a, N> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // Tutaj przenosimy logikę z obecnych metod print, używając writeln!(f, ...)
//     }
// }
// ---------------------------------------------------------------------------------------


