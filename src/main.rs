mod doas;
/*******************************************************
    @brief Dedicated OpenBSD Application Subexecutor
*******************************************************/
fn main() {
    // Parse command-line arguments here.
    // You can use libraries like `clap` or manually parse `std::env::args()`.
    let ch: char = 'x';
    match ch {
        'a' => todo!("For argument a"),
        'C' => todo!("For argument C"),
        'u' => todo!("For argument u"),
        'n' => todo!("For argument n"),
        'S' => todo!("For argument S"),
        's' => todo!("For argument s"),
        _ => doas::usage(),
    }

    let regsize = std::mem::size_of::<usize>()* 8;
}