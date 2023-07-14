use bevy::{prelude::*, render::primitives::Aabb};
use bevy_mod_yarn::prelude::*;

fn main() {
    App::new()

        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1024., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(YarnPlugin)
        .init_resource::<State>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            dialogue_init,
            dialogue_navigation,
            dialogue_display,
        ))
        .run();
}

#[derive(Resource, Default)]
struct State {
    handle: Handle<YarnAsset>,
    done: bool
}

#[derive(Component)]

struct CharacterName(String);

// marker components
#[derive(Component)]
struct DialogueTextMarker;
#[derive(Component)]
struct DialogueNameMarker;

fn setup(
    mut state: ResMut<State>, 
    asset_server: Res<AssetServer>, 
    mut commands: bevy::prelude::Commands,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

) {
    // load the yarn file
    state.handle = asset_server.load("dialogues/single_node_three_characters.yarn");

    // bevy boilerplate
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(6.0, 12.0, 6.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 122000.0,
            range: 150.,
            shadows_enabled: true,
            color: Color::ORANGE,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 18.0),
        ..default()
    });

    // ground plane
      commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        material: materials.add(Color::SILVER.into()),
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
            bottom: Val::Px( 30.0),
            left: Val::Px(80.0),      
            max_width: Val::Px(300.0),     
            max_height: Val::Px(40.0),
            ..default()
        }),
        DialogueTextMarker
    ));


    // character stand ins in the 3d world
    commands.spawn((
        CharacterName("Lamik".to_string()),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            material: materials.add(Color::rgb(0.8, 0.1, 0.1).into()),
            transform: Transform::from_xyz(
                0.0,
                1.0,
                0.0,
            ),
            ..default()
        }
    ));


    commands.spawn((
        CharacterName("Dona".to_string()),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            material: materials.add(Color::rgb(0.2, 0.8, 0.1).into()),
            transform: Transform::from_xyz(
                3.0,
                1.0,
                0.0,
            ),
            ..default()
        }
    ));

    commands.spawn((
        CharacterName("Blob".to_string()),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            material: materials.add(Color::rgb(0.1, 0.3, 0.8).into()),
            transform: Transform::from_xyz(
                1.0,
                1.0,
                5.0,
            ),
            ..default()
        }
    ));

}

fn dialogue_init(mut state: ResMut<State>, dialogues: Res<Assets<YarnAsset>>, mut commands: bevy::prelude::Commands) {
    if let Some(dialogues)= dialogues.get(&state.handle) {
        if !state.done {
            commands.spawn(
                DialogueRunner::new(dialogues.clone(), "Start")
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
    mut text: Query<(&mut Text, &mut Style, &Node) ,(With<DialogueTextMarker>, Without<DialogueNameMarker>) >, //  &CalculatedSize
    characters: Query<(&CharacterName, &GlobalTransform, &Aabb)>,

    // for projection & placing bubble above characters
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
){
    let mut text_data = text.single_mut();
    let text = &mut text_data.0.sections[0].value;
    *text = "".to_string();
    // text.push_str("------------------------------\n");

    let mut bubble_style = text_data.1;
    let bubble_size = text_data.2.size();

    let window = windows.single();
    
    let camera = cameras.single();
    let (camera, camera_global_transform) = camera;

    if let Ok(runner) = runners.get_single() {
        match runner.current_statement() {
            Statements::Dialogue(dialogue) => {
                // println!("{:?}: {:?}", dialogue.who, dialogue.what);
                text.push_str(&format!("{}\n", dialogue.what));

                // FIXME: very inneficient, but does the job, perhaps switch to for each to break out early
                for (name, global_transform, aabb) in characters.iter() {
                    if name.0 == dialogue.who {       
                        // we want to position bubbles ABOVE meshes, sadly the builtin Aabb component does not seem to have correct data,
                        place_bubble(aabb, global_transform, camera, camera_global_transform, window, &bubble_size, &mut bubble_style);
                    }
                }
            }
            Statements::Choice(_) => {
                let (choices, current_choice_index) = runner.get_current_choices();
                for (index, dialogue) in choices.iter().enumerate(){
                    // *name_display = "".to_string();

                    if index == current_choice_index{
                        // text.push_str(&format!("--> {:?}: {:?}\n", dialogue.who, dialogue.what));
                        text.push_str(&format!("--> {}\n", dialogue.what));

                    }else {
                        // text.push_str(&format!("{:?}: {:?}\n", dialogue.who, dialogue.what));
                        text.push_str(&format!("{}\n", dialogue.what));
                    }

                    // FIXME: very inneficient, but does the job, perhaps switch to for each to break out early
                    for (name, global_transform, aabb) in characters.iter() {
                        if name.0 == dialogue.who {                 
                            place_bubble(aabb, global_transform, camera, camera_global_transform, window, &bubble_size, &mut bubble_style);
                            // portrait.texture = portrait_img.0.clone();
                            // name_display.push_str(&name.0);
                        }
                    }
                }
            }
            Statements::Exit => {
                text.push_str("");//"end of the node! (Exit)");
            },
            _ => {
                
            }
        }
    }
}

fn place_bubble(
    aabb: &Aabb, global_transform: &GlobalTransform, 
    camera: &Camera, camera_global_transform: &GlobalTransform, 
    window: &Window, 
    bubble_size: &Vec2,
    mut bubble_style:  &mut Style
){
    let vertical_offset =  aabb.half_extents.y;
    let mut offset_position = global_transform.translation().clone();
    offset_position.y += vertical_offset; 

    match camera.world_to_ndc(camera_global_transform, offset_position)
    {
        Some(coords) => {
            // println!("unprojected coords {}",coords);
            let width = bubble_size.x/ 2.0;
            let height = bubble_size.y/ 2.0;
            let mapped_x = coords.x * window.width() * 0.5;
            let mapped_y = coords.y * window.height() * 0.5;
            let mid_x = window.width()/2.0 - width;
            let mid_y = window.height()/2.0 - height;
            bubble_style.left =   Val::Px( mid_x + mapped_x );
            bubble_style.top = Val::Px(mid_y - mapped_y - 50.);
            bubble_style.display=Display::Flex; // since it was hidden, display it again (avoids text position flicker)
        }
        None => {

        }
    }
}