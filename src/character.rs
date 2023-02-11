use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct TurnTowardCamera(pub bool);

pub fn turning_toward_camera(
  mut query: Query<(&TurnTowardCamera, &mut Transform)>,
  camera: Query<&Transform, (With<Camera>, Without<TurnTowardCamera>)>,
  time: Res<Time>
) {
  let camera = camera.single();
  for (should_turn, mut obj_transform) in &mut query {
    if should_turn.0 == true {
      let new_rotation = obj_transform.looking_at(camera.translation, Vec3::Y).rotation * Quat::from_euler(EulerRot::XYZ, 0.0, 3.14, 0.0);
      let delta_rotation = Quat::slerp(obj_transform.rotation, new_rotation, time.delta_seconds() * 5.0);
      obj_transform.rotation.y = delta_rotation.y;
    }
  }
}


#[derive(Bundle)]
pub struct CharacterBundle {
    pub turn_to_camera: TurnTowardCamera
}

impl Default for CharacterBundle {
    fn default() -> Self {
        Self {
            turn_to_camera: TurnTowardCamera(true)
        }
    }
}