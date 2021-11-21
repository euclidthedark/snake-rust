use std::collections::HashSet;

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

    pub fn add_body_part(&mut self) -> Result<(i32, i32), &str> {
        let coordinate_set: HashSet<(i32, i32)> = self.body.iter().cloned().collect();

        match self.orientation {
            Orientation::Left => if let Some((last_x, last_y)) = self.body.clone().last() {
                self.body.push((*last_x + 1, *last_y));
                Ok((*last_x + 1, *last_y))
            } else {
                Err("Coordinate can't be appened to body.")
            },
            _ => Err("snake.oriented must be set, and it isn't set."),
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

    #[test]
    fn it_adds_a_body_part_when_moving_left_without_collisions() {
        let mut snake = Snake::new(20, 20);

        // snake orientation defaults to the left
        // when there is no body part in the next slot to the right
        assert_eq!(Ok((11, 10)), snake.add_body_part());
        assert_eq!(Some(&(11, 10)), snake.body.last());

        // oriente the snake right 
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
