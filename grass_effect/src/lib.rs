use gdnative::api::AnimatedSprite;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
struct GrassEffect {
    animated_sprite: Option<Ref<AnimatedSprite>>,
}

#[methods]
impl GrassEffect {
    fn new(_owner: &Node2D) -> Self {
        GrassEffect {
            animated_sprite: None,
        }
    }

    #[godot]
    fn _ready(&mut self, #[base] owner: &Node2D) {
        self.animated_sprite = unsafe {
            Some(
                owner
                    .get_node_as::<AnimatedSprite>("AnimatedSprite")
                    .unwrap()
                    .claim(),
            )
        };
        unsafe {
            self.animated_sprite.unwrap().assume_safe().set_frame(0);
        }
        unsafe {
            self.animated_sprite
                .unwrap()
                .assume_safe()
                .play("Animate", false);
        }
    }

    #[allow(non_snake_case)]
    #[godot]
    fn _on_AnimatedSprite_animation_finished(&self, #[base] owner: &Node2D) {
        owner.queue_free();
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<GrassEffect>();
}

godot_init!(init);
