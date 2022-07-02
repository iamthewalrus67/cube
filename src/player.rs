use bevy::{
    ecs::schedule::ShouldRun,
    input::{keyboard::KeyboardInput, mouse::MouseMotion},
    prelude::*,
};

#[derive(Component)]
pub struct Player;

const PLAYER_SPEED: f32 = 20.0;

pub fn move_player_system(
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
) {
    for mut transform in player_query.iter_mut() {
        let mut new_pos = Vec3::ZERO;

        let forward = transform.forward();
        let right = transform.right();

        if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
            new_pos += forward;
        }

        if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
            new_pos += -forward;
        }

        if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
            new_pos += -right;
        }

        if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
            new_pos += right;
        }

        transform.translation += new_pos.normalize_or_zero() * PLAYER_SPEED * time.delta_seconds();
    }
}

#[derive(Default)]
pub struct MouseTracker {
    pitch: f32,
    yaw: f32,
}

const MOUSE_SENSITIVITY: f32 = 0.06;

pub fn camera_look_system(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut mouse_tracker: Local<MouseTracker>,
) {
    for mut transform in camera_query.iter_mut() {
        for ev in motion_evr.iter() {
            mouse_tracker.pitch -= (MOUSE_SENSITIVITY * ev.delta.y).to_radians();
            mouse_tracker.yaw -= (MOUSE_SENSITIVITY * ev.delta.x).to_radians();

            mouse_tracker.pitch = mouse_tracker.pitch.clamp(-1.54, 1.54);

            transform.rotation = Quat::from_axis_angle(Vec3::Y, mouse_tracker.yaw)
                * Quat::from_axis_angle(Vec3::X, mouse_tracker.pitch);
        }
    }
}

pub fn run_if_cursor_grabbed(windows: Res<Windows>) -> ShouldRun {
    let window = windows.get_primary().unwrap();

    if window.cursor_locked() {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }
}

fn setup_player(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Player);
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player).add_system_set(
            SystemSet::new()
                .with_run_criteria(run_if_cursor_grabbed)
                .with_system(camera_look_system)
                .with_system(move_player_system),
        );
    }
}
