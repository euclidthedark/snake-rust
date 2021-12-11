use std::collections::{HashSet};

#[derive(PartialEq, Debug)]
pub enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

fn derive_coordinate_from_direction((x, y): (&i32, &i32), direction: &Orientation) -> (i32, i32) {
    match direction {
        Orientation::Up => (*x, *y - 1),
        Orientation::Right => (*x - 1, *y),
        Orientation::Down => (*x, *y + 1),
        Orientation::Left => (*x + 1, *y)
    }
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

    pub fn add_body_part(&mut self) -> Result<(i32 ,i32), &str> {
        let collision_message = "Collision when trying to append body part after the tail.";
        let coordinates: HashSet<(i32, i32)> = self.body
            .clone()
            .into_iter()
            .collect();

        if let Some((last_x, last_y)) = self.body.clone().last() {
            let orientations = [Orientation::Up, Orientation::Right, Orientation::Down, Orientation::Left];
            let mut new_coordinate: Result<(i32, i32), &str> = if !coordinates.contains(&derive_coordinate_from_direction((last_x, last_y), &self.orientation)) {
                self.body.push(derive_coordinate_from_direction((last_x, last_y), &self.orientation));
                Ok(derive_coordinate_from_direction((last_x, last_y), &self.orientation))
            } else { Err(collision_message) };

            if new_coordinate == Err(collision_message) {
                for direction in  orientations.iter().filter(|direction| **direction != self.orientation) {
                    if !coordinates.contains(&derive_coordinate_from_direction((last_x, last_y), &direction)) {
                        self.body.push(derive_coordinate_from_direction((last_x, last_y), &self.orientation));
                        new_coordinate = Ok(derive_coordinate_from_direction((last_x, last_y), &direction));
                        break;
                    }
                }
            }

            new_coordinate
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

        // when going left
        assert_eq!(Ok((11, 10)), snake.add_body_part());
        assert_eq!(Some(&(11, 10)), snake.body.last());
        // when going right
        snake.body = vec![(10, 10)];
        snake.orientation = Orientation::Right;
        assert_eq!(Ok((9, 10)), snake.add_body_part());
        assert_eq!(Some(&(9, 10)), snake.body.last());
        // when going down
        snake.body = vec![(10, 10)];
        snake.orientation = Orientation::Down;
        assert_eq!(Ok((10, 11)), snake.add_body_part());
        assert_eq!(Some(&(10, 11)), snake.body.last());
        // when going down
        snake.body = vec![(10, 10)];
        snake.orientation = Orientation::Down;
        assert_eq!(Ok((10, 11)), snake.add_body_part());
        assert_eq!(Some(&(10, 11)), snake.body.last());
        // when going up
        snake.body = vec![(10, 10)];
        snake.orientation = Orientation::Up;
        assert_eq!(Ok((10, 9)), snake.add_body_part());
        assert_eq!(Some(&(10, 9)), snake.body.last());

    }
}
