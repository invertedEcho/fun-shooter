fn check_collider_constructor_hierarchy_ready(
    _trigger: On<ColliderConstructorHierarchyReady>,
    colliders: Query<(&Collider, &GlobalTransform)>,
    mut local: Local<usize>,
) {
    let mut saved_colliders: Vec<(Collider, GlobalTransform)> = Vec::new();
    for collider in colliders {
        saved_colliders.push((collider.0.clone(), *collider.1));
    }
    let json = serde_json::to_string(&saved_colliders);
    match json {
        Ok(serialized) => {
            let filename = format!(
                "/home/echo/Downloads/shooter-collider-{}.json",
                *local
            );
            if let Err(error) = File::create(&filename)
                .unwrap()
                .write_all(serialized.as_bytes())
            {
                error!("Failed to save collider file: {}", error);
            } else {
                info!("Saved collider file to {filename}!");
                *local += 1;
            }
        }
        Err(error) => {
            error!("Failed to convert colliders to json string: {}", error);
        }
    }
}
