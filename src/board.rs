pub struct Board {
    origin: (i32, i32),
    food: Option<(i32, i32)>,    
    predator: Option<(i32, i32)>,
    max_x: i32,
    max_y: i32,
}

impl Board {
    pub fn new(max_x: i32, max_y: i32) -> Board {
        Board {
            max_x,
            max_y,
            origin: (max_x / 2, max_y / 2),
            food: None,
            predator: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_board_with_the_correct_max_dimensions() {
        let board = Board::new(10, 10);

        assert_eq!(board.max_x, 10);
        assert_eq!(board.max_y, 10);
    }

    #[test]
    fn it_creates_a_new_board_with_the_correct_origin() {
        let board = Board::new(10, 10);

        assert_eq!(board.origin, (5, 5));
    }
}