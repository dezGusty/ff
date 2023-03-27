use bevy::{prelude::*, window::PrimaryWindow};

pub const PLAYER_SIZE: f32 = 64.0;
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const PLAYER_SPEED: f32 = 500.0;
pub const ENEMY_SPEED: f32 = 300.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_framepace::FramepacePlugin)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_enemies)
        .add_system(player_movement)
        .add_system(enemy_movement)
        .add_system(confine_player_movement)
        .add_system(respawn_enemies)
        .run();
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let window = window_query.get_single().unwrap();

    let texture_handle = asset_server.load("ship1.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(45.0, 45.0), 1, 3, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let animation_indices = AnimationIndices { first: 1, last: 3 };
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        animation_indices,
        Player { speed: 500.0 },
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    });
}

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub direction: Vec2,
}

pub enum ShipType {
    Player,
    Enemy1,
    Enemy2,
    Enemy3,
    Enemy4,
    Enemy5,
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(
        &mut Transform,
        With<Player>,
        &mut TextureAtlasSprite,
        &AnimationIndices,
    )>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        transform.2.index = 0;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
            transform.2.index = 2;
        }

        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
            transform.2.index = 1;
        }

        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.0.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * enemy.speed * time.delta_seconds();
    }
}

pub fn respawn_enemies(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &Transform, &Enemy)>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    for enemy_transform in enemy_query.iter_mut() {
        if enemy_transform.1.translation.y < 0.0 {
            // enemy_transform.0.translation.y = 600.0;
            println!("Enemy has reached the bottom of the screen");
            commands.entity(enemy_transform.0).despawn();
        }
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let limit_outside = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + limit_outside;
        let x_max = window.width() - limit_outside;
        let y_min = 0.0 + limit_outside;
        let y_max = window.height() / 2.0 - limit_outside;

        if transform.translation.x < x_min {
            transform.translation.x = x_min;
        } else if transform.translation.x > x_max {
            transform.translation.x = x_max;
        }

        if transform.translation.y < y_min {
            transform.translation.y = y_min;
        } else if transform.translation.y > y_max {
            transform.translation.y = y_max;
        }
    }
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = rand::random::<f32>() * window.width();
        let random_y = rand::random::<f32>() * window.height() / 2.0 + window.height() / 2.0;
        let random_ship_type = rand::random::<u32>() % 5;
        let enemy_texture = match random_ship_type {
            0 => "enemy1.png",
            1 => "enemy2.png",
            2 => "enemy3.png",
            3 => "enemy4.png",
            4 => "enemy5.png",
            _ => "sistem.png",
        };

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load(enemy_texture),
                ..default()
            },
            Enemy {
                direction: Vec2::new(0.0, -1.0).normalize(),
                speed: ENEMY_SPEED,
            },
        ));
    }
}
