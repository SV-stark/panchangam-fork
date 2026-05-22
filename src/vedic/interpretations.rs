//! Vedic Interpretations Engine (Career, Progeny, and Nadi Astrology)

use crate::vedic::dignity::{calculate_dignity, Dignity};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use alloc::vec::Vec;
use alloc::string::{String, ToString};

/// Nadi Astrology Division Information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct NadiInterpretation {
    pub coordinate_idx: u16, // 1 to 1800
    pub nadi_name: String,
    pub element: String,
    pub ruling_planet: String,
    pub spiritual_nature: String,
    pub early_life_focus: String,
    pub mid_life_focus: String,
    pub late_life_focus: String,
}

/// Progeny Prediction results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct ProgenyReport {
    pub fertility_score: f64, // 0 to 100
    pub timing_window_years: String,
    pub child_health_indicator: String,
    pub progeny_yogas: Vec<String>,
    pub guidance: String,
}

/// Career Guidance results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct CareerReport {
    pub career_strength: f64, // 0 to 100
    pub dominant_element: String,
    pub recommended_professions: Vec<String>,
    pub leadership_style: String,
    pub timing_of_rise: String,
}

const NADI_NAMES: [&str; 150] = [
    "Vasudha", "Vaishnavi", "Brahmi", "Kalabhra", "Kaumari", "Varahi", "Harini", "Kaumudi",
    "Sharada", "Malini", "Angada", "Shambhavi", "Shivada", "Shakthi", "Kumudini", "Kalahamnsa",
    "Kamada", "Chandi", "Varada", "Jhansi", "Gauri", "Dharma", "Daya", "Shanti", "Priya",
    "Nirmala", "Vimala", "Mangala", "Amrita", "Kalyani", "Narayani", "Kamakshi", "Ganga", "Yamuna",
    "Saraswati", "Indrani", "Rudrani", "Gayatri", "Savitri", "Durga", "Bhavani", "Lakshmi", "Radha",
    "Sita", "Rukmini", "Satyabhama", "Aditi", "Diti", "Danu", "Kala", "Vidya", "Maya",
    "Kripa", "Bhakti", "Shraddha", "Medha", "Pragna", "Buddhi", "Mukti", "Siddhi", "Riddhi",
    "Kirti", "Kanti", "Dheera", "Veera", "Shanti", "Tripti", "Tushti", "Pushti", "Arogya",
    "Aishwarya", "Bhumi", "Pavani", "Gomati", "Vela", "Prabha", "Jyoti", "Lalita", "Tripura",
    "Matangi", "Bagala", "Chinnamasta", "Bhairavi", "Dhumavati", "Sodasi", "Kali", "Tara", "Ugra",
    "Ghora", "Rudra", "Shiva", "Hara", "Mrityunjaya", "Kapali", "Bhairava", "Virabhadra", "Nandi",
    "Ganesha", "Kartikeya", "Kubera", "Yama", "Varuna", "Indra", "Agni", "Vayu", "Nirriti",
    "Soma", "Ishana", "Surya", "Chandra", "Mangala", "Budha", "Guru", "Shukra", "Shani",
    "Rahu", "Ketu", "Dattatreya", "Vyasa", "Valmiki", "Vashishta", "Vishwamitra", "Agastya", "Bhrigu",
    "Parashara", "Narada", "Kapila", "Kanada", "Gautama", "Patanjali", "Jaimini", "Adi", "Madhya",
    "Antya", "Satya", "Treta", "Dvapara", "Kali", "Krita", "Dharma", "Artha", "Kama",
    "Moksha", "Samsara", "Karma", "Gyana", "Yoga", "Tantra", "Mantra", "Yantra"
];

