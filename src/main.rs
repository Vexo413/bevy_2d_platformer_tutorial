use bevy::{color::palettes::css, prelude::*};
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, RapierPhysicsPlugin::<NoUserData>::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, (manage_collisions, manage_position))
        .insert_resource(PlayerPhysics {
            speed: 2.5,
            friction: 0.9,
            jump_force: 50.0,
            gravity: 1.5,
        })
        .run();
}

#[derive(Component)]
struct Player {
    velocity: Vec2,
    grounded: bool,
}

#[derive(Component)]
struct GroundSensor;

#[derive(Resource)]
struct PlayerPhysics {
    speed: f32,
    friction: f32,
    jump_force: f32,
    gravity: f32,
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Transform::from_scale(Vec3::splat(0.05)),
    ));
    commands.spawn((
        Sprite::from_color(css::WHITE, Vec2::new(40.0, 1.0)),
        RigidBody::Fixed,
        Collider::cuboid(20.0, 0.5),
        Transform::default(),
    ));
    commands.spawn((
        Sprite::from_color(css::RED, Vec2::new(2.0, 2.0)),
        RigidBody::Fixed,
        Collider::cuboid(1.0, 1.0),
        Transform::from_xyz(10.0, 1.5, 0.0),
    ));
    commands.spawn((
        Sprite::from_color(css::BLUE, Vec2::new(2.0, 2.0)),
        Collider::cuboid(1.0, 1.0),
        Sensor,
        Transform::from_xyz(10.0, 10.0, 0.0),
        ActiveEvents::COLLISION_EVENTS,
        ActiveCollisionTypes::all(),
    ));

    commands
        .spawn((
            Sprite::from_color(css::GREEN, Vec2::new(1.0, 1.0)),
            RigidBody::KinematicPositionBased,
            Collider::cuboid(0.5, 0.5),
            Transform::from_xyz(0.0, 50.0, 0.0),
            KinematicCharacterController {
                ..Default::default()
            },
            Player {
                velocity: Vec2::ZERO,
                grounded: false,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::ball(0.1),
                Sensor,
                Transform::from_xyz(0.0, -0.6, 0.0),
                GroundSensor,
                ActiveEvents::COLLISION_EVENTS,
                ActiveCollisionTypes::all(),
            ));
        });
}

fn manage_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut query: Query<&mut Player>,
    sensor_query: Query<Entity, With<GroundSensor>>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(entity1, entity2, _) => {
                if sensor_query.get(*entity1).is_ok() || sensor_query.get(*entity2).is_ok() {
                    if let Ok(mut player) = query.get_single_mut() {
                        player.grounded = true;
                    }
                }
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {
                if sensor_query.get(*entity1).is_ok() || sensor_query.get(*entity2).is_ok() {
                    if let Ok(mut player) = query.get_single_mut() {
                        player.grounded = false;
                    }
                }
            }
        }
    }
}

fn manage_position(
    time: Res<Time>,
    mut query: Query<(&mut KinematicCharacterController, &mut Player)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    player_physics: Res<PlayerPhysics>,
) {
    let (mut controller, mut player) = query.single_mut();

    let mut direction = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x = -1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x = 1.0;
    }

    player.velocity += direction * player_physics.speed * time.delta_secs();
    if player.grounded {
        player.velocity *= player_physics.friction;
    } else {
        player.velocity.x *= player_physics.friction;
    }
    player.velocity.y -= player_physics.gravity * time.delta_secs();

    if keyboard.pressed(KeyCode::KeyW) && player.grounded {
        player.velocity.y = player_physics.jump_force * time.delta_secs();
    }

    //player.velocity.y *= player_physics.air_friction;

    controller.translation = Some(player.velocity);
}
