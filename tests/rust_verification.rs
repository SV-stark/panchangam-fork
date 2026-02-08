
use panchangam::vedic::vargas::{self, VargaType, VargaConfig};
use panchangam::vedic::kp;
use panchangam::vedic::transits;
use panchangam::vedic::dasha;
use panchangam::vedic::ashtakavarga;

#[test]
fn test_d249_logic() {
    let config = VargaConfig::new();
    
    // Aries (1)
    // 1.0 deg. Odd sign. Count from Sign.
    // 1.0 / (30/249) = 8.3 -> Index 8 (9th part)
    // 1 + 8 = 9 (Sagittarius)
    
    let res = vargas::calculate_varga_position(1.0, VargaType::D249, &config);
    assert_eq!(res.sign, 9, "D249 Aries 1deg should be Sagittarius");
}

#[test]
fn test_kp_system() {
    // 0 deg Aries.
    // Aswini (Ketu). Sub Lord Ketu.
    let res = kp::calculate_kp_lords(0.0);
    assert_eq!(res.sign_id, 1);
    assert_eq!(res.star_lord, 9); // Ketu
    assert_eq!(res.sub_lord, 9);
    
    // 1 deg Aries.
    // Aswini. Sub Lord Venus.
    let res2 = kp::calculate_kp_lords(1.0);
    assert_eq!(res2.sub_lord, 6); // Venus
}

#[test]
fn test_transits() {
    // Sade Sati Rising: Saturn(12) from Moon(1)
    let ss = transits::check_sade_sati(1, 12);
    assert!(ss.is_active);
    assert_eq!(ss.phase, "Rising");
    
    // Dhaiya Ardhastama: Saturn(4) from Moon(1)
    let dh = transits::check_dhaiya(1, 4);
    assert!(dh.is_active);
    assert_eq!(dh.type_name, "Ardhastama");
}

#[test]
fn test_yogini_pratyanta() {
    // 0 deg -> Ashwini -> Start Dasha Bhramari (Mars) 4y.
    // At t=0, MD=Bhramari, AD=Bhramari, PD=Bhramari.
    let res = dasha::calculate_yogini(0.0, 0.0, 0.0);
    assert_eq!(res.mahadasha, "Bhramari");
    assert_eq!(res.antardasha, "Bhramari");
    assert_eq!(res.pratyantardasha, "Bhramari");
}

#[test]
fn test_ashtakavarga_pinda() {
    // Reusing the example from pinda_verify.test.ts
    // Rasi Pinda = 92, Graha Pinda = 63. Total = 155.
    
    let reduced_bindus = [1, 2, 0, 3, 1, 0, 2, 1, 0, 2, 1, 0];
     // Sun(0) in Aries (10.0) -> Index 0. 
     // Moon(1) in Taurus (40.0) -> Index 1.
     // Mars(2) in Cancer (100.0) -> Index 3.
     // Mer(3) in Virgo (160.0) -> Index 5.
     // Jup(4) in Scorpio (220.0) -> Index 7.
     // Ven(5) in Cap (280.0) -> Index 9.
     // Sat(6) in Pisces (340.0) -> Index 11.
    
    let planets = [
        10.0, 40.0, 100.0, 160.0, 220.0, 280.0, 340.0
    ];
    
    let pinda = ashtakavarga::calculate_pinda(&reduced_bindus, &planets, 0);
    assert_eq!(pinda, 155);
}
