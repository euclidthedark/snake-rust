use std::collections::{HashSet};

#[derive(PartialEq, Debug)]
pub enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    body: Vec<(i32, i32)>,
    orientation: Orientation,
}

impl Snake {
    pub fn new((x, y): (i32, i32)) -> Snake {
        Snake {
            body: vec!((x, y)),
            orientation: Orientation::Left,
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

    pub fn add_body_part(&mut self) -> Result<(i32 ,i32), &str> {
        let collision_message = "Collision when trying to append body part after the tail.";
        let coordiantes: HashSet<(i32, i32)> = self.body
            .clone()
            .into_iter()
            .collect();

        if let Some((last_x, last_y)) = self.body.clone().last() {
            match self.orientation {
                Orientation::Left => {
                    if !coordiantes.contains(&(*last_x - 1, *last_y)) {
                        self.body.push((*last_x - 1, *last_y));
                        Ok((last_x - 1, *last_y))
                    } else { Err(collision_message) }
                },
                Orientation::Right => {
                    if !coordiantes.contains(&(*last_x + 1, *last_y)) {
                        self.body.push((*last_x + 1, *last_y));
                        Ok((last_x + 1, *last_y))
                    } else { Err(collision_message) }
                },
                Orientation::Up => {
                    if !coordiantes.contains(&(*last_x, *last_y + 1)) {
                        self.body.push((*last_x, *last_y + 1));
                        Ok((*last_x, *last_y + 1))
                    } else { Err(collision_message) }
                },
                Orientation::Down => {
                    if !coordiantes.contains(&(*last_x, *last_y - 1)) {
                        self.body.push((*last_x, *last_y - 1));
                        Ok((*last_x, *last_y - 1))
                    } else { Err(collision_message) }
                },
            }
        } else { Err("No body parts exist on snake.") }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_new_snake_with_one_body_part() {
        let snake = Snake::new((1, 1));

        assert_eq!(snake.body.len(), 1);
    }

    #[test]
    fn it_creates_a_new_snake_that_is_moving_left() {
        let snake = Snake::new((1, 1));

        assert_eq!(snake.orientation, Orientation::Left);
    }

    // TODO: make the collision cases magic
    #[test]
    fn it_adds_a_body_part_when_moving_without_collisions() {
        let collision_message = "Collision when trying to append body part after the tail.";
        let mut snake = Snake::new((10, 10));

        // add a body part while moving left
        assert_eq!(Ok((9, 10)), snake.add_body_part());
        assert_eq!(Some(&(9, 10)), snake.body.last());
        // if the coordinate to the body already exists
        snake.body.push((10, 10));
        assert_eq!(Err(collision_message), snake.add_body_part());
        // reset snake
        snake.body = vec![(10, 10)];
        snake.orientation = Orientation::Right;
        // add a body part while moving right
        assert_eq!(Ok((11, 10)), snake.add_body_part());
        assert_eq!(Some(&(11, 10)), snake.body.last());
        // if the coordinate to the body already exists
        snake.body.push((10, 10));
        assert_eq!(Err(collision_message), snake.add_body_part());
        // reset snake
        snake.body = vec![(10, 10)];
        snake.orientation = Orientation::Up;
        // add a body part while moving up
        assert_eq!(Ok((10, 11)), snake.add_body_part());
        assert_eq!(Some(&(10, 11)), snake.body.last());
        // if the coordinate to the body already exists
        snake.body.push((10, 10));
        assert_eq!(Err(collision_message), snake.add_body_part());
        // reset snake
        snake.body = vec![(10, 10)];
        snake.orientation = Orientation::Down;
        // add a body part while moving up
        assert_eq!(Ok((10, 9)), snake.add_body_part());
        assert_eq!(Some(&(10, 9)), snake.body.last());
        // if the coordinate to the body already exists
        snake.body.push((10, 10));
        assert_eq!(Err(collision_message), snake.add_body_part());

    }
}
