use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use wasm_bindgen::prelude::*;


struct CoffeePlugin;

impl Plugin for CoffeePlugin {
  fn build(&self, app: &mut App) {
    //
    // TODO
    //
    //app.insert_resource()
    //
  }
}

/*
struct ActionsPlugin;

impl Plugin for ActionsPlugin {
  fn build(&self, app: &mut App) {
    //
    // TODO
    //
    app.init_resource::<Actions>().add_system_set(
      SystemSet
    //
  }
}
*/

fn setup(mut commands: Commands) {
  let mut cam = Camera2dBundle::default();
  cam.projection.scaling_mode = ScalingMode::FixedVertical(10.);
  commands.spawn_bundle(cam);
}

#[derive(Component)]
struct Spriter;

fn spawn_simon(mut commands: Commands) {
  //
  // TODO: build simon here
  //
  commands.spawn_bundle(SpriteBundle {
    sprite: Sprite {
      color:  Color::rgb(0., 1., 0.),
      custom_size: Some(Vec2::new(1., 1.)),
      ..default()
    },
    ..default()
  })
  .insert(Spriter);
}

fn moving(time: Res<Time>, mut spriters: Query<&mut Transform, With<Spriter>>) {
  //
  for mut spriter in &mut spriters {
    spriter.rotate_z(4. * time.delta_seconds());
  }
  //
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum GameState {
  //
  // TODO
  //
  Static,
  Rotating
  //
}

#[wasm_bindgen]
pub fn start_soe() {
  App::new()
    .add_state(GameState::Rotating)
    .insert_resource(ClearColor(Color::rgb(1., 1., 1.)))
    .add_plugins(DefaultPlugins.set( WindowPlugin {
      window: WindowDescriptor {
        fit_canvas_to_parent: true,
        canvas: Some("#soe-bevy".to_owned()),
        ..Default::default()
      },
      ..default()
    }))
    .add_plugin(CoffeePlugin)
    .add_startup_system(setup)
    .add_startup_system(spawn_simon)
    //
    .add_system_set(SystemSet::on_update(GameState::Rotating).with_system(moving))
    //
    .run();
  //
  // TODO
  //
  //
}

