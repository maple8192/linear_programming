use std::io;

use problem::Problem;
use simplex_table::SimplexTable;

mod util;
mod problem;
mod simplex_table;

fn main() -> anyhow::Result<()> {
    let problem = Problem::read_from_input(io::stdin().lock())?;
    println!("{problem}");

    println!("## Simplex Table");
    println!();
    let mut simplex = SimplexTable::from(problem);
    println!("{simplex}");

    while {
        let end = simplex.step();
        println!();
        println!("{simplex}");
        !end
    } {}

    Ok(())
}
