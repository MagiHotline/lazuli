use bitos::bitos;

#[bitos(2)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Format {
    #[default]
    U8       = 0b00,
    U16      = 0b01,
    U24      = 0b10,
    Reserved = 0b11,
}

#[bitos(2)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Op {
    #[default]
    Disabled = 0b00,
    Add      = 0b01,
    Replace  = 0b10,
    Reserved = 0b11,
}

#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Mode {
    #[bits(0..2)]
    pub format: Format,
    #[bits(2..4)]
    pub op: Op,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Texture {
    pub mode: Mode,
    pub bias: u32,
}
