use bevy::{
    ecs::query::{QueryData, QueryFilter, ROQueryItem},
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    prelude::*,
};

pub struct CollisionPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_systems(Update, collision_detection);
    }
}

#[derive(Component)]
pub enum Collider {
    Circle(BoundingCircle),
    Aabb(Aabb2d),
}

impl Collider {
    fn with_transform(&self, transform: &Transform) -> Collider {
        match self {
            Collider::Circle(c) => Self::Circle(BoundingCircle::new(
                c.center + transform.translation.xy(),
                c.radius(),
            )),
            Collider::Aabb(b) => Self::Aabb(Aabb2d {
                min: b.min + transform.translation.xy(),
                max: b.max + transform.translation.xy(),
            }),
        }
    }

    fn collides_with(&self, other: &Collider) -> bool {
        match self {
            Collider::Circle(a) => match other {
                Collider::Circle(b) => a.intersects(b),
                Collider::Aabb(b) => a.intersects(b),
            },
            Collider::Aabb(a) => match other {
                Collider::Circle(b) => a.intersects(b),
                Collider::Aabb(b) => a.intersects(b),
            },
        }
    }
}

#[derive(Event)]
pub struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

impl CollisionEvent {
    pub fn get_query_entity<'a, D: QueryData, F: QueryFilter>(
        &self,
        query: &'a Query<'_, '_, D, F>,
    ) -> Result<(ROQueryItem<'a, D>, Entity), ()> {
        if let Ok(data) = query.get(self.entity_a) {
            Ok((data, self.entity_b))
        } else if let Ok(data) = query.get(self.entity_b) {
            Ok((data, self.entity_a))
        } else {
            Err(())
        }
    }
}

// impl<'w, 's, D: QueryData, F: QueryFilter> Query<'w, 's, D, F>

fn collision_detection(
    colliders: Query<(Entity, &Transform, &Collider)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    for [(entity_a, transform_a, collider_a), (entity_b, transform_b, collider_b)] in
        colliders.iter_combinations()
    {
        let world_a = collider_a.with_transform(transform_a);
        let world_b = collider_b.with_transform(transform_b);

        if world_a.collides_with(&world_b) {
            collision_events.send(CollisionEvent { entity_a, entity_b });
        }
    }
}
