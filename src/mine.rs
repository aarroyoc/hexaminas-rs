

#[derive(Eq,PartialEq)]
pub enum Mine{
    HexCell(bool),
    Flag(bool),
    Question(bool),
    OutOfTable,
}

impl Mine {
    pub fn mine(&self) -> i32{
        match self {
            &Mine::HexCell(m) => if m { 1 } else { 0},
            &Mine::Flag(m) => if m { 1 } else {0},
            &Mine::Question(m) => if m { 1 } else {0},
            &Mine::OutOfTable => 0
        }
    }

    pub fn is_mine(&self) -> bool{
        match self {
            &Mine::HexCell(m) => m,
            &Mine::Flag(m) => m,
            &Mine::Question(m) => m,
            &Mine::OutOfTable => false
        }
    }
}
