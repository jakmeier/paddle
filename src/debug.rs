#[cfg(debug_assertions)]
macro_rules! debug_println {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[cfg(not(debug_assertions))]
macro_rules! debug_println {
    ( $( $t:tt )* ) => {};
}
