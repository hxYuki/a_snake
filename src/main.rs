mod constants;

use bevy::{prelude::*, time::FixedTimestep};
use constants::{ARENA_HEIGHT, ARENA_WIDTH, FOOD_COLOR, SNAKE_HEAD_COLOR, SNAKE_SEGMENT_COLOR};
use rand::prelude::random;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Snake!".to_string(),
                width: 500.0,
                height: 500.0,
                ..default()
            },
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.250))
                .with_system(snake_movement)
                .with_system(snake_eating.after(snake_movement))
                .with_system(snake_growth.after(snake_eating))
                .with_system(game_over.after(snake_movement)),
        )
        .add_system(snake_movement_input.before(snake_movement))
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.))
                .with_system(food_spawner),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
    body: Vec<Entity>,
    last_tail: Option<Position>,
}

fn spawn_snake(mut commands: Commands) {
    let b = spawn_segement(&mut commands, Position { x: 3, y: 2 });
    let mut head = commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10., 10., 10.),
                ..default()
            },
            ..default()
        },
        SnakeSegment,
        Position { x: 3, y: 3 },
        Size::square(0.8),
    ));

    let c = SnakeHead {
        direction: Direction::Right,
        body: vec![head.id(), b],
        last_tail: None,
    };

    head.insert(c);
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}
impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
        }
    }
}
fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    for mut head in heads.iter_mut() {
        let dir = if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
            Direction::Left
        } else if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
            Direction::Right
        } else if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
            Direction::Down
        } else if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
            Direction::Up
        } else {
            head.direction
        };

        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}
fn snake_movement(
    mut over_writer: EventWriter<GameOverEvent>,
    mut heads: Query<(&mut SnakeHead, Entity)>,
    mut positions: Query<&mut Position>,
) {
    for (mut head, et) in heads.iter_mut() {
        let headbody = &head.body;
        let segspos = headbody
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();

        let mut head_pos = positions.get_mut(et).unwrap();
        match &head.direction {
            Direction::Left => head_pos.x -= 1,
            Direction::Up => head_pos.y += 1,
            Direction::Right => head_pos.x += 1,
            Direction::Down => head_pos.y -= 1,
        }

        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x as u32 >= ARENA_WIDTH
            || head_pos.y as u32 >= ARENA_HEIGHT
        {
            info!(target:"game over event","out bound, {:?}", head_pos);

            over_writer.send(GameOverEvent);
        }
        if segspos[1..].contains(&head_pos) {
            info!(target:"game over event","tail eaten");

            over_writer.send(GameOverEvent);
        }

        segspos
            .iter()
            .zip(headbody.iter().skip(1))
            .for_each(|(p, seg)| {
                *positions.get_mut(*seg).unwrap() = *p;
            });

        head.last_tail = Some(*segspos.last().unwrap());
    }
}

#[derive(Component)]
struct SnakeSegment;

fn spawn_segement(commands: &mut Commands, position: Position) -> Entity {
    (*commands)
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_SEGMENT_COLOR,
                    ..default()
                },
                ..default()
            },
            SnakeSegment,
            position,
            Size::square(0.65),
        ))
        .id()
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();

    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.,
        )
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos * tile_size - (bound_window / 2.) + tile_size / 2.
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width(), ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height(), ARENA_HEIGHT as f32),
            0.,
        )
    }
}

#[derive(Component)]
struct Food;

fn food_spawner(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        },
        Food,
        Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        },
        Size::square(0.8),
    ));
}

struct GrowthEvent;

fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

fn snake_growth(
    mut commands: Commands,
    mut heads: Query<&mut SnakeHead>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    for mut head in heads.iter_mut() {
        let pos = head.last_tail.unwrap();
        if growth_reader.iter().next().is_some() {
            head.body.push(spawn_segement(&mut commands, pos))
        }
    }
}

struct GameOverEvent;

fn game_over(
    mut commands: Commands,
    over_reader: EventReader<GameOverEvent>,
    segments: Query<Entity, With<SnakeSegment>>,
    food: Query<Entity, With<Food>>,
) {
    // info!("{:?}", over_reader.len());
    if !over_reader.is_empty() {
        over_reader.clear();
        for ent in segments.iter().chain(food.iter()) {
            commands.entity(ent).despawn();
        }

        spawn_snake(commands);
    }
}
