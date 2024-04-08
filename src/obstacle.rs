use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::PROJECTION_SIZE;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ObstacleMaterial>()
            .add_event::<SpawnObstacleEvent>()
            .add_systems(Update, (obstacle_spawner, spawn_obstacle_sprite).chain());
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ObstacleOrigin {
    Top,
    Bottom,
}

#[derive(Component)]
pub struct Obstacle {
    origin: ObstacleOrigin,
    length: f32,
}

#[derive(Bundle)]
pub struct SpawnObstacleBundle {
    obstacle: Obstacle,
    transform: Transform,
}

#[derive(Resource)]
pub struct ObstacleMaterial(Handle<ColorMaterial>);

impl FromWorld for ObstacleMaterial {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.resource_mut::<Assets<ColorMaterial>>();
        let color = Color::rgba(0.8, 0.0, 0.0, 1.0);
        ObstacleMaterial(materials.add(color))
    }
}

pub fn spawn_obstacle_sprite(
    mut commands: Commands,
    material: Res<ObstacleMaterial>,
    mut query: Query<(Entity, &mut Transform, &Obstacle), Added<Obstacle>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, transform, obstacle) in &mut query {
        let shape = Mesh2dHandle(meshes.add(Rectangle::new(50.0, obstacle.length)));

        let y_pos = match obstacle.origin {
            ObstacleOrigin::Top => (PROJECTION_SIZE.y / 2.0) - (obstacle.length / 2.0),
            ObstacleOrigin::Bottom => (-PROJECTION_SIZE.y / 2.0) + (obstacle.length / 2.0),
        };

        commands.entity(entity).insert(MaterialMesh2dBundle {
            material: material.0.clone(),
            mesh: shape,
            transform: transform
                .with_translation(transform.translation + Vec3::new(0.0, y_pos, 0.0)),
            ..default()
        });
    }
}

#[derive(Event)]
pub struct SpawnObstacleEvent {
    pub x_position: f32,
    pub origin: ObstacleOrigin,
    pub length: f32,
}

pub fn obstacle_spawner(mut commands: Commands, mut events: EventReader<SpawnObstacleEvent>) {
    for event in events.read() {
        commands.spawn(SpawnObstacleBundle {
            obstacle: Obstacle {
                origin: event.origin,
                length: event.length,
            },
            transform: Transform::from_translation(Vec3::new(event.x_position, 0.0, 0.0)),
        });
    }
}
