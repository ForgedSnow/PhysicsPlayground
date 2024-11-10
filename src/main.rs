use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct BigBall;

#[derive(Component)]
struct BigBallCore(Vec2);

const RADIUS: f32 = 250.0;
const PI: f32 = std::f32::consts::PI;
const BLACK: Color = Color::hsl(0.0, 0.0, 1.0);
const WHITE: Color = Color::hsl(0.0, 0.0, 0.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup_graphics, setup_physics))
        .add_systems(Update, move_big_ball)
        .insert_resource(ClearColor(WHITE))
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let radius = RADIUS;
    let faces = 64;
    let perimeter = 2.0 * faces as f32 * radius * f32::sin(PI / faces as f32);
    let cuboid_height = 1.0;
    let face_length = perimeter / faces as f32;
    let minor_radius = f32::cos(PI / faces as f32) * radius;

    for i in 0..faces {
        let angle = (i as f32) * (2.0 * PI / faces as f32);
        let x = minor_radius * angle.cos();
        let y = minor_radius * angle.sin();
        let mut position = Transform::from_translation(Vec3::new(x, y, 0.0));
        position.rotate_z(angle + PI / 2.0);
        commands.spawn((
            RigidBody::Fixed,
            Collider::cuboid(face_length / 2.0, cuboid_height / 2.0),
            position,
            GlobalTransform::IDENTITY,
            BigBall,
        ));
    }

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius })),
            material: materials.add(BLACK),
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -0.1,
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(BigBallCore(Vec2 { x: 100.0, y: 125.0 }));

    let radius = 50.0;

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(radius))
        .insert(Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Max,
        })
        .insert(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius })),
            material: materials.add(WHITE),
            ..Default::default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 200.0, 0.0)));
}

fn move_big_ball(
    mut core: Query<(&mut BigBallCore, &mut Transform), Without<BigBall>>,
    mut edges: Query<&mut Transform, With<BigBall>>,
    time: Res<Time>,
    window: Query<&Window>,
) {
    let window = window.single();
    let width = window.resolution.width();
    let height = window.resolution.height();

    let mut ball_speed = Vec3::default();
    for (mut ball, mut transform) in core.iter_mut() {
        let mut velocity = ball.0;
        ball_speed = Vec3::from((velocity, 0.0)) * time.delta_seconds();
        let future_pos = transform.translation + ball_speed;
        if future_pos.x + RADIUS >= width / 2.0 || future_pos.x - RADIUS <= -width / 2.0 {
            velocity.x = -velocity.x;
        }
        if future_pos.y + RADIUS >= height / 2.0 || future_pos.y - RADIUS <= -height / 2.0 {
            velocity.y = -velocity.y;
        }
        ball_speed = Vec3::from((velocity, 0.0)) * time.delta_seconds();
        transform.translation += ball_speed;
        ball.0 = velocity;
    }
    for mut transform in edges.iter_mut() {
        transform.translation += ball_speed;
    }
}
