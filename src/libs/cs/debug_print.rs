// src/libs/cs/debug_print.rs
use crate::libs::angle::{AngleExt, AngleFmt};
use crate::libs::cs::model::Cs;
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
    fn print_dms_sn_we(&self, name: &str);
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
    fn print_dms_sn_we(&self, name: &str) {
        // Pobieramy nasz rozszerzony trait dla formatowania znaków
        use crate::libs::cs::utils::SignStrExt;

        // Dekompresujemy ECEF (XYZ) do DTO z DMS
        let dms = self.to_dms_sn_we_from_xyz();

        // Wyciągamy litery kierunków na podstawie znaków (N/S i E/W)
        let lat_dir = dms.sn_lat_d.sign_sn();
        let lon_dir = dms.we_lon_d.sign_we();

        // Wyświetlamy dokładnie jak na zrzucie ekranu z Google Maps!
        // {:02} dodaje zera wiodące do minut (np. 8 -> 08)
        // {:04.1} dodaje zera i ustala 1 miejsce po przecinku dla sekund (np. 2.3 -> 02.3)
        println!(" {} 🌍 {}°{:02}'{:04.1}\"{} {}°{:02}'{:04.1}\"{}",
            name,
            dms.sn_lat_d.abs(), dms.sn_lat_m, dms.sn_lat_s, lat_dir,
            dms.we_lon_d.abs(), dms.we_lon_m, dms.we_lon_s, lon_dir
        );
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
        self.print_dms_sn_we(&spacer);
        println!(" ");
    }
}
