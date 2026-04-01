// 📃 ./src/libs/cs/helper/d3.rs

use crate::libs::cs::abstract_traits::{AbstractHelperCs3, AbstractMathCs3};
use crate::libs::{angle::AngleFmt, cs::model::Cs};

// ===================================================================================
// TRAIT DLA 3D
// ===================================================================================

impl AbstractHelperCs3 for Cs<3> {
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
		println!(" {} 🟫 (R: {:?}, Φ: {}, Θ: {})", name, rft[0], fmt.format(rft[1]), fmt.format(rft[2]));
	}

	#[rustfmt::skip] #[inline]
	fn print_rfx(&self, name: &str, fmt: AngleFmt) {
		let rfx = self.to_rfx_from_xyz();
		println!(" {} 🟧 (R: {:?}, Φ: {}, x: {:?})", name, rfx[0], fmt.format(rfx[1]), rfx[2]);
	}

	#[rustfmt::skip] #[inline]
	fn print_rfy(&self, name: &str, fmt: AngleFmt) {
		let rfy = self.to_rfy_from_xyz();
		println!(" {} 🟨 (R: {:?}, Φ: {}, y: {:?})", name, rfy[0], fmt.format(rfy[1]), rfy[2]);
	}

	#[rustfmt::skip] #[inline]
	fn print_rfz(&self, name: &str, fmt: AngleFmt) {
		let rfz = self.to_rfz_from_xyz();
		println!(" {} 🟥 (R: {:?}, Φ: {}, z: {:?})", name, rfz[0], fmt.format(rfz[1]), rfz[2]);
	}

	/// 📚 【 POL】: Wyświetla współrzędne w formacie geodezyjnym DMS (Stopnie, Minuty, Sekundy) z oznaczeniami N/S i E/W.
	/// 📚 【 ENG】: Displays coordinates in geodetic DMS (Degrees, Minutes, Seconds) format with N/S and E/W indicators.
	#[rustfmt::skip] #[inline]
	fn print_dms_sn_we(&self, name: &str) {
		// Pobieramy nasz rozszerzony trait dla formatowania znaków
		use crate::libs::cs::abstract_traits::AbstractSignStrExt;

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
