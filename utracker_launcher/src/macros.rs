#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        #[allow(clippy::print_stdout, clippy::print_stderr)]
        if *$crate::consts::HAS_ANSI {
            _ = eprintln!("\x1b[31;4mERROR\x1b[0m: {}", format!($($arg)*));
        }
        else {
            _ = eprintln!("ERROR: {}", format!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        #[allow(clippy::print_stdout, clippy::print_stderr)]
        if *$crate::consts::HAS_ANSI {
            _ = eprintln!("\x1b[33;4mWARN\x1b[0m: {}", format!($($arg)*));
        }
        else {
            _ = eprintln!("WARN: {}", format!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        #[allow(clippy::print_stdout, clippy::print_stderr)]
        if *$crate::consts::HAS_ANSI {
            _ = eprintln!("\x1b[34;4mINFO\x1b[0m: {}", format!($($arg)*));
        }
        else {
            _ = eprintln!("INFO: {}", format!($($arg)*));
        }
    };
}
