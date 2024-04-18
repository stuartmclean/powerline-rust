use std::marker::PhantomData;

use super::Module;
use crate::{Color, Powerline, Style};

pub struct Cmd<S: CmdScheme> {
    scheme: PhantomData<S>,
}

pub trait CmdScheme {
    const CMD_PASSED_FG: Color;
    const CMD_PASSED_BG: Color;
    const CMD_FAILED_BG: Color;
    const CMD_FAILED_FG: Color;
    const CMD_ROOT_SYMBOL: &'static str = "\n#";
    const CMD_USER_SYMBOL: &'static str = "\n$";
}

impl<S: CmdScheme> Cmd<S> {
    pub fn new() -> Cmd<S> {
        Cmd { scheme: PhantomData }
    }
}

impl<S: CmdScheme> Module for Cmd<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        let special = if users::get_current_uid() == 0 { S::CMD_ROOT_SYMBOL } else { S::CMD_USER_SYMBOL };
        powerline.add_short_segment(special, Style::simple(S::CMD_PASSED_FG, S::CMD_PASSED_BG));
    }
}
