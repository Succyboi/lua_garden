use nih_plug::prelude::Enum;

#[derive(Enum, Debug, PartialEq)]
pub enum DataReadMode {
    #[id = "bit1"]
    Bit1,

    #[id = "bit4"]
    Bit4,

    #[id = "bit8"]
    Bit8,
    
    #[id = "bit16"]
    Bit16,
}