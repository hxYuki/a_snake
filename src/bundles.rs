use bevy::prelude::*;

use crate::{Position, Size, SnakeSegment};

#[derive(Bundle)]
pub struct SnakeSegmentBundle {
    pos: Position,
    size: Size,
    marker: SnakeSegment,
}

impl SnakeSegmentBundle {
    pub fn new(head: Entity, pos: Position) -> Self {
        SnakeSegmentBundle {
            pos,
            size: Size::square(0.65),
            marker: SnakeSegment(head),
        }
    }
}
