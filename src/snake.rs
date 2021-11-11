#[derive(PartialEq, Debug)]
pub enum Orientiation {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    health: u8,
    body: Vec<(i32, i32)>,
    is_moving: Orientiation,
    food_eaten_count: u8,
}

impl Snake {
    pub fn new(max_x: i32, max_y: i32) -> Snake {
	// TODO:: Only allow even numbers for coordinates
        Snake {
            health: 3,
            body: vec!((max_x / 2, max_y / 2)),
            is_moving: Orientiation::Left,
            food_eaten_count: 0,
        }
    }

    pub fn add_body_part(&mut self) {
        // TODO: perform a coordinate search to make sure body
        // part addition does not collide with other body parts
        let (x, y) = self.body[self.body.len() - 1];
        self.body.push((x + 1, y));
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
        let snake = Snake::new(1, 1);

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

        assert_eq!(snake.is_moving, Orientiation::Left);
    }

    // TODO: add test cases to test for collisions
    #[test]
    fn it_adds_a_body_part_to_the_snake() {
        let mut snake = Snake::new(10, 10);

        snake.add_body_part();
        assert_eq!(snake.body[snake.body.len() - 1], (6, 5));
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
