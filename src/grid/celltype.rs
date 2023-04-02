pub enum CellType {
    Ground,
    Fruit,
    Snake
}

pub fn to_string(t: &CellType) -> String{
    return match t {
        CellType::Ground => "-".to_string(),
        CellType::Fruit => "@".to_string(),
        CellType::Snake => "*".to_string()
    }
}