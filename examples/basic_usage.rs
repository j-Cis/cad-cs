// examples/basic_usage.rs

// Importujemy makro zdefiniowane na poziomie głównego crate'a
use cad_cs::{cs, dms};

// Importujemy struktury z Twojej biblioteki
use cad_cs::libs::angle::AngleFmt;
use cad_cs::libs::cs::debug_print::{Cs2ConsoleDebug, Cs3ConsoleDebug};
use cad_cs::libs::cs::model_coords::{Coords2dXyPolar, Coords3dXyzSpherical};
use cad_cs::libs::cs::{Cs2, Cs3};
fn main() {
    println!("🚀 Uruchamiam demonstrację biblioteki cad-cs!\n");

    // =================================================================
    // 1. SZYBKIE TWORZENIE PRZEZ MAKRO
    // =================================================================
    let pt_2d = cs![3.0, 4.0];
    let pt_3d = cs![3.0, 4.0, 12.0];

    println!("🔷 Punkt 2D kartezjański: {:?}", pt_2d);
    println!("🔶 Punkt 3D kartezjański: {:?}", pt_3d);

    // Obliczenia na rdzeniu (metody z `math.rs` i `d2/d3.rs`)
    println!("📏 Promień 2D (rxy): {}", pt_2d.rxy());
    println!("📏 Promień 3D pełny (rxyz): {}", pt_3d.rxyz());
    println!("📐 Kąt (azymut) dla 2D: {:.4} rad", pt_2d.arctan_y_x());

    println!("\n----------------------------------------------------\n");

    // =================================================================
    // 2. BEZPIECZNE TWORZENIE PRZEZ DTO (Domain Transfer Objects)
    // =================================================================

    // Z układu biegunowego (R, Φ) na kartezjański Cs2
    let punkt_z_biegunowego: Cs2 = Coords2dXyPolar {
        r_xy: 10.0,
        f_yx: std::f64::consts::PI / 2.0, // 90 stopni (prosto w górę osi Y)
    }
    .into();

    println!(
        "🟪 Cs2 z biegunowego (R=10, Φ=90°): {:?}",
        punkt_z_biegunowego
    );

    // Z układu sferycznego (R, Φ, Θ) na kartezjański Cs3
    let punkt_ze_sferycznego: Cs3 = Coords3dXyzSpherical {
        r_xyz: 20.0,
        f_yx: 0.0,                        // Azymut w osi X
        t_zr: std::f64::consts::PI / 4.0, // Odchylenie 45 stopni od osi Z
    }
    .into();

    println!(
        "🟫 Cs3 ze sferycznego (R=20, Φ=0°, Θ=45°): {:?}",
        punkt_ze_sferycznego
    );

    println!("\n----------------------------------------------------\n");

    // =================================================================
    // 3. DMS
    // =================================================================

    // Tworzymy wektor z radianami Szerokości i Długości geograficznej
    let stawowa2 = cs![dms!(50, 14, 56.3), dms!(19, 8, 2.3)];

    // Promień Ziemi w metrach (uproszczenie sferyczne)
    let r_ziemi = 6_371_000.0;

    // Konwertujemy do twardego układu kartezjańskiego (ECEF XYZ)
    let stawowa_xyz = stawowa2.to_ecef_from_rad_sn_we(r_ziemi);

    println!("🌐: {:?}", stawowa2);
    stawowa_xyz.print("stawowa2", AngleFmt::Deg);
    stawowa_xyz.print_dms_sn_we("stawowa_xyz");
    println!("🌐 Radiany (Szer, Dług): {:?}", stawowa2);
    println!("📍 Ulica Stawowa względem Jądra Ziemi (metry):");
    println!("   X: {:.2} m", stawowa_xyz[0]);
    println!("   Y: {:.2} m", stawowa_xyz[1]);
    println!("   Z: {:.2} m", stawowa_xyz[2]);
    println!(
        "📏 Odległość od Jądra (kontrola): {:.2} m",
        stawowa_xyz.rxyz()
    );

    // DEKOMPRESJA - powrót z XYZ do DMS
    let z_powrotem_na_powierzchnie = stawowa_xyz.to_dms_sn_we_from_xyz();
    println!("🔄 Z powrotem do DMS: {:?}", z_powrotem_na_powierzchnie);
}
