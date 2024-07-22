// mod arrays_slices;
mod debug;
mod display;
mod format_print;
mod formatting;
mod hello_world;
mod operator;
mod tuples;

fn main() {
    print!("--------------------------------------hello_world\n");
    hello_world::main();
    print!("--------------------------------------debug\n");
    debug::main();
    print!("--------------------------------------display\n");
    display::main();
    print!("--------------------------------------format_print\n");
    format_print::main();
    print!("--------------------------------------formatting\n");
    formatting::main();
    print!("--------------------------------------operator\n");
    operator::main();
    print!("--------------------------------------tuples\n");
    tuples::main();
    // print!("--------------------------------------arrays\n");
    // arrays_slices::main();
}
