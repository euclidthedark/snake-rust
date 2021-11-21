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
        if self.body.len() > 1
        && self.orientation == Orientation::Left
        && direction == Orientation::Right {
            ()
        } else if self.body.len() > 1
        && self.orientation == Orientation::Right
        && direction == Orientation::Left {
            ()
        } else { self.orientation = direction }
    }

    pub fn add_body_part(&mut self) -> Result<(i32, i32), &str> {
        let coordinate_cant_be_added_error = "Coordinate can't be appened to body.";

        match self.orientation {
            Orientation::Left => if let Some((last_x, last_y)) = self.body.clone().last() {
                self.body.push((*last_x + 1, *last_y));
                Ok((*last_x + 1, *last_y))
            } else { Err(coordinate_cant_be_added_error) },
            Orientation::Right => if let Some((last_x, last_y)) = self.body.clone().last() {
                self.body.push((*last_x - 1, *last_y));
                Ok((*last_x - 1, *last_y))
            } else { Err(coordinate_cant_be_added_error) },
            Orientation::Up => if let Some((last_x, last_y)) = self.body.clone().last() {
                self.body.push((*last_x, *last_y + 1));
                Ok((*last_x, *last_y + 1))
            } else { Err(coordinate_cant_be_added_error) },
            Orientation::Down => if let Some((last_x, last_y)) = self.body.clone().last() {
                self.body.push((*last_x, *last_y - 1));
                Ok((*last_x, *last_y - 1))
            } else { Err(coordinate_cant_be_added_error) },
        }
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
    fn it_will_not_let_you_reoriente_from_left_to_right_when_length_is_greater_than_1() {
        let mut snake = Snake::new(100, 100);
        assert_eq!(Ok((51, 50)), snake.add_body_part());
        snake.redirect_orientation(Orientation::Right);
        assert_eq!(snake.orientation, Orientation::Left);
    }

    #[test]
    fn it_adds_a_body_part_when_moving_without_collisions() {
        let mut snake = Snake::new(20, 20);

        // snake orientation defaults to the left
        // when there isn't a collision
        assert_eq!(Ok((11, 10)), snake.add_body_part());
        assert_eq!(Some(&(11, 10)), snake.body.last());

        // oriente the snake right 
        // and when there isn't a collision
        snake.body = vec![(10, 10)];
        snake.redirect_orientation(Orientation::Right);
        assert_eq!(Ok((9, 10)), snake.add_body_part());
        assert_eq!(Some(&(9, 10)), snake.body.last());

        // oriente the snake up
        // and when there isn't a collision
        snake.body = vec![(10, 10)];
        snake.redirect_orientation(Orientation::Up);
        assert_eq!(Ok((10, 11)), snake.add_body_part());
        assert_eq!(Some(&(10, 11)), snake.body.last());

        // oriente the snake down
        // and when there isn't a collision
        snake.body = vec![(10, 10)];
        snake.redirect_orientation(Orientation::Down);
        assert_eq!(Ok((10, 9)), snake.add_body_part());
        assert_eq!(Some(&(10, 9)), snake.body.last());

        // set up collision cases
    }
}
