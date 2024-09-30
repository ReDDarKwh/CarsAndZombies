use bevy::prelude::*;
use bevy_flycam::prelude::*;
use bevy_rapier3d::prelude::*;

fn keyboard_input(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        // Space was pressed
        exit.send(AppExit::Success);
    }
}

struct Truster {
    position: Vec3,
}

#[derive(Component)]
struct TrusterGroup {
    trusters: Vec<Truster>,
    force: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::new(10.0, 10.0))),
        material: materials.add(Color::srgb_u8(124, 144, 0)),
        transform: Transform::from_xyz(0.0, -2.0, 0.0),
        ..default()
    });

    let f = 20.0;

    commands
        .spawn(RigidBody::Dynamic)
        .insert(TrusterGroup {
            force: 10.0,
            trusters: vec![
                Truster {
                    position: Vec3::new(0.0, 0.0, 0.0),
                },
                Truster {
                    position: Vec3::new(0.0, 0.0, 0.0),
                },
                Truster {
                    position: Vec3::new(0.0, 0.0, 0.0),
                },
                Truster {
                    position: Vec3::new(0.0, 0.0, 0.0),
                },
            ],
        })
        .insert(ExternalForce {
            force: Vec3::new(0.0, f, 0.0),
            torque: Vec3::ZERO,
        })
        // .insert(ExternalForce::at_point(
        //     Vec3::new(0.0, f, 0.0),
        //     Vec3::new(1.5, 0.0, -0.5),
        //     Vec3::new(0.0, 0.0, 0.0),
        // ))
        .insert(Collider::cuboid(1.5, 0.25, 0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(PbrBundle {
            mesh: meshes.add(Cuboid::new(3.0, 0.5, 1.0)),
            material: materials.add(Color::srgb_u8(124, 144, 255)),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 1.0, 0.0)));
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    /* Create the bouncing ball. */
}

fn truster_system(mut query: Query<(&TrusterGroup, &Transform, &mut ExternalForce)>) {
    for (truster_group, transform, mut force) in &mut query {
        
        force.force = Vec3::ZERO;
        
        for truster in truster_group.trusters {

            force.force = force.force + Vec3::new(
                0.0,
                transform.translation.y.remap(0.0, 2.0, 1.0, 0.0) * truster_group.force,
                0.0,
            );

        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(PlayerPlugin)
        .insert_resource(ClearColor(Color::srgb(0.9, 0.3, 0.6)))
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, (keyboard_input, truster_system))
        .run();
}
