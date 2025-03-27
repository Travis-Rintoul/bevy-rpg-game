pub struct SceneResources<'a> {
	pub commands: &'a mut Commands,
    pub meshes: &'a mut ResMut<'a, Assets<Mesh>>,
    pub materials: &'a mut ResMut<'a, Assets<StandardMaterial>>,
}