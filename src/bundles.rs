use bevy::prelude::*;

use crate::{constants::SNAKE_SEGMENT_COLOR, Position, Size, SnakeSegment};

#[derive(Bundle)]
pub struct SnakeSegmentBundle {
    sprite: SpriteBundle,
    pos: Position,
    size: Size,
    marker: SnakeSegment,
}

impl SnakeSegmentBundle {
    pub fn new(head: Entity, pos: Position) -> Self {
        SnakeSegmentBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_SEGMENT_COLOR,
                    ..default()
                },
                ..default()
            },
            pos,
            size: Size::square(0.65),
            marker: SnakeSegment(head),
        }
    }
}
