mod generic_function;
// use generic_function::give_me;

macro_rules! scanline {
    ($e:expr) => {{
        use std::io::Write;
        let mut stdout = std::io::stdout();
        stdout.write_all($e.as_bytes()).unwrap();
        stdout.flush().unwrap();
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s
    }};
    () => {{
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s
    }};
}

fn main() {
    let a = scanline!("ad");
    println!("Hi. {}", a);
    let a = scanline!();
    println!("Hi. {}", a);
}
