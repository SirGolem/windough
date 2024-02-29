#[macro_export]
macro_rules! printinfo {
    ($($arg:tt)*) => {
        eprintln!("\x1B[1m\x1B[34minfo:\x1B[0m {}", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! printwarning {
    ($($arg:tt)*) => {
        eprintln!("\x1B[1m\x1B[33mwarning:\x1B[0m {}", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! printerror {
    ($($arg:tt)*) => {
        eprintln!("\x1B[1m\x1B[31merror:\x1B[0m {}", format!($($arg)*))
    };
}
