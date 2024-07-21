#[macro_export]
macro_rules! put {
    ($($arg:tt)*) => {
        println!("\x1b[90m[\x1b[35mPOLL\x1b[90m]\x1b[0m {}", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! delete {
    ($($arg:tt)*) => {
        println!("\x1b[90m[\x1b[31mCLOSE\x1b[90m]\x1b[0m {}", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! patch {
    ($($arg:tt)*) => {
        println!("\x1b[90m[\x1b[33mSYNC\x1b[90m]\x1b[0m {}", format_args!($($arg)*));
    };
}