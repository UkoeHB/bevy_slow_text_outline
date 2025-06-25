use bevy::prelude::*;
use bevy_slow_text_outline::prelude::*;

//-------------------------------------------------------------------------------------------------------------------

fn spawn(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>)
{
    commands.spawn(Camera2d);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(200.0, 50.0))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        children![
            (
                Text2d::new("1px"),
                TextColor(bevy::color::palettes::basic::WHITE.into()),
                TextOutline{ width:1.0, ..default() },
                Transform::from_translation(Vec3::default().with_x(-65.0).with_z(1.0)),
            ),
            (
                Text2d::new("2px"),
                TextColor(bevy::color::palettes::basic::WHITE.into()),
                TextOutline{ width:2.0, ..default() },
                Transform::from_translation(Vec3::default().with_x(-15.0).with_z(1.0)),
            ),
            (
                Text2d::new("2px+aa"),
                TextColor(bevy::color::palettes::basic::WHITE.into()),
                TextOutline{ width:2.0, anti_aliasing:Some(0.6), ..default() },
                Transform::from_translation(Vec3::default().with_x(50.0).with_z(1.0)),
            ),
        ],
    ));
}

//-------------------------------------------------------------------------------------------------------------------

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SlowTextOutlinePlugin::default())
        .add_systems(Startup, spawn)
        .run();
}

//-------------------------------------------------------------------------------------------------------------------
