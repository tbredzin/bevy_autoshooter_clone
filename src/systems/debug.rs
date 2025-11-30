/*
   for button in GamepadButton::all() {
       commands.entity(parent).with_child((
           HUDLevelUp {},
           Text(format!("{:?}", &button)),
           Node {
               width: Val::Px(32.0),
               ..default()
           },
           children![
ImageNode::from_atlas_image(
   gamepad_asset.texture.clone(),
   TextureAtlas::from(gamepad_asset.layout.clone())
       .with_index(gamepad_asset.get_button_index(&button)),
),
               Node {
                   position_type: PositionType::Relative,
                   margin: UiRect::top(Val::Px(50.0)),
                   ..default()
               },
           ],
       ));
   }
*/
