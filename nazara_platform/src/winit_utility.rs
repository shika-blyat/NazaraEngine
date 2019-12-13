use crate::events::{
    KeyEvent, MouseButton as NazarustMouseButton, MouseEvent, State,
    WindowEvent as NazarustWindowEvent,
};
use std::hash::Hash;
use winit::event::{
    ElementState, Event as WinitEvent, KeyboardInput, ModifiersState as WinitModifiersState,
    MouseButton as WinitMouseButton, VirtualKeyCode, WindowEvent,
};
pub trait NazarustEvent: Eq + Hash {}
impl NazarustEvent for KeyEvent {}
impl NazarustEvent for MouseEvent {}
impl NazarustEvent for NazarustWindowEvent {}

pub enum NazarustEvents {
    KeyEvent(KeyEvent, NazarustModifiersState),
    MouseEvent(MouseEvent, Option<NazarustModifiersState>),
    WindowEvent(NazarustWindowEvent),
    Unknown,
}
/*pub fn nazarustevents_to_event(nazarust_event: NazarustEvents) -> Option<impl NazarustEvent>{
    if let NazarustEvents::KeyEvent(key_event, _) = nazarust_event{
       return  Some(key_event)
    };
    if let NazarustEvents::MouseEvent(mouse_event, _) = nazarust_event {
        return Some(mouse_event)
    };
    if let NazarustEvents::WindowEvent(window_event) = nazarust_event {
        return Some(window_event)
    } else {
        return None
    }
}*/
fn winit_to_nazarust_modifiers(modifiers: WinitModifiersState) -> NazarustModifiersState {
    NazarustModifiersState {
        shift: modifiers.shift,
        ctrl: modifiers.ctrl,
        alt: modifiers.alt,
        logo: modifiers.logo,
    }
}
pub struct NazarustModifiersState {
    shift: bool,
    ctrl: bool,
    alt: bool,
    logo: bool,
}
pub fn from_winit_event<K>(winit_event: WinitEvent<K>) -> NazarustEvents {
    let nazarust_event = match winit_event {
        WinitEvent::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(_) => NazarustEvents::WindowEvent(NazarustWindowEvent::Resized),
            WindowEvent::Moved(_) => NazarustEvents::WindowEvent(NazarustWindowEvent::Moved),
            WindowEvent::CloseRequested => {
                NazarustEvents::WindowEvent(NazarustWindowEvent::CloseRequested)
            }
            WindowEvent::CursorMoved { .. } => NazarustEvents::MouseEvent(MouseEvent::Moved, None),
            WindowEvent::MouseInput {
                state,
                button,
                modifiers,
                ..
            } => {
                let nazarust_modifiers = winit_to_nazarust_modifiers(modifiers);
                match (button, state) {
                    (WinitMouseButton::Left, ElementState::Pressed) => NazarustEvents::MouseEvent(
                        MouseEvent::Button {
                            mouse_button: NazarustMouseButton::Left,
                            state: State::Pressed,
                        },
                        Some(nazarust_modifiers),
                    ),
                    (WinitMouseButton::Left, ElementState::Released) => NazarustEvents::MouseEvent(
                        MouseEvent::Button {
                            mouse_button: NazarustMouseButton::Left,
                            state: State::Released,
                        },
                        Some(nazarust_modifiers),
                    ),
                    (WinitMouseButton::Right, ElementState::Pressed) => NazarustEvents::MouseEvent(
                        MouseEvent::Button {
                            mouse_button: NazarustMouseButton::Right,
                            state: State::Pressed,
                        },
                        Some(nazarust_modifiers),
                    ),
                    (WinitMouseButton::Right, ElementState::Released) => {
                        NazarustEvents::MouseEvent(
                            MouseEvent::Button {
                                mouse_button: NazarustMouseButton::Right,
                                state: State::Released,
                            },
                            Some(nazarust_modifiers),
                        )
                    }
                    (WinitMouseButton::Middle, ElementState::Pressed) => {
                        NazarustEvents::MouseEvent(
                            MouseEvent::Button {
                                mouse_button: NazarustMouseButton::Middle,
                                state: State::Pressed,
                            },
                            Some(nazarust_modifiers),
                        )
                    }
                    (WinitMouseButton::Middle, ElementState::Released) => {
                        NazarustEvents::MouseEvent(
                            MouseEvent::Button {
                                mouse_button: NazarustMouseButton::Middle,
                                state: State::Released,
                            },
                            Some(nazarust_modifiers),
                        )
                    }
                    (WinitMouseButton::Other(n), ElementState::Pressed) => {
                        NazarustEvents::MouseEvent(
                            MouseEvent::Button {
                                mouse_button: NazarustMouseButton::Other(n),
                                state: State::Pressed,
                            },
                            Some(nazarust_modifiers),
                        )
                    }
                    (WinitMouseButton::Other(n), ElementState::Released) => {
                        NazarustEvents::MouseEvent(
                            MouseEvent::Button {
                                mouse_button: NazarustMouseButton::Other(n),
                                state: State::Released,
                            },
                            Some(nazarust_modifiers),
                        )
                    }
                }
            }
            WindowEvent::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    virtual_keycode,
                    state,
                    modifiers,
                    ..
                } => {
                    let nazarust_modifiers = winit_to_nazarust_modifiers(modifiers);
                    match (virtual_keycode, state) {
                        (Some(VirtualKeyCode::A), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::A {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::B), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::B {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::C), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::C {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::D), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::D {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::E), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::E {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::G), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::G {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::H), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::H {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::I), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::I {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::J), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::J {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::K), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::K {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::L), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::L {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::M), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::M {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::N), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::N {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::O), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::O {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::P), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::P {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Q), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Q {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::R), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::R {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::S), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::S {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::T), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::T {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::U), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::U {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::V), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::V {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::W), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::W {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::X), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::X {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Y), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Y {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Z), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Z {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Key1), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Key1 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Key2), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Key2 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Key3), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Key3 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Key4), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Key4 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Key5), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Key5 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Key6), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Key6 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Key7), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Key7 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Key8), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Key8 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Key9), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Key9 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Escape {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F1), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F1 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F2), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F2 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F3), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F3 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F4), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F4 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F5), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F5 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F6), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F6 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F7), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F7 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F8), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F8 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F9), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F9 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F10), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F10 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F11), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F11 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F12), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F12 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F13), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F13 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F14), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F14 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F15), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F15 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F16), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F16 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F17), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F17 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F18), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F18 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F19), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F19 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F20), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F20 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F21), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F21 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F22), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F22 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F23), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F23 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F24), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F24 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Snapshot), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Snapshot {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Scroll), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::ScrollLock {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Pause), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Pause {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Insert), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Insert {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Home), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Home {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Delete), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Delete {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::End), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::End {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::PageDown), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::PageDown {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::PageUp), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::PageUp {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Left), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Left {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Up), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Up {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Right), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Right {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Down), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Down {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Back), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Back {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Return), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Enter {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Space), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Space {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Compose), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Compose {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Caret), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Caret {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Numlock), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Numlock {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Numpad0), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Numpad0 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Numpad1), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Numpad1 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Numpad2), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Numpad2 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Numpad3), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Numpad3 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Numpad4), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Numpad4 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Numpad5), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Numpad5 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Numpad6), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Numpad6 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Numpad7), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Numpad7 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Numpad8), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Numpad8 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Numpad9), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Numpad9 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::AbntC1), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::AbntC1 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::AbntC2), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::AbntC2 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Add), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Add {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Apostrophe), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Apostrophe {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Apps), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Apps {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::At), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::At {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Ax), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Ax {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Backslash), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Backslash {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Calculator), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Calculator {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Capital), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Capital {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Colon), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Colon {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Comma), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Comma {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Convert), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Convert {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Decimal), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Decimal {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Divide), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Divide {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Equals), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Equals {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Grave), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Grave {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Kana), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Kana {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Kanji), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Kanji {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::LAlt), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::LBracket {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::LControl), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::LShift {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::LWin), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::LWin {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Mail), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Mail {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::MediaSelect), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::MediaSelect {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::MediaStop), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::MediaStop {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Minus), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Minus {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Multiply), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Multiply {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Mute), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Mute {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::MyComputer), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::MyComputer {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::NavigateForward), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::NavigateForward {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::NavigateBackward), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::NavigateBackward {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::NextTrack), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::NextTrack {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::NoConvert), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::NoConvert {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::NumpadComma), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::NumpadComma {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::NumpadEnter), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::NumpadEnter {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::NumpadEquals), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::NumpadEquals {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::OEM102), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::OEM102 {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Period), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Period {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::PlayPause), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::PlayPause {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Power), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Power {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::PrevTrack), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::PrevTrack {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::RAlt), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::RAlt {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::RBracket), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::RBracket {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::RControl), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::RControl {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::RShift), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::RShift {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::RWin), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::RWin {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Semicolon), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Semicolon {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Slash), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Slash {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Sleep), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Sleep {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Stop), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Stop {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Subtract), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Subtract {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Sysrq), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Sysrq {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Tab), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Tab {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Underline), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Underline {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Unlabeled), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Unlabeled {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::VolumeDown), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::VolumeDown {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::VolumeUp), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::VolumeUp {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Wake), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Wake {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::WebBack), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::WebBack {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::WebFavorites), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::WebFavorites {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::WebForward), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::WebForward {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::WebHome), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::WebHome {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::WebRefresh), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::WebRefresh {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::WebSearch), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::WebSearch {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::WebStop), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::WebStop {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Yen), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Yen {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Copy), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Copy {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Paste), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Paste {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Cut), ElementState::Pressed) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Cut {
                                    state: State::Pressed,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::A), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::A {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::B), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::B {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::C), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::C {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::D), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::D {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::E), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::E {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::G), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::G {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::H), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::H {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::I), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::I {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::J), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::J {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::K), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::K {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::L), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::L {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::M), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::M {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::N), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::N {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::O), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::O {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::P), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::P {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Q), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Q {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::R), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::R {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::S), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::S {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::T), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::T {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::U), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::U {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::V), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::V {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::W), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::W {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::X), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::X {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Y), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Y {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Z), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Z {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Key1), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Key1 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Key2), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Key2 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Key3), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Key3 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Key4), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Key4 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Key5), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Key5 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Key6), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Key6 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Key7), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Key7 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Key8), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Key8 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Key9), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Key9 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Escape), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Escape {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F1), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F1 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F2), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F2 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F3), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F3 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F4), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F4 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F5), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F5 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F6), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F6 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F7), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F7 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F8), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F8 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F9), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F9 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F10), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F10 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F11), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F11 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F12), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F12 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F13), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F13 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F14), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F14 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F15), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F15 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F16), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F16 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F17), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F17 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F18), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F18 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F19), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F19 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F20), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F20 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F21), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F21 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F22), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F22 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F23), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F23 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::F24), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::F24 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Snapshot), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Snapshot {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Scroll), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::ScrollLock {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Pause), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Pause {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Insert), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Insert {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Home), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Home {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Delete), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Delete {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::End), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::End {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::PageDown), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::PageDown {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::PageUp), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::PageUp {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Left), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Left {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Up), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Up {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Right), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Right {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Down), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Down {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Back), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Back {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Return), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Enter {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Space), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Space {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Compose), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Compose {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Caret), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Caret {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Numlock), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Numlock {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Numpad0), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Numpad0 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Numpad1), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Numpad1 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Numpad2), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Numpad2 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Numpad3), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Numpad3 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Numpad4), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Numpad4 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Numpad5), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Numpad5 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Numpad6), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Numpad6 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Numpad7), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Numpad7 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Numpad8), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Numpad8 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Numpad9), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Numpad9 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::AbntC1), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::AbntC1 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::AbntC2), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::AbntC2 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Add), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Add {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Apostrophe), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Apostrophe {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Apps), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Apps {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::At), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::At {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Ax), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Ax {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Backslash), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Backslash {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Calculator), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Calculator {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Capital), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Capital {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Colon), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Colon {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Comma), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Comma {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Convert), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Convert {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Decimal), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Decimal {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Divide), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Divide {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Equals), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Equals {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Grave), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Grave {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Kana), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Kana {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Kanji), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Kanji {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::LAlt), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::LBracket {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::LControl), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::LShift {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::LWin), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::LWin {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Mail), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Mail {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::MediaSelect), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::MediaSelect {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::MediaStop), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::MediaStop {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Minus), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Minus {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Multiply), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Multiply {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Mute), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Mute {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::MyComputer), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::MyComputer {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::NavigateForward), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::NavigateForward {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::NavigateBackward), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::NavigateBackward {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::NextTrack), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::NextTrack {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::NoConvert), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::NoConvert {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::NumpadComma), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::NumpadComma {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::NumpadEnter), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::NumpadEnter {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::NumpadEquals), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::NumpadEquals {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::OEM102), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::OEM102 {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Period), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Period {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::PlayPause), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::PlayPause {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Power), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Power {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::PrevTrack), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::PrevTrack {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::RAlt), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::RAlt {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::RBracket), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::RBracket {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::RControl), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::RControl {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::RShift), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::RShift {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::RWin), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::RWin {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Semicolon), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Semicolon {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Slash), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Slash {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Sleep), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Sleep {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Stop), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Stop {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Subtract), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Subtract {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Sysrq), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Sysrq {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Tab), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Tab {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Underline), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Underline {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Unlabeled), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Unlabeled {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::VolumeDown), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::VolumeDown {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::VolumeUp), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::VolumeUp {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Wake), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Wake {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::WebBack), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::WebBack {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::WebFavorites), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::WebFavorites {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::WebForward), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::WebForward {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::WebHome), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::WebHome {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::WebRefresh), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::WebRefresh {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::WebSearch), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::WebSearch {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::WebStop), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::WebStop {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Yen), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Yen {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Copy), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Copy {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Paste), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Paste {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        (Some(VirtualKeyCode::Cut), ElementState::Released) => {
                            NazarustEvents::KeyEvent(
                                KeyEvent::Cut {
                                    state: State::Released,
                                },
                                nazarust_modifiers,
                            )
                        }
                        _ => NazarustEvents::Unknown,
                    }
                }
            },
            _ => NazarustEvents::Unknown,
        },
        _ => NazarustEvents::Unknown,
    };
    nazarust_event
}