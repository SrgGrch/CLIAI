pub static mut VERBOSE: bool = false;

pub fn set_verbose(verbose: bool) {
    unsafe {
        VERBOSE = verbose;
    }
}

pub fn log_verbose(message: &str) {
    unsafe {
        if VERBOSE {
            println!("{}", message);
        }
    }
}
