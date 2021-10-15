pub struct Snake {
    health: u8,
    body_parts: Vec<(i32, i32)>,
}

impl Snake {
    pub fn new(max_x: i32, max_y: i32) -> Snake {
	// TODO:: Only allow even numbers for coordinates
        Snake {
            health: 3,
            body_parts: vec!((max_x / 2, max_y / 2)),
        }
    }

    pub fn add_body_part(&mut self) {
        // TODO: perform a coordinate search to make sure body
        // part addition does not collide with other body parts
        let (x, y) = self.body_parts[self.body_parts.len() - 1];
        self.body_parts.push((x + 1, y));
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
    fn it_creates_a_new_snake_with_body_parts() {
        let snake = Snake::new(2, 2);

        assert_eq!(snake.body_parts.len(), 1);
        assert_eq!(snake.body_parts[0], (1, 1))
    }

    // TODO: add test cases to test for collisions
    fn it_adds_a_body_part_to_the_snake() {
        let mut snake = Snake::new(10, 10);

        snake.add_body_part();
        assert_eq!(snake.body_parts[snake.body_parts.len() - 1], (6, 5));
    }
}
