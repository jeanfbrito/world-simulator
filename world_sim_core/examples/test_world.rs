//! Small test world example for tilemap integration

use bevy::prelude::*;
use world_sim_core::{
    tilemap::{TilemapPlugin, world_generation::{WorldGenConfig, BiomeType, generate_world_wfc}},
    tilemap::world_grid::WorldGrid,
    components::{PositionComponent, MovementComponent, NameComponent},
    systems::movement_system::movement_system,
    tilemap::pathfinding::process_path_requests,
};
use world_sim_interface::Position;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Small Test World".to_string(),
                resolution: (800.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, (setup_small_world, spawn_test_entities).chain())
        .add_systems(Update, (
            movement_system,
            process_path_requests,
            camera_movement,
            request_movement,
        ))
        .run();
}

/// Setup a small test world
fn setup_small_world(
    mut commands: Commands,
    mut world_grid: ResMut<WorldGrid>,
) {
    // Create small world config (32x32 instead of 128x128)
    let config = WorldGenConfig {
        width: 32,
        height: 32,
        seed: 12345,
        biome_type: BiomeType::Mixed,
        resource_density: 0.2,
    };
    
    // Initialize world grid
    *world_grid = WorldGrid::new(config.width, config.height);
    
    // Generate the world
    generate_world_wfc(&config, &mut world_grid);
    
    info!("Generated small test world: {}x{}", config.width, config.height);
    
    // Setup camera
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(
            (config.width as f32 * 16.0),
            (config.height as f32 * 16.0),
            999.0
        ),
        ..default()
    });
}

/// Spawn a few test entities
fn spawn_test_entities(
    mut commands: Commands,
    world_grid: Res<WorldGrid>,
) {
    // Find walkable spawn positions
    let spawn_positions = vec![
        Position::new(5, 5),
        Position::new(10, 10),
        Position::new(15, 15),
    ];
    
    for (i, pos) in spawn_positions.iter().enumerate() {
        // Check if position is walkable
        if let Some(tile) = world_grid.get_tile(pos) {
            if tile.is_walkable() {
                commands.spawn((
                    PositionComponent { position: *pos },
                    MovementComponent::new(2.0),
                    NameComponent(format!("Test Entity {}", i + 1)),
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(0.0, 1.0, 0.0),
                            custom_size: Some(Vec2::new(24.0, 24.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            pos.x as f32 * 32.0,
                            pos.y as f32 * 32.0,
                            10.0
                        ),
                        ..default()
                    },
                ));
                info!("Spawned test entity {} at {:?}", i + 1, pos);
            }
        }
    }
}

/// Simple camera movement with arrow keys
fn camera_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    if let Ok(mut transform) = camera_query.get_single_mut() {
        let speed = 200.0 * time.delta_seconds();
        
        if keyboard.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= speed;
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            transform.translation.x += speed;
        }
        if keyboard.pressed(KeyCode::ArrowUp) {
            transform.translation.y += speed;
        }
        if keyboard.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= speed;
        }
    }
}

/// Request movement on mouse click
fn request_movement(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut entities: Query<&mut MovementComponent>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        if let Ok(window) = windows.get_single() {
            if let Some(cursor_pos) = window.cursor_position() {
                if let Ok((camera, camera_transform)) = camera_query.get_single() {
                    // Convert screen to world coordinates
                    if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                        let tile_x = (world_pos.x / 32.0) as i32;
                        let tile_y = (world_pos.y / 32.0) as i32;
                        let target = Position::new(tile_x, tile_y);
                        
                        // Move first entity to clicked position
                        if let Some(mut movement) = entities.iter_mut().next() {
                            movement.set_target(Some(target));
                            info!("Requested movement to {:?}", target);
                        }
                    }
                }
            }
        }
    }
}