/// Map planetary coordinates to the 1800 named Nadi Amsas
pub fn calculate_nadi_astrology(
    longitude: f64,
) -> NadiInterpretation {
    let sign_idx = ((longitude % 360.0) / 30.0).floor() as usize % 12;
    let deg_in_sign = longitude % 30.0;
    
    // 150 divisions per sign, each spans 12 minutes (0.2 degrees)
    let div_idx = (deg_in_sign / 0.2).floor() as usize;
    let is_odd_sign = sign_idx % 2 == 0;
    
    let nadi_idx = if is_odd_sign {
        div_idx.min(149)
    } else {
        (149 - div_idx).max(0).min(149)
    };

    let nadi_name = NADI_NAMES[nadi_idx].to_string();
    let coordinate_idx = (sign_idx * 150 + nadi_idx + 1) as u16;

    let elements = ["Fire", "Earth", "Air", "Water"];
    let element = elements[sign_idx % 4].to_string();

    let planets = ["Sun", "Moon", "Mars", "Mercury", "Jupiter", "Venus", "Saturn"];
    let ruling_planet = planets[nadi_idx % 7].to_string();

    // Spirited descriptions mapped to Nadi indexes
    let spiritual_natures = [
        "Highly intuitive, connected to ancestral intelligence, seeking spiritual self-realization.",
        "Grounded, practical, possesses immense mental resilience and a healing presence.",
        "Dynamic and fiery, characterized by leadership, swift decisions, and protective instincts.",
        "Intellectual, communicative, loves exploring dualities and deep mystical knowledge.",
        "Philosophical and expansive, focused on higher wisdom, teaching, and spiritual growth.",
        "Harmonious, creative, seeks beauty and divine connection through aesthetic expressions.",
        "Structured, highly disciplined, a silent observer with deep spiritual endurance."
    ];
    let spiritual_nature = spiritual_natures[nadi_idx % 7].to_string();

    let early_life_focus = alloc::format!(
        "Acquisition of baseline knowledge and early character building under the influence of the {} element. Fostering keen mental curiosity.",
        element
    );
    let mid_life_focus = alloc::format!(
        "Expansion of worldly duties, establishing stability, and manifesting creative/professional goals under the rulership of planet {}.",
        ruling_planet
    );
    let late_life_focus = "Introspection, mentoring others, transcending material desires, and aligning completely with ancestral and universal wisdom.".to_string();

    NadiInterpretation {
        coordinate_idx,
        nadi_name,
        element,
        ruling_planet,
        spiritual_nature,
        early_life_focus,
        mid_life_focus,
        late_life_focus,
    }
}

/// Assess child prediction aspects using 5th house and D7 indicators
pub fn predict_progeny(
    jupiter_long: f64,
    fifth_house_long: f64,
    fifth_lord_shadbala: f64,
    d7_jupiter_sign: u8,
) -> ProgenyReport {
    let mut fertility_score = 60.0;
    let mut progeny_yogas = Vec::new();

    // 1. Evaluate Jupiter (Putrakaraka / General Child Significator)
    let jup_dignity = calculate_dignity("Jupiter", jupiter_long);
    match jup_dignity {
        Dignity::Exalted => {
            fertility_score += 20.0;
            progeny_yogas.push("Deva-Putra Yoga: Strong planetary significator (Jupiter Exalted) indicating highly intelligent progeny.".to_string());
        },
        Dignity::OwnSign => {
            fertility_score += 15.0;
            progeny_yogas.push("Bhadra-Praja Yoga: Jupiter in own sign, ensuring dutiful and healthy children.".to_string());
        },
        Dignity::Debilitated => {
            fertility_score -= 20.0;
            progeny_yogas.push("Ksheena-Putrakaraka: Jupiter is debilitated, requiring spiritual remedies (Upayas) to remove progeny blocks.".to_string());
        },
        _ => {}
    }

    // 2. Evaluate 5th Lord Strength
    if fifth_lord_shadbala >= 400.0 {
        fertility_score += 15.0;
        progeny_yogas.push("Bala-Adhipati Yoga: The 5th lord is extremely strong in Shadbala, indicating smooth conception.".to_string());
    } else if fifth_lord_shadbala <= 250.0 {
        fertility_score -= 15.0;
    }

    // 3. Evaluate D7 (Saptamsha) Jupiter sign
    if [9, 12].contains(&d7_jupiter_sign) {
        fertility_score += 10.0;
        progeny_yogas.push("D7 Saptamsha Sukha: Fortified divisional planetary strength indicating auspicious timing.".to_string());
    }

    if fertility_score > 100.0 {
        fertility_score = 100.0;
    } else if fertility_score < 0.0 {
        fertility_score = 0.0;
    }

    let timing_window_years = if fertility_score >= 75.0 {
        "Highly favorable timing within the next 1 to 2 years, especially during Jupiter transits over 5th/9th houses."
    } else if fertility_score >= 50.0 {
        "Moderately favorable timing within the next 2 to 3 years. Favorable transits required."
    } else {
        "Auspicious windows are delayed; timing is ideal in 4 to 5 years after performing traditional remedies."
    }.to_string();

    let child_health_indicator = if fertility_score >= 70.0 {
        "Robust health and high vitality predicted for children."
    } else {
        "Moderate health; child health needs support via diet and planetary adjustments."
    }.to_string();

    let guidance = if fertility_score >= 70.0 {
        "Planets are highly supportive. Proceed with optimism."
    } else {
        "Recommended remedies: Recitation of Santana Gopala Mantra, and charities on Thursdays to strengthen Jupiter."
    }.to_string();

    ProgenyReport {
        fertility_score,
        timing_window_years,
        child_health_indicator,
        progeny_yogas,
        guidance,
    }
}

