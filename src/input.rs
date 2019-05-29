//! Input processing. Using the command pattern but instead of returning an `action`, will
//! return a transform to be applied.
use std::collections::HashMap;
use std::ops::Neg;

use enum_map::{Enum, EnumMap};
use wgpu::winit::{KeyboardInput, VirtualKeyCode, ElementState};
use cgmath::{Vector3, Zero};

pub type Camera = Vector3<f32>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Enum)]
pub enum Action {
    CameraMovePX,
    CameraMovePY,
    CameraMovePZ,
    CameraMoveNX,
    CameraMoveNY,
    CameraMoveNZ,
}

#[derive(Debug, Copy, Clone)]
pub struct ActionState {
    emap: EnumMap<Action, bool>,
}

impl ActionState {
    fn on(&mut self, action: Action) {
        self.emap[action] = true;
    }

    fn off(&mut self, action: Action) {
        self.emap[action] = false;
    }
}

impl Default for ActionState {
    fn default() -> Self {
        let mut emap = EnumMap::new();
        emap.iter_mut()
            .for_each(|(_, s)| *s = false);
        
        ActionState { emap }
    }
}

fn compute_camera(state: &ActionState, increment: f32) -> Camera {
    state.emap
        .iter()
        .fold(Camera::zero(), |mut v, (a, s)| -> Camera {
            match (a, s) {
                (Action::CameraMovePX, true) => v.x = increment,
                (Action::CameraMoveNX, true) => v.x = increment.neg(),
                (Action::CameraMovePY, true) => v.y = increment,
                (Action::CameraMoveNY, true) => v.y = increment.neg(),
                (Action::CameraMovePZ, true) => v.z = increment,
                (Action::CameraMoveNZ, true) => v.z = increment.neg(),
                _ => (),
            }
            v
        })
}

/// Which keypresses carry out which which actions and by how much.
pub struct Bindings {
    bindings: HashMap<VirtualKeyCode, Action>,
    camera_increment: f32,
}

impl Bindings {
    pub fn new(camera_increment: f32) -> Self {
        Bindings {
            bindings: HashMap::new(),
            camera_increment,
        }
    }

    pub fn bind(&mut self, vkc: VirtualKeyCode, action: Action) -> Option<Action> {
        self.bindings.insert(vkc, action)
    }

    pub fn unbind(&mut self, vkc: &VirtualKeyCode) -> Option<Action> {
        self.bindings.remove(vkc)
    }
}

impl Default for Bindings {
    fn default() -> Self {
        let mut bindings = Bindings::new(0.1f32);
        bindings.bind(VirtualKeyCode::Up, Action::CameraMoveNY);
        bindings.bind(VirtualKeyCode::Down, Action::CameraMovePY);
        bindings.bind(VirtualKeyCode::Left, Action::CameraMovePX);
        bindings.bind(VirtualKeyCode::Right, Action::CameraMoveNX);

        bindings
    }
}

pub fn handle_keyboard(
    event: &KeyboardInput, bindings: &Bindings, state: &mut ActionState,
) -> Option<Camera> {
    let ci = bindings.camera_increment;
    let vkc = event.virtual_keycode
        .unwrap_or(VirtualKeyCode::Escape); // Escape is already caught beforehand.

    bindings.bindings
        .get(&vkc)
        .map(|action| {
            match event.state {
                ElementState::Pressed => state.on(*action),
                ElementState::Released => state.off(*action),
            }
            compute_camera(state, ci)
        })
}

