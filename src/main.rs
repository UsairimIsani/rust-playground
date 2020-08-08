// mod generic_function;`
// use generic_function::give_me;
#[macro_use]
mod macros;

fn main() {
    // let a = scanline!();
    // println!("Got Input : {}", a);
    let new_map = map! {
        "a" => "b",
        "b" => "c"
    };
    println!("{:#?}", new_map);
}
