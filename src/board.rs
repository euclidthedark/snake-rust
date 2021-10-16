// TODO: make sure randome coordinates don't collide with others
use rand::{thread_rng, Rng};

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

    pub fn set_predator(&mut self) {
        self.predator = Some(self.generate_board_position());
    }

    pub fn set_food(&mut self) {
        self.food = Some(self.generate_board_position());
    }

    fn generate_board_position(&self) -> (i32, i32) {
        let mut rng = thread_rng();

        (rng.gen_range(0..self.max_x), rng.gen_range(0..self.max_y))
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

    // TODO: make test metamorphic
    #[test]
    fn it_creates_a_predator() {
        let mut board = Board::new(10, 10);
        board.set_predator();

        assert_ne!(board.predator, None);
    }

    // TODO: make test metamorphic
    #[test]
    fn it_creates_food() {
        let mut board = Board::new(10, 10);
        board.set_food();

        assert_ne!(board.food, None);
    }
}