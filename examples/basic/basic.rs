use bevy::prelude::*;
use bevy_mod_yarn::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, YarnPlugin))
        .init_resource::<State>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (dialogue_init, dialogue_navigation, dialogue_display),
        )
        .run();
}

#[derive(Resource, Default)]
struct State {
    handle: Handle<YarnAsset>,
    done: bool,
}

fn setup(
    mut state: ResMut<State>,
    asset_server: Res<AssetServer>,
    mut commands: bevy::prelude::Commands,
) {
    // load the yarn dialogue file
    state.handle = asset_server.load("dialogues/single_node_simple.yarn");

    // setup a simple 2d camera
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    });

    commands.spawn(
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 18.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}

fn dialogue_init(
    mut state: ResMut<State>,
    dialogues: Res<Assets<YarnAsset>>,
    mut commands: bevy::prelude::Commands,
) {
    if let Some(dialogues) = dialogues.get(&state.handle) {
        if !state.done {
            commands.spawn(DialogueRunner::new(dialogues.clone(), "Start"));
            state.done = true;
        }
    }
}

fn dialogue_navigation(keys: Res<Input<KeyCode>>, mut runners: Query<&mut DialogueRunner>) {
    if let Ok(mut runner) = runners.get_single_mut() {
        if keys.just_pressed(KeyCode::Return) {
            runner.next_entry();
        }
        if keys.just_pressed(KeyCode::Down) {
            println!("next choice");
            runner.next_choice()
        }
        if keys.just_pressed(KeyCode::Up) {
            println!("prev choice");
            runner.prev_choice()
        }
    }
}

fn dialogue_display(runners: Query<&DialogueRunner>, mut text: Query<&mut Text>) {
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;
    *text = "".to_string();
    text.push_str("------------------------------\n");

    if let Ok(runner) = runners.get_single() {
        match runner.current_statement() {
            Statements::Dialogue(dialogue) => {
                text.push_str(&format!("{:?}: {:?}\n", dialogue.who, dialogue.what));
            }
            Statements::Choice(_) => {
                let (choices, current_choice_index) = runner.get_current_choices();
                for (index, dialogue) in choices.iter().enumerate() {
                    if index == current_choice_index {
                        text.push_str(&format!("--> {:?}: {:?}\n", dialogue.who, dialogue.what));
                    } else {
                        text.push_str(&format!("{:?}: {:?}\n", dialogue.who, dialogue.what));
                    }
                }
            }
            Statements::Exit => {
                text.push_str("end of the dialogue! (Exit)");
            }
            _ => {}
        }
    }
}
