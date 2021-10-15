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
}
