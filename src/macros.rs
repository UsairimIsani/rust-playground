use std::io::stdin;
// A convenient macro to read input as string into a buffer
#[macro_export]
macro_rules! scanline {
    ($x:expr) => {{
        stdin().read_line(&mut $x).unwrap();
        $x.trim();
    }};
}
