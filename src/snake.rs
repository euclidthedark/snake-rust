#[derive(PartialEq, Debug)]
pub enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    health: u8,
    body: Vec<(i32, i32)>,
    orientation: Orientation,
    food_eaten_count: u8,
}

impl Snake {
    pub fn new(max_x: i32, max_y: i32) -> Snake {
	// TODO:: store max coordinates in memory
        Snake {
            health: 3,
            body: vec!((max_x / 2, max_y / 2)),
            orientation: Orientation::Left,
            food_eaten_count: 0,
        }
    }

    pub fn redirect_orientation(&mut self, direction: Orientation) {
        self.orientation = direction;
    }

    /**
     * 1) Create a hash map for (x, y)'s
     * 2) Create a body part based on the snakes motion orientation, derived from
     * the coordinate hash map
     */

    pub fn add_body_part(&mut self) -> Option<(i32, i32)> {
        // TODO: perform a coordinate search to make sure body
        // part addition does not collide with other body parts
        let (x, y) = self.body[self.body.len() - 1];

        if self.orientation == Orientation::Left {
            self.body.push((x + 1, y));
            return Some((x + 1, y));
        } else {
            return None;
        }
    }

    pub fn take_health(&mut self) {
        self.health -= 1;
    }

    pub fn eat_food(&mut self) {
        self.food_eaten_count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_new_snake_with_correct_health() {
        let snake = Snake::new(2, 2);

        assert_eq!(snake.health, 3);
    }

    #[test]
    fn it_creates_a_new_snake_with_one_body_part() {
        let snake = Snake::new(2, 2);

        assert_eq!(snake.body.len(), 1);
    }

    #[test]
    fn it_creates_a_new_snake_with_one_body_part_in_the_center() {
        let snake = Snake::new(2, 2);

        assert_eq!(snake.body[0], (1, 1));
    }

    #[test]
    fn it_creates_a_new_snake_that_is_moving_left() {
        let snake = Snake::new(2, 2);

        assert_eq!(snake.orientation, Orientation::Left);
    }

    #[test]
    fn it_reorientes_the_snake_when_redirect_orientation_is_called_to_the_direction_given() {
        let mut snake = Snake::new(2, 2);

        // reorient right
        snake.redirect_orientation(Orientation::Right);
        assert_eq!(snake.orientation, Orientation::Right);

        // reorient left
        snake.redirect_orientation(Orientation::Left);
        assert_eq!(snake.orientation, Orientation::Left);


        // reorient up
        snake.redirect_orientation(Orientation::Up);
        assert_eq!(snake.orientation, Orientation::Up);

        // reorient down
        snake.redirect_orientation(Orientation::Down);
        assert_eq!(snake.orientation, Orientation::Down);
    }

    // TODO: add test cases to test for collisions
    // Add tests for when None is returned
    #[test]
    fn it_adds_a_body_part_when_the_snake_is_moving_left() {
        let mut snake = Snake::new(100, 100);

        snake.add_body_part();
        assert_eq!(snake.body[snake.body.len() - 1], (51, 50));
    }

    #[test]
    fn it_takes_away_one_health_point() {
        let mut snake = Snake::new(2, 2);
        snake.take_health();

        assert_eq!(snake.health, 2);
    }

    #[test]
    fn it_eats_food() {
        let mut snake = Snake::new(2, 2);
        snake.eat_food();
        snake.eat_food();

        assert_eq!(snake.food_eaten_count, 2);
    }
}
