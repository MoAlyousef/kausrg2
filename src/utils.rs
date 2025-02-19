pub(crate) fn nationality(i: i64) -> String {
    match i {
        1 => "Saudi".to_string(),
        2 => "Egyptian".to_string(),
        _ => "Saudi".to_string(),
    }
}

pub(crate) fn rank(i: i64) -> String {
    match i {
        1 => "Professor".to_string(),
        2 => "Associate Professor".to_string(),
        3 => "Assistant Professor".to_string(),
        _ => "Assistant Professor".to_string(),
    }
}

pub(crate) fn division(i: i64) -> String {
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
}
