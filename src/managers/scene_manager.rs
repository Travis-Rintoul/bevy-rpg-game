// use bevy::prelude::*;

// use crate::{models::enums::game_scene::GameScene, plugins::scenes::location1_plugin::Location1ScenePlugin};

// pub struct SceneManager;

// impl SceneManager {
//     pub fn new() -> Self {
//         SceneManager {}	
//     }

//     pub fn setup(
//         mut commands: Commands,
//         mut meshes: ResMut<Assets<Mesh>>,
//         mut materials: ResMut<Assets<StandardMaterial>>,
//     ) {
        
//     }

//     pub fn cleanup() {
        
//     }

//     pub fn on_scene_change(
//         scene: Res<State<GameScene>>,
//         commands: &mut Commands
//     ) {
//         match *scene.get() {
//             GameScene::Location1 => {
//                 println!("Entering Location1 Scene...");
//                 app.add_plugins(Location1ScenePlugin);
//             },
//             _ => {}
//         }
//     }
// }