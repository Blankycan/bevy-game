use bevy::prelude::*;
use bevy_mod_aseprite::*;

pub fn build_component_sprite(commands: Commands) {
    println!("build_component_sprite");
}

pub fn load_assets(asset_server: Res<AssetServer>, mut aseprite_handles: ResMut<AsepriteHandles>) {
    let character: Handle<Aseprite> = asset_server.load("Character.aseprite");
    aseprite_handles.push(character);
}

pub fn setup(
    mut commands: Commands,
    aseprite_handles: Res<AsepriteHandles>,
    aseprites: Res<Assets<Aseprite>>,
) {
    let aseprite_handle = &aseprite_handles[0];
    let aseprite = aseprites.get(aseprite_handle).unwrap();
    let animation = AsepriteAnimation::new(aseprite.info(), "idle");

    commands.spawn(Player).insert(AsepriteBundle {
        texture_atlas: aseprite.atlas().clone_weak(),
        sprite: TextureAtlasSprite::new(animation.current_frame()),
        aseprite: aseprite_handle.clone_weak(),
        animation,
        ..default()
    });
}

fn transition_player(
    time: Res<Time>,
    player_q: Query<(&PlayerState, &Handle<Aseprite>, &AsepriteAnimation), With<Player>>,
    aseprites: Res<Assets<Aseprite>>,
    mut ev_player_changed: EventWriter<PlayerChanged>,
) {
    let (&player_state, handle, anim) = player_q.single();
    let aseprite = aseprites.get(handle).unwrap();
    match player_state {
        PlayerState::Attack => {
            let remaining_frames = anim.remaining_tag_frames(aseprite.info()).unwrap();
            let frame_finished = anim.frame_finished(time.delta());
            if remaining_frames == 0 && frame_finished {
                ev_player_changed.send(PlayerChanged::default().new_state(PlayerState::Stand));
            }
        }
        _ => (),
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
struct AsepriteHandles(Vec<Handle<Aseprite>>);
