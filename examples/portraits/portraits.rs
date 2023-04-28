use bevy::prelude::*;
use bevy_mod_yarn::prelude::*;

fn main() {
    App::new()

        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))

        .add_plugin(YarnPlugin)
        .init_resource::<State>()

        .add_startup_system(setup)
        .add_system(dialogue_init)
        .add_system(dialogue_navigation)
        .add_system(dialogue_display)

        .run();
}

#[derive(Resource, Default)]
struct State {
    handle: Handle<YarnAsset>,
    done: bool
}

#[derive(Component)]

struct CharacterName(String);

#[derive(Component)]

struct CharacterPortraitPath(String);

#[derive(Component)]

struct CharacterPortrait(Handle<Image>);


// marker components
#[derive(Component)]
struct DialogueTextMarker;

#[derive(Component)]
struct DialogueNameMarker;

fn setup(
    mut state: ResMut<State>, 
    asset_server: Res<AssetServer>, 
    mut commands: bevy::prelude::Commands
) {
    // load the yarn file
    state.handle = asset_server.load("dialogues/two_nodes_jump_nested_choices.yarn");
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });


    // for character dialogue text
    commands.spawn((
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
            position: UiRect {
                bottom: Val::Px( 30.0),
                left: Val::Px(80.0),
                ..default()
            },
            ..default()
        }),
        DialogueTextMarker
        )
    );

    // for character names
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 18.0,
                color: Color::GOLD,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px( 60.0),
                left: Val::Px(80.0),
                ..default()
            },
            ..default()
        }),
        DialogueNameMarker
        )
    );

    commands
        .spawn(NodeBundle {
            style: Style {
                // size: Size::width(Val::Percent(100.)),
                // position_type: PositionType::Absolute,
                // justify_content: JustifyContent::Center,
                // align_items: AlignItems::FlexStart,
                position_type: PositionType::Absolute,
                position: UiRect {
                bottom: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            },
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // bevy logo (image)
            parent
                .spawn(ImageBundle {
                    style: Style {
                        size: Size::new(Val::Px(64.0), Val::Px(64.0)),
                        ..default()
                    },
                    image: asset_server.load("textures/portrait0.png").into(),
                    ..default()
                })
                .with_children(|parent| {
                    // alt text
                    parent
                        .spawn(TextBundle::from_section("portraits", TextStyle::default()));
                });
        });

    // spawn our characters
    commands.spawn((
        CharacterName("Lamik".to_string()),
        CharacterPortraitPath("textures/portrait1.png".to_string()),
        CharacterPortrait(asset_server.load("textures/portrait1.png").into())
        
    ));

    commands.spawn((
        CharacterName("Dona".to_string()),
        CharacterPortraitPath("textures/portrait2.png".to_string()),
        CharacterPortrait(asset_server.load("textures/portrait2.png").into())
    ));
}

fn dialogue_init(mut state: ResMut<State>, dialogues: Res<Assets<YarnAsset>>, mut commands: bevy::prelude::Commands) {
    if let Some(dialogues)= dialogues.get(&state.handle) {
        if !state.done {
            commands.spawn(
                DialogueRunner::new(dialogues.clone(), "Test_node")
            );
            state.done = true;
        }
    }
}

fn dialogue_navigation(
    keys: Res<Input<KeyCode>>,
    mut runners: Query<&mut DialogueRunner>,
) {
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

fn dialogue_display(
    runners: Query<&DialogueRunner>,
    mut name_display: Query<&mut Text ,(With<DialogueNameMarker>, Without<DialogueTextMarker>)>,
    mut text: Query<&mut Text ,(With<DialogueTextMarker>, Without<DialogueNameMarker>) >,
    mut portrait: Query<&mut UiImage>,
    characters: Query<(&CharacterName, &CharacterPortrait)>
){
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;
    *text = "".to_string();
    text.push_str("------------------------------\n");

    let mut name_display = name_display.single_mut();
    let name_display = &mut name_display.sections[0].value;
    *name_display = "".to_string();

    let mut portrait = portrait.single_mut();
    
    if let Ok(runner) = runners.get_single() {
        match runner.current_statement() {
            Statements::Dialogue(dialogue) => {
                // println!("{:?}: {:?}", dialogue.who, dialogue.what);
                text.push_str(&format!("{}\n", dialogue.what));

                // FIXME: very inneficient, but does the job, perhaps switch to for each to break out early
                for (name, portrait_img) in characters.iter() {
                    if name.0 == dialogue.who {                        
                        portrait.texture = portrait_img.0.clone();
                        name_display.push_str(&name.0);
                    }
                }
            }
            Statements::Choice(_) => {
                let (choices, current_choice_index) = runner.get_current_choices();
                for (index, dialogue) in choices.iter().enumerate(){
                    *name_display = "".to_string();

                    if index == current_choice_index{
                        // text.push_str(&format!("--> {:?}: {:?}\n", dialogue.who, dialogue.what));
                        text.push_str(&format!("--> {}\n", dialogue.what));

                    }else {
                        // text.push_str(&format!("{:?}: {:?}\n", dialogue.who, dialogue.what));
                        text.push_str(&format!("{}\n", dialogue.what));
                    }

                    // FIXME: very inneficient, but does the job, perhaps switch to for each to break out early
                    for (name, portrait_img) in characters.iter() {
                        if name.0 == dialogue.who {                        
                            portrait.texture = portrait_img.0.clone();
                            name_display.push_str(&name.0);
                        }
                    }
                }
            }
            Statements::Exit => {
                text.push_str("end of the node! (Exit)");
            },
            _ => {
                
            }
        }
    }
}