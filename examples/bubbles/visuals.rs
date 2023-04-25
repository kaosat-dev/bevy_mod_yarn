pub fn update_text_position(
    windows: Res<Windows>,
  
    mut text_bubbles: Query<(&mut Style, &CalculatedSize, &DialogueBubble)>,
    with_dialogues : Query<(&GlobalTransform, &Collider), Or<(With<HasDialogue>, With<Interactible>)>>, // the 3d "entities" where there can be a dialogue bubble of sorts
    camera_query: Query<(&Camera, &GlobalTransform), With<Tracking>>,
  ) {
  
    let window_width = windows.primary().width(); // FIXME : not sure if this is reliable 
    let window_height = windows.primary().height();
  
    for (camera, camera_transform) in camera_query.iter() {
      for text_bubble in text_bubbles.iter_mut() {
        let (mut style, calculated, dialogue_bubble) = text_bubble;
  
           // we need to find the 2D Ui elements associated with the entity above
           if !dialogue_bubble.owner.is_none(){
            if let Ok(with_dialogue) = with_dialogues.get(dialogue_bubble.owner.unwrap()){
              // println!("with_dialogue :{:?}", dialogue_debug.owner.unwrap());
              // println!("bounding box : {:?}", with_dialogue.1);
  
              // we want to position bubbles ABOVE meshes, sadly the builtin Aabb component does not seem to have correct data,
              // so we use the collider (which is wrong, not all interactible things have a collider !!)
              let z_offset =  with_dialogue.1.raw.compute_local_bounding_sphere().radius;
              let mut offset_position = with_dialogue.0.translation().clone();
              offset_position.z += z_offset;
  
              match camera.world_to_ndc(camera_transform, offset_position)
              {
                  Some(coords) => {
                    if let Ok(width) = (calculated.size.width / 2.0).evaluate(1.0){
                      if let Ok(height) = (calculated.size.height / 2.0).evaluate(1.0){
                        // original values between -1.0 & 1.0
                        let mapped_x = coords.x * window_width * 0.5;
                        let mapped_y = coords.y * window_height * 0.5;
                        let mid_x = window_width/2.0 - width;
                        let mid_y = window_height/2.0 - height;
                        style.position.left =   Val::Px( mid_x + mapped_x );
                        style.position.top = Val::Px(mid_y - mapped_y - 50.);
                        style.display=Display::Flex; // since it was hidden, display it again (avoids text position flicker)
  
                      }
                    }
                  }
                  None => {
                      // A hack to hide the text when the cube is behind the camera
                      // TODO: actually toggle visibility
                      style.position.bottom = Val::Px(-1000.0);
                  }
              }
            }
           } 
      }
    }
  }
  