use bevy::prelude::*;

use crate::{BodyOf, Position, Size, SnakeSegment};

pub fn new_snake_segment(head: Entity, pos: Position) -> impl Bundle {
    (pos, Size::square(0.65), SnakeSegment, BodyOf(head))
}
