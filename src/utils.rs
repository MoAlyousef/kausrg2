use axum::http::Uri;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Language {
    English,
    Arabic,
}

pub fn nationality(i: i64, lang: Language) -> String {
    if lang == Language::English {
        match i {
            1 => "Saudi".to_string(),
            2 => "Egyptian".to_string(),
            _ => "Saudi".to_string(),
        }
    } else {
        match i {
            1 => "سعودي".to_string(),
            2 => "مصري".to_string(),
            _ => "سعودي".to_string(),
        }
    }
}

pub fn rank(i: i64, lang: Language) -> String {
    if lang == Language::English {
        match i {
            1 => "Professor".to_string(),
            2 => "Associate Professor".to_string(),
            3 => "Assistant Professor".to_string(),
            _ => "Assistant Professor".to_string(),
        }
    } else {
        match i {
            1 => "أستاذ".to_string(),
            2 => "أستاذ مشارك".to_string(),
            3 => "أستاذ مساعد".to_string(),
            _ => "أستاذ مساعد".to_string(),
        }
    }
}

pub fn division(i: i64, lang: Language) -> String {
    if lang == Language::English {
        match i {
            1 => "General Surgery".to_string(),
            2 => "Neurosurgery".to_string(),
            3 => "Cardiac Surgery".to_string(),
            4 => "Thoracic Surgery".to_string(),
            5 => "Vascular Surgery".to_string(),
            6 => "Plastic Surgery".to_string(),
            7 => "Pediatric Surgery".to_string(),
            _ => "General Surgery".to_string(),
        }
    } else {
        match i {
            1 => "الجراحة العامة".to_string(),
            2 => "جراحة المخ والأعصاب".to_string(),
            3 => "جراحة القلب".to_string(),
            4 => "جراحة الصدر".to_string(),
            5 => "جراحة الأوعية الدموية".to_string(),
            6 => "جراحة التجميل".to_string(),
            7 => "جراحة الأطفال".to_string(),
            _ => "الجراحة العامة".to_string(),
        }
    }
}

pub fn language(uri: &Uri) -> Language {
    if uri.path().starts_with("/ar") {
        Language::Arabic
    } else {
        Language::English
    }
}
