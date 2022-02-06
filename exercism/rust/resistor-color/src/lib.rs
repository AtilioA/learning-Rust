use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[derive(Debug, IntoEnumIterator, PartialEq, Clone, Copy, IntEnum)]
#[repr(usize)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(color: ResistorColor) -> usize {
    match color {
        ResistorColor::Black => return 0,
        ResistorColor::Brown => return 1,
        ResistorColor::Red => return 2,
        ResistorColor::Orange => return 3,
        ResistorColor::Yellow => return 4,
        ResistorColor::Green => return 5,
        ResistorColor::Blue => return 6,
        ResistorColor::Violet => return 7,
        ResistorColor::Grey => return 8,
        ResistorColor::White => return 9,
    }
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Ok(color) => format!("{:?}", color),
        Err(_) => "value out of range".to_string(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    return ResistorColor::into_enum_iter().collect();
}
