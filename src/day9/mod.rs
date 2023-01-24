struct Position {
    x: u16,
    y: u16,
}

enum MoveDecision {
    Up,
    Down,
    Left,
    Right,
    RightUp,
    RightDown,
    LeftUp,
    LeftDown,
    NoMove,
    Error,
}

impl MoveDecision {
    fn move_offset(&self) -> (i16, i16) {
        match self {
            Self::Up => (0, 1),
            Self::Down => (0, -1),
            Self::Right => (1, 0),
            Self::Left => (-1, 0),
            Self::LeftUp => (-1, 1),
            Self::RightUp => (1, 1),
            Self::RightDown => (1, -1),
            Self::LeftDown => (-1, -1),
            Self::NoMove => (0, 0),
            Self::Error => (0, 0),
        }
    }
}

impl Position {
    fn relative_distance(&self, other_position: &Position) -> (i16, i16) {
        (
            i16::try_from(self.x).unwrap() - i16::try_from(other_position.x).unwrap(),
            i16::try_from(self.y).unwrap() - i16::try_from(other_position.y).unwrap(),
        )
    }

    fn make_decision(&self, other_position: &Position) -> MoveDecision {
        let (x_relative_distance, y_relative_distance) = self.relative_distance(other_position);
        let x_distance = i16::abs(x_relative_distance);
        let y_distance = i16::abs(y_relative_distance);
        if x_distance == 2 && y_distance == 0 {
            if x_relative_distance > 0 {
                return MoveDecision::Right;
            } else {
                return MoveDecision::Left;
            }
        }

        if y_distance == 2 && x_distance == 0 {
            if y_relative_distance > 0 {
                return MoveDecision::Up;
            } else {
                return MoveDecision::Down;
            }
        }

        if x_distance == 2 && y_distance == 1 {
            if x_relative_distance > 0 {
                if y_relative_distance > 0 {
                    return MoveDecision::RightUp;
                } else {
                    return MoveDecision::RightDown;
                }
            } else if y_relative_distance > 0 {
                return MoveDecision::LeftUp;
            } else {
                return MoveDecision::LeftDown;
            }
        }

        if y_distance == 2 && x_distance == 1 {
            if y_relative_distance > 0 {
                if x_relative_distance > 0 {
                    return MoveDecision::RightUp;
                } else {
                    return MoveDecision::LeftUp;
                }
            } else if x_relative_distance > 0 {
                return MoveDecision::RightDown;
            } else {
                return MoveDecision::LeftDown;
            }
        }

        if x_distance < 1 && y_distance < 1 {
            return MoveDecision::NoMove;
        }

        MoveDecision::Error
    }
}

struct Rope {
    head: Position,
    tail: Position,
    initial_position: Position,
}

impl Rope {
    fn new() -> Self {
        Self {
            head: Position { x: 0, y: 0 },
            tail: Position { x: 0, y: 0 },
            initial_position: Position { x: 0, y: 0 },
        }
    }
}
