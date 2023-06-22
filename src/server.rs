use ambient_api::{
    animation::{AnimationPlayer, PlayClipFromUrlNode},
    components::core::camera::aspect_ratio_from_window,
    components::core::{animation::apply_animation_player, app::main_scene, model::model_from_url},
    concepts::make_perspective_camera,
    concepts::make_transformable,
    entity::despawn,
    prelude::*,
};

#[main]
pub async fn main() {
    Entity::new()
        .with_merge(make_perspective_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), Vec3::ONE * 5.0)
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();
    println!("Start Loading"); // this works
    let clip = PlayClipFromUrlNode::new(
        asset::url(format!("assets/landing.fbx/animations/mixamo.com.anim")).unwrap(),
    );
    let animator = AnimationPlayer::new(&clip);
    let model = make_transformable()
        .with(model_from_url(), asset::url("assets/X Bot.fbx").unwrap())
        .with(apply_animation_player(), animator.0)
        .spawn();
    clip.wait_until_loaded().await;
    println!("Congratz you Loaded"); // Never reached
    despawn(model);
}
