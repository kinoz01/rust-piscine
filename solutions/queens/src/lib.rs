#[derive(Debug)]
pub struct ChessPosition {
    pub rank: i32,
    pub file: i32,
}

#[derive(Debug)]
pub struct Queen {
    pub position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if (0..=7).contains(&rank) && (0..=7).contains(&file) {
            Some(Self{rank, file})
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self{position}
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let dr = (self.position.rank - other.position.rank).abs();
        let df = (self.position.file - other.position.file).abs();

        if dr == 0 || df == 0 || df ==dr {
            return true
        } else {
            return false
        }
    }
}