/// Professional and career guidance engine using 10th house and D10 indicators
pub fn analyze_career_guidance(
    amatyakaraka_long: f64,
    tenth_house_long: f64,
    tenth_lord_shadbala: f64,
    amatyakaraka_planet_idx: usize, // 0: Sun ... 6: Saturn
) -> CareerReport {
    let planet_names = ["Sun", "Moon", "Mercury", "Venus", "Mars", "Jupiter", "Saturn"];
    let amk_name = if amatyakaraka_planet_idx < 7 {
        planet_names[amatyakaraka_planet_idx]
    } else {
        "Jupiter"
    };

    let ten_sign_idx = ((tenth_house_long % 360.0) / 30.0).floor() as usize % 12;
    let elements = ["Fire", "Earth", "Air", "Water"];
    let dominant_element = elements[ten_sign_idx % 4].to_string();

    // 1. Base Career Strength
    let mut career_strength = 50.0;
    let amk_dignity = calculate_dignity(amk_name, amatyakaraka_long);
    match amk_dignity {
        Dignity::Exalted => career_strength += 25.0,
        Dignity::OwnSign | Dignity::Moolatrikona => career_strength += 20.0,
        Dignity::GreatFriend | Dignity::Friend => career_strength += 10.0,
        Dignity::Debilitated => career_strength -= 15.0,
        _ => {}
    }

    if tenth_lord_shadbala >= 400.0 {
        career_strength += 15.0;
    } else if tenth_lord_shadbala <= 250.0 {
        career_strength -= 10.0;
    }

    if career_strength > 100.0 {
        career_strength = 100.0;
    } else if career_strength < 0.0 {
        career_strength = 0.0;
    }

    // 2. Recommended Professions based on Amatyakaraka
    let mut recommended_professions = Vec::new();
    match amk_name {
        "Sun" => {
            recommended_professions.push("Government administration & Civil services".to_string());
            recommended_professions.push("Political leadership or strategic policy making".to_string());
            recommended_professions.push("Medical management or cardiology".to_string());
        },
        "Moon" => {
            recommended_professions.push("Public relations & Hospitality management".to_string());
            recommended_professions.push("Food industry, dairy, or maritime trade".to_string());
            recommended_professions.push("Psychology, counseling, or creative arts".to_string());
        },
        "Mars" => {
            recommended_professions.push("Mechanical, civil, or systems engineering".to_string());
            recommended_professions.push("Military, law enforcement, or security services".to_string());
            recommended_professions.push("Surgical medicine or competitive sports coaching".to_string());
        },
        "Mercury" => {
            recommended_professions.push("Data analysis, software development, or IT".to_string());
            recommended_professions.push("Financial accounting or journalism/writing".to_string());
            recommended_professions.push("Marketing, public speaking, or teaching".to_string());
        },
        "Jupiter" => {
            recommended_professions.push("Educational leadership, professorship, or research".to_string());
            recommended_professions.push("Judiciary, corporate law, or legal advisory".to_string());
            recommended_professions.push("Investment banking or spiritual counseling".to_string());
        },
        "Venus" => {
            recommended_professions.push("Creative design, architecture, or fine arts".to_string());
            recommended_professions.push("Fashion, luxury goods branding, or hospitality".to_string());
            recommended_professions.push("Media production, acting, or cosmetics entrepreneurship".to_string());
        },
        _ => {
            // Saturn
            recommended_professions.push("Real estate development, construction, or mining".to_string());
            recommended_professions.push("Supply chain logistics or agricultural technology".to_string());
            recommended_professions.push("Industrial manufacturing or labor relations".to_string());
        }
    }

    let leadership_style = match amk_name {
        "Sun" | "Mars" => "Authoritative, action-oriented, decisive, and leading from the front.",
        "Jupiter" | "Mercury" => "Consultative, mentorship-driven, highly analytical, and strategic.",
        "Venus" | "Moon" => "Collaborative, highly empathetic, creative, and harmony-focused.",
        _ => "Highly disciplined, hands-on, detail-oriented, and process-driven."
    }.to_string();

    let timing_of_rise = if career_strength >= 75.0 {
        "Early professional rise, with major breakthroughs in the mid-20s during Amatyakaraka Dasha."
    } else if career_strength >= 50.0 {
        "Steady and progressive career growth, peaking in the early 30s during favorable Jupiter transits."
    } else {
        "Delayed but solid career stability, peaking after age 36. Patience and diligence required."
    }.to_string();

    CareerReport {
        career_strength,
        dominant_element,
        recommended_professions,
        leadership_style,
        timing_of_rise,
    }
}
