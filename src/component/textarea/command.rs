use tuirealm::command::Cmd;

pub const NEWLINE: Cmd = Cmd::Custom("0");
pub const DEL_LINE_BY_END: Cmd = Cmd::Custom("1");
pub const DEL_LINE_BY_HEAD: Cmd = Cmd::Custom("2");
pub const DEL_WORD: Cmd = Cmd::Custom("3");
pub const DEL_NEXT_WORD: Cmd = Cmd::Custom("4");
pub const MOVE_WORD_FORWARD: Cmd = Cmd::Custom("5");
pub const MOVE_WORD_BACK: Cmd = Cmd::Custom("6");
pub const MOVE_PARAGRAPH_FORWARD: Cmd = Cmd::Custom("7");
pub const MOVE_PARAGRAPH_BACK: Cmd = Cmd::Custom("8");
pub const MOVE_TOP: Cmd = Cmd::Custom("9");
pub const MOVE_BOTTOM: Cmd = Cmd::Custom("a");
pub const UNDO: Cmd = Cmd::Custom("b");
pub const REDO: Cmd = Cmd::Custom("c");
pub const PASTE: Cmd = Cmd::Custom("d");
