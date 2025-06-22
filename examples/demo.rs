use bevy::prelude::*;
use bevy_slow_text_outline::prelude::*;

//-------------------------------------------------------------------------------------------------------------------

fn spawn_ui(mut c: Commands)
{
    c.spawn(Camera2d);
    c.spawn((
        Node{ padding: UiRect::horizontal(Val::Px(10.0)), ..default() },
        BackgroundColor(Color::WHITE),
        children![
            (
                Node{ margin: UiRect::top(Val::Px(10.0)), ..default() },
                Text::new("1px"),
                TextColor(Color::WHITE),
                TextOutline{ width:1.0, ..default() }
            ),
            (
                Node{height:Val::Px(50.0), border:UiRect::left(Val::Px(1.0)), margin:UiRect::horizontal(Val::Px(5.0)), ..default()},
                BorderColor(Color::BLACK)
            ),
            (
                Node{ margin: UiRect::top(Val::Px(10.0)), ..default() },
                Text::new("2px"),
                TextColor(Color::WHITE),
                TextOutline{ width:2.0, ..default() }
            ),
            (
                Node{height:Val::Px(50.0), border:UiRect::left(Val::Px(1.0)), margin:UiRect::horizontal(Val::Px(5.0)), ..default()},
                BorderColor(Color::BLACK)
            ),
            (
                Node{ margin: UiRect::top(Val::Px(10.0)), ..default() },
                Text::new("2px+aa"),
                TextColor(Color::WHITE),
                TextOutline{ width:2.0, anti_aliasing:Some(0.6), ..default() }
            ),
        ]
    ));
}

//-------------------------------------------------------------------------------------------------------------------

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SlowTextOutlinePlugin::default())
        .add_systems(Startup, spawn_ui)
        .run();
}

//-------------------------------------------------------------------------------------------------------------------
