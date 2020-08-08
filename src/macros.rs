#[macro_export]
macro_rules! scanline {
    ($e:expr) => {{
        use std::io::Read;
        use std::io::Write;
        let mut stdout = std::io::stdout();
        stdout.write_all($e.as_bytes()).unwrap();
        stdout.flush().unwrap();
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s
    }};
    () => {{
        use std;
        let mut s = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut s).unwrap();
        s
    }};
}
#[macro_export]
macro_rules! map {
    ($( $x:expr => $y:expr ),*) => {{

        let mut map = std::collections::HashMap::new();
        $(
            map.insert($x,$y);
        )*
        map
    }
    };
}
