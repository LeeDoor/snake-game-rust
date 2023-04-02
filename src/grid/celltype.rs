pub enum CellType {
    Ground,
    Fruit,
    Snake
}

pub fn to_string(t: &CellType) -> char{
    return match t {
        CellType::Ground => '-',
        CellType::Fruit => '@',
        CellType::Snake => '*'
    }
}