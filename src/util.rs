macro_rules! or_user_error {
    ($e:expr, $msg:expr, $($args:expr),*) => {
        $e.map_err(|_| UserError { message: format!($msg, $($args),*) })?;
    };

    ($e:expr, $msg:expr) => {
        or_user_error!($e, $msg,)
    }
}

macro_rules! ok_or_user_error {
    ($e:expr, $msg:expr, $($args:expr),*) => {
        $e.ok_or(UserError { message: format!($msg, $($args),*) })?;
    };

    ($e:expr, $msg:expr) => {
        ok_or_user_error!($e, $msg,)
    }
}

pub fn parse_key_name(key: &str) -> Option<usize> {
    match key {
        "ESC" => Some(0),
        "GRAVE" => Some(1),
        "TAB" => Some(2),
        "CAPSLOCK" => Some(3),
        "LEFTSHIFT" => Some(4),
        "LEFTCTRL" => Some(5),

        "1" => Some(6),
        "Q" => Some(7),
        "A" => Some(8),
        "LEFTMETA" => Some(10),

        "F1" => Some(11),
        "2" => Some(12),
        "W" => Some(13),
        "S" => Some(14),
        "Z" => Some(15),
        "LEFTALT" => Some(16),

        "F2" => Some(17),
        "3" => Some(18),
        "E" => Some(19),
        "D" => Some(20),
        "X" => Some(21),

        "F3" => Some(23),
        "4" => Some(24),
        "R" => Some(25),
        "F" => Some(26),
        "C" => Some(27),

        "F4" => Some(28),
        "5" => Some(29),
        "T" => Some(30),
        "G" => Some(31),
        "V" => Some(32),

        "6" => Some(33),
        "Y" => Some(34),
        "H" => Some(35),
        "B" => Some(36),
        "SPACE" => Some(37),

        "F5" => Some(48),
        "7" => Some(49),
        "U" => Some(50),
        "J" => Some(51),
        "N" => Some(52),

        "F6" => Some(53),
        "8" => Some(54),
        "I" => Some(55),
        "K" => Some(56),
        "M" => Some(57),

        "F7" => Some(59),
        "9" => Some(60),
        "O" => Some(61),
        "L" => Some(62),
        "COMMA" => Some(63),

        "F8" => Some(65),
        "0" => Some(66),
        "P" => Some(67),
        "SEMICOLON" => Some(68),
        "DOT" => Some(69),
        "RIGHTALT" => Some(70),

        "MINUS" => Some(72),
        "LEFTBRACE" => Some(73),
        "APOSTROPHE" => Some(74),
        "SLASH" => Some(75),
        "FN" => Some(76),
        "RIGHTMETA" => Some(76),

        "F9" => Some(78),
        "EQUAL" => Some(79),
        "RIGHTBRACE" => Some(80),
        "BACKSLASH" => Some(81),
        "RIGHTSHIFT" => Some(82),
        "COMPOSE" => Some(83),

        "F10" => Some(84),
        "F11" => Some(85),
        "F12" => Some(86),
        "BACKSPACE" => Some(87),
        "ENTER" => Some(88),
        "RIGHTCTRL" => Some(89),

        "SYSRQ" => Some(99),
        "INSERT" => Some(100),
        "DELETE" => Some(101),
        "LEFT" => Some(102),

        "SCROLLLOCK" => Some(103),
        "HOME" => Some(104),
        "END" => Some(105),
        "UP" => Some(106),
        "DOWN" => Some(107),

        "PAUSE" => Some(108),
        "PAGEUP" => Some(109),
        "PAGEDOWN" => Some(110),
        "RIGHT" => Some(111),

        "NUMLOCK" => Some(113),
        "KP7" => Some(114),
        "KP4" => Some(115),
        "KP1" => Some(116),
        "KP0" => Some(117),

        "KPSLASH" => Some(119),
        "KP8" => Some(120),
        "KP5" => Some(121),
        "KP2" => Some(122),

        "KPASTERISK" => Some(124),
        "KP9" => Some(125),
        "KP6" => Some(126),
        "KP3" => Some(127),
        "KPDOT" => Some(128),

        "KPMINUS" => Some(129),
        "KPPLUS" => Some(130),
        "KPENTER" => Some(131),

        _ => None,
    }
}
