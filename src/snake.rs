use std::collections::{HashSet};

#[derive(PartialEq, Debug)]
pub enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

// tested in the struct method
fn derive_coordinate_from_direction((x, y): (&i32, &i32), direction: &Orientation) -> (i32, i32) {
    match direction {
        Orientation::Up => (*x, *y - 1),
        Orientation::Right => (*x - 1, *y),
        Orientation::Down => (*x, *y + 1),
        Orientation::Left => (*x + 1, *y)
    }
}

fn direction_of_adjacency((previuos_x, previous_y): &(i32, i32), (x, y): &(i32, i32)) -> Orientation {
    if previuos_x < x { Orientation::Left }
    else if previuos_x > x { Orientation::Right }
    else {
        if previous_y > y { Orientation::Up }
        else { Orientation::Down }
    }
}

#[test]
fn it_returns_the_correct_coordinate_adjacency_direction() {
    // it returns left
    assert_eq!(direction_of_adjacency(&(1, 2), &(2, 2)), Orientation::Left);
    assert_eq!(direction_of_adjacency(&(4, 2), &(3, 2)), Orientation::Right);
    assert_eq!(direction_of_adjacency(&(3, 3), &(3, 2)), Orientation::Up);
    assert_eq!(direction_of_adjacency(&(3, 1), &(3, 2)), Orientation::Down);
}

fn move_snake(body: &mut Vec<(i32, i32)>, direction: &Orientation) -> Vec<(i32, i32)> {
    let mut previous_coordinate = (0, 0);
    let mut current_coordinate;

    for i in 0..body.len() {
        current_coordinate = body[i];
        let (x, y) = body[i];

        if i == 0 {
            previous_coordinate = body[i];
            body[i] = match direction {
                Orientation::Up => derive_coordinate_from_direction((&x, &y), &Orientation::Down),
                Orientation::Down => derive_coordinate_from_direction((&x, &y), &Orientation::Up),
                Orientation::Right => derive_coordinate_from_direction((&x, &y), &Orientation::Left),
                Orientation::Left => derive_coordinate_from_direction((&x, &y), &Orientation::Right),
            };
        } else {
            body[i] = previous_coordinate;
            previous_coordinate = current_coordinate;
        }
    }

    body.to_vec()
}

#[test]
fn the_snake_moves() {
   // it moves to the left with no zigzags 
   let mut body = vec![(10, 10), (11, 10), (12, 10)];

   assert_eq!(move_snake(&mut body, &Orientation::Left), [(9, 10), (10, 10), (11, 10)]);

   // it moves to the right with no zigzags 
   let mut body = vec![(10, 10), (9, 10), (8, 10)];

   assert_eq!(move_snake(&mut body, &Orientation::Right), [(11, 10), (10, 10), (9, 10)]);

   // it moves up with no zigzags
   let mut body = vec![(10, 10), (10, 9), (10, 8)];

   assert_eq!(move_snake(&mut body, &Orientation::Up), [(10, 11), (10, 10), (10, 9)]);

   // it moves down with no zigzags
   let mut body = vec![(10, 8), (10, 9), (10, 10)];

   assert_eq!(move_snake(&mut body, &Orientation::Down), [(10, 7), (10, 8), (10, 9)]);

   // it moves left with zigzags
   let mut body = vec![(10, 10), (11, 10), (11, 9), (12, 9)];

   assert_eq!(move_snake(&mut body, &Orientation::Left), [(9, 10), (10, 10), (11, 10), (11, 9)]);

   // it moves right with zigzags
   let mut body = vec![(10, 10), (9, 10), (9, 9), (10, 9)];

   assert_eq!(move_snake(&mut body, &Orientation::Right), [(11, 10), (10, 10), (9, 10), (9, 9)]);

   // it moves up with zigzags
   let mut body = vec![(10, 10), (10, 9), (11, 9), (11, 8)];

   assert_eq!(move_snake(&mut body, &Orientation::Up), [(10, 11), (10, 10), (10, 9), (11, 9)]);

   // it down up with zigzags
   let mut body = vec![(10, 10), (10, 11), (11, 11), (11, 12)];

   assert_eq!(move_snake(&mut body, &Orientation::Down), [(10, 9), (10, 10), (10, 11), (11, 11)]);
}

fn is_reflecting_across_y_axis(snake: &mut Snake, direction_to_go: &Orientation) -> bool {
    (snake.orientation == Orientation::Left && *direction_to_go == Orientation::Right)
        || (snake.orientation == Orientation::Right && *direction_to_go == Orientation::Left)
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

    // TODO: Write reoriente function
    pub fn reoriente_snake(&mut self, reoriente_direction: Orientation) -> Option<()> {
        if self.body.len() == 2 && is_reflecting_across_y_axis(self, &reoriente_direction) {
            None
        } else {
            self.orientation = reoriente_direction;
            Some(())
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

    #[test]
    fn it_reorientes_the_snake() {
        let mut snake = Snake::new((10, 10));

        // reorient right
        assert_eq!(Some(()), snake.reoriente_snake(Orientation::Right));
        assert_eq!(snake.orientation, Orientation::Right);

        // reorient left
        assert_eq!(Some(()), snake.reoriente_snake(Orientation::Left));
        assert_eq!(snake.orientation, Orientation::Left);

        // reorient up
        assert_eq!(Some(()), snake.reoriente_snake(Orientation::Up));
        assert_eq!(snake.orientation, Orientation::Up);

        // reorient down
        assert_eq!(Some(()), snake.reoriente_snake(Orientation::Down));
        assert_eq!(snake.orientation, Orientation::Down);

        // reorient left when the body has two parts
        assert_eq!(Some(()), snake.reoriente_snake(Orientation::Left));
        assert_eq!(Ok((11, 10)), snake.add_body_part());
        assert_eq!(None, snake.reoriente_snake(Orientation::Right));

        // reorient right when the body has two parts
        snake.body = vec![(10, 10)];
        assert_eq!(Some(()), snake.reoriente_snake(Orientation::Right));
        assert_eq!(Ok((9, 10)), snake.add_body_part());
        assert_eq!(None, snake.reoriente_snake(Orientation::Left));
    }

    #[test]
    fn it_adds_a_body_part_when_moving_without_collisions() {
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
