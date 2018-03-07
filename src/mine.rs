

#[derive(Eq,PartialEq)]
pub enum Mine{
    HexCell(bool),
    Flag(bool),
    Reveal(i32),
    OutOfTable,
}

impl Mine {
    pub fn mine(&self) -> i32{
        match self {
            &Mine::HexCell(m) => if m { 1 } else { 0},
            &Mine::Flag(m) => if m { 1 } else {0},
            &Mine::Reveal(_) => 0,
            &Mine::OutOfTable => 0
        }
    }

    pub fn is_mine(&self) -> bool{
        match self {
            &Mine::HexCell(m) => m,
            &Mine::Flag(m) => m,
            &Mine::Reveal(_) => false,
            &Mine::OutOfTable => false
        }
    }
}
