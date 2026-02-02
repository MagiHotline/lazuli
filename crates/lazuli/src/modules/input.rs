//! Input module interface.

#[derive(Debug, Clone, Copy)]
pub struct ControllerState {
    // Analog
    pub analog_x: u8,
    pub analog_y: u8,
    pub analog_sub_x: u8,
    pub analog_sub_y: u8,

    // Analog Triggers
    pub analog_trigger_left: u8,
    pub analog_trigger_right: u8,

    // Digital Triggers
    pub trigger_z: bool,
    pub trigger_left: bool,
    pub trigger_right: bool,

    // Pad
    pub pad_left: bool,
    pub pad_right: bool,
    pub pad_down: bool,
    pub pad_up: bool,

    // Buttons
    pub button_a: bool,
    pub button_b: bool,
    pub button_x: bool,
    pub button_y: bool,
    pub button_start: bool,
}

impl Default for ControllerState {
    fn default() -> Self {
        Self {
            analog_x: 128,
            analog_y: 128,
            analog_sub_x: 128,
            analog_sub_y: 128,
            analog_trigger_left: Default::default(),
            analog_trigger_right: Default::default(),
            trigger_z: Default::default(),
            trigger_left: Default::default(),
            trigger_right: Default::default(),
            pad_left: Default::default(),
            pad_right: Default::default(),
            pad_down: Default::default(),
            pad_up: Default::default(),
            button_a: Default::default(),
            button_b: Default::default(),
            button_x: Default::default(),
            button_y: Default::default(),
            button_start: Default::default(),
        }
    }
}

/// Trait for controller modules.
pub trait InputModule: Send {
    fn controller(&mut self, index: usize) -> Option<ControllerState>;
}

/// An implementation of [`InputModule`] which does nothing: every controller is always
/// disconnected.
#[derive(Debug, Clone, Copy)]
pub struct NopInputModule;

impl InputModule for NopInputModule {
    fn controller(&mut self, _: usize) -> Option<ControllerState> {
        None
    }
}
