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
    RotateShapePX,
    RotateShapePY,
    RotateShapeNX,
    RotateShapeNY,
}

impl Action {
    pub fn bitset(&self) -> u16 {
        match self {
            Action::CameraMovePX =>  0b0000_0000_0000_0001,
            Action::CameraMovePY =>  0b0000_0000_0000_0010,
            Action::CameraMovePZ =>  0b0000_0000_0000_0100,
            Action::CameraMoveNX =>  0b0000_0000_0001_0000,
            Action::CameraMoveNY =>  0b0000_0000_0010_0000,
            Action::CameraMoveNZ =>  0b0000_0000_0100_0000,
            Action::RotateShapePX => 0b0000_0001_0000_0000,
            Action::RotateShapePY => 0b0000_0010_0000_0000,
            Action::RotateShapeNX => 0b0000_0100_0000_0000,
            Action::RotateShapeNY => 0b0000_1000_0000_0000,
        }
    }

    pub fn bitmask(&self) -> u16 {
        match self {
            Action::CameraMovePX =>  0b1111_1111_1111_1110,
            Action::CameraMovePY =>  0b1111_1111_1111_1101,
            Action::CameraMovePZ =>  0b1111_1111_1111_1011,
            Action::CameraMoveNX =>  0b1111_1111_1110_1111,
            Action::CameraMoveNY =>  0b1111_1111_1101_1111,
            Action::CameraMoveNZ =>  0b1111_1111_1011_1111,
            Action::RotateShapePX => 0b1111_1110_1111_1111,
            Action::RotateShapePY => 0b1111_1101_1111_1111,
            Action::RotateShapeNX => 0b1111_1011_1111_1111,
            Action::RotateShapeNY => 0b1111_0111_1111_1111,
        }
    }
}

pub trait ActionState {
    fn on(&mut self, action: Action);
    fn off(&mut self, action: Action);
    fn camera_increment(&self, increment: f32) -> Camera;
}

#[derive(Debug, Copy, Clone)]
pub struct EnumActionState {
    emap: EnumMap<Action, bool>,
}

impl ActionState for EnumActionState{
    fn on(&mut self, action: Action) {
        self.emap[action] = true;
    }

    fn off(&mut self, action: Action) {
        self.emap[action] = false;
    }

    fn camera_increment(&self, increment: f32) -> Camera {
        self.emap
            .iter()
            .fold(Camera::zero(), |mut c, (a, s)| -> Camera {
                match (a, s) {
                    (Action::CameraMovePX, true) => c.x = increment,
                    (Action::CameraMoveNX, true) => c.x = increment.neg(),
                    (Action::CameraMovePY, true) => c.y = increment,
                    (Action::CameraMoveNY, true) => c.y = increment.neg(),
                    (Action::CameraMovePZ, true) => c.z = increment,
                    (Action::CameraMoveNZ, true) => c.z = increment.neg(),
                    _ => (),
                }
                c
            })
    }
}

impl Default for EnumActionState {
    fn default() -> Self {
        let mut emap = EnumMap::new();
        emap.iter_mut()
            .for_each(|(_, s)| *s = false);
        
        EnumActionState { emap }
    }
}

impl ActionState for u16 {
    fn on(&mut self, action: Action) {
        *self |= action.bitset();
    }

    fn off(&mut self, action: Action) {
        *self &= action.bitmask();
    }

    fn camera_increment(&self, increment: f32) -> Camera {
        let mut camera = Camera::zero();

        if *self & Action::CameraMovePX.bitset() > 0 { camera.x = increment; }
        if *self & Action::CameraMoveNX.bitset() > 0 { camera.x = increment.neg(); }
        if *self & Action::CameraMovePY.bitset() > 0 { camera.y = increment; }
        if *self & Action::CameraMoveNY.bitset() > 0 { camera.y = increment.neg(); }
        if *self & Action::CameraMovePZ.bitset() > 0 { camera.z = increment; }
        if *self & Action::CameraMoveNZ.bitset() > 0 { camera.z = increment.neg(); }

        camera
    }
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
        bindings.bind(VirtualKeyCode::W, Action::CameraMoveNY);
        bindings.bind(VirtualKeyCode::S, Action::CameraMovePY);
        bindings.bind(VirtualKeyCode::A, Action::CameraMovePX);
        bindings.bind(VirtualKeyCode::D, Action::CameraMoveNX);

        bindings
    }
}

pub fn handle_keyboard<T: ActionState>(
    event: &KeyboardInput, bindings: &Bindings, state: &mut T,
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
            state.camera_increment(ci)
        })
}

