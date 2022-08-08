use gdnative::api::AnimationNodeStateMachinePlayback;
use gdnative::api::AnimationPlayer;
use gdnative::api::AnimationTree;
use gdnative::prelude::*;

enum State {
    Move,
    Roll,
    Attack,
}

const MAX_SPEED: f32 = 80.0;
const ACCELERATION: f32 = 500.0;
const ROLL_SPEED: f32 = 120.0;
const FRICTION: f32 = 500.0;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
struct Player {
    velocity: Vector2,
    roll_vector: Vector2,
    animation_player: Option<Ref<AnimationPlayer>>,
    animation_tree: Option<Ref<AnimationTree>>,
    animation_state: Option<Ref<AnimationNodeStateMachinePlayback>>,
    state: State,
}

#[methods]
impl Player {
    fn new(_owner: &KinematicBody2D) -> Self {
        Player {
            velocity: Vector2::ZERO,
            roll_vector: Vector2::LEFT,
            animation_player: None,
            animation_tree: None,
            animation_state: None,
            state: State::Move,
        }
    }

    #[godot]
    fn _ready(&mut self, #[base] owner: &KinematicBody2D) {
        owner.set_physics_process(true);
        self.animation_player = unsafe {
            Some(
                owner
                    .get_node_as::<AnimationPlayer>("AnimationPlayer")
                    .unwrap()
                    .claim(),
            )
        };
        self.animation_tree = unsafe {
            Some(
                owner
                    .get_node_as::<AnimationTree>("AnimationTree")
                    .unwrap()
                    .claim(),
            )
        };
        unsafe {
            self.animation_tree.unwrap().assume_safe().set_active(true);
        }
        unsafe {
            self.animation_state = self
                .animation_tree
                .unwrap()
                .assume_safe()
                .get("parameters/playback")
                .to_object::<AnimationNodeStateMachinePlayback>();
        }
    }

    #[godot]
    fn _physics_process(&mut self, #[base] owner: &KinematicBody2D, delta: f32) {
        match self.state {
            State::Move => self.move_state(owner, delta),
            State::Roll => self.roll_state(owner),
            State::Attack => self.attack_state(),
        }
    }

    fn move_state(&mut self, owner: &KinematicBody2D, delta: f32) {
        let input = Input::godot_singleton();
        let mut input_vector = Vector2::new(0.0, 0.0);

        input_vector.x = (Input::get_action_strength(input, "ui_right", false)
            - Input::get_action_strength(input, "ui_left", false)) as f32;
        input_vector.y = (Input::get_action_strength(input, "ui_down", false)
            - Input::get_action_strength(input, "ui_up", false)) as f32;
        if input_vector.x != 0.0 || input_vector.y != 0.0 {
            unsafe {
                self.animation_state
                    .as_ref()
                    .unwrap()
                    .assume_safe()
                    .travel("Run");
            }
            input_vector = input_vector.normalized();
            self.roll_vector = input_vector;
            unsafe {
                self.animation_tree
                    .unwrap()
                    .assume_safe()
                    .set("parameters/Idle/blend_position", input_vector);
                self.animation_tree
                    .unwrap()
                    .assume_safe()
                    .set("parameters/Run/blend_position", input_vector);
                self.animation_tree
                    .unwrap()
                    .assume_safe()
                    .set("parameters/Attack/blend_position", input_vector);
                self.animation_tree
                    .unwrap()
                    .assume_safe()
                    .set("parameters/Roll/blend_position", input_vector);
            }
            self.velocity = self
                .velocity
                .move_toward(input_vector * MAX_SPEED, ACCELERATION * delta);
        } else {
            unsafe {
                self.animation_state
                    .as_ref()
                    .unwrap()
                    .assume_safe()
                    .travel("Idle");
            }
            self.velocity = self.velocity.move_toward(Vector2::ZERO, FRICTION * delta);
        }
        self.velocity =
            owner.move_and_slide(self.velocity, Vector2::ZERO, false, 4, 0.785398, true);
        if input.is_action_just_pressed("attack", false) {
            self.state = State::Attack;
        }
        if input.is_action_just_pressed("roll", false) {
            self.state = State::Roll;
        }
    }

    fn roll_state(&mut self, owner: &KinematicBody2D) {
        self.velocity = self.roll_vector * ROLL_SPEED;
        unsafe {
            self.animation_state
                .as_ref()
                .unwrap()
                .assume_safe()
                .travel("Roll");
        }
        self.velocity =
            owner.move_and_slide(self.velocity, Vector2::ZERO, false, 4, 0.785398, true);
    }

    fn attack_state(&mut self) {
        self.velocity = Vector2::ZERO;
        unsafe {
            self.animation_state
                .as_ref()
                .unwrap()
                .assume_safe()
                .travel("Attack");
        }
    }

    #[godot]
    pub fn attack_animation_finished(&mut self) {
        self.state = State::Move;
    }

    #[godot]
    pub fn roll_animation_finished(&mut self) {
        self.velocity *= 0.8;
        self.state = State::Move;
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<Player>();
}

godot_init!(init);
