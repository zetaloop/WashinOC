use crate::drivers::display::Display;
use crate::drivers::motor::Motor;
use crate::drivers::touch::TouchButton;

pub fn main_loop(
    _touch: &mut TouchButton<'_>,
    _display: &mut Display<'_>,
    _motor: &mut Motor<'_>,
) -> ! {
    loop {
        // TODO: state machine tick
    }
}
