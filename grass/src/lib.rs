use gdnative::api::PackedScene;
use gdnative::api::ResourceLoader;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
struct Grass;

#[methods]
impl Grass {
    fn new(_owner: &Node2D) -> Self {
        Grass
    }

    #[godot]
    fn _process(&self, #[base] owner: &Node2D, _delta: f32) {
        let input = Input::godot_singleton();
        if input.is_action_just_pressed("attack", false) {
            let grass_effect_resource =
                ResourceLoader::godot_singleton().load("res://Effects/GrassEffect.tscn", "", false);
            let grass_effect = unsafe {
                grass_effect_resource
                    .unwrap()
                    .assume_safe()
                    .cast::<PackedScene>()
                    .unwrap()
                    .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
                    .unwrap()
            };
            unsafe {
                owner
                    .get_tree()
                    .unwrap()
                    .assume_safe()
                    .current_scene()
                    .unwrap()
                    .assume_safe()
                    .add_child(grass_effect.assume_safe(), false)
            };
            unsafe {
                grass_effect
                    .assume_safe()
                    .cast::<Node2D>()
                    .unwrap()
                    .set_global_position(owner.global_position());
            }

            owner.queue_free();
        }
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<Grass>();
}

godot_init!(init);
