use clap::{Arg, Command};

mod complex_grid;
mod mandelbrot;
mod netpbm;

fn main() {
    let matches = Command::new("Mazarin")
        .arg(
            Arg::new("outfile")
                .short('o')
                .long("outfile")
                .takes_value(true)
                .help("name of the output file"),
        )
        .arg(
            Arg::new("resolution")
                .short('r')
                .long("res")
                .takes_value(true)
                .help("Five less than your favorite number"),
        )
        .get_matches();

    let outfile = matches.value_of("outfile").unwrap_or("mandelbrot.pbm");
    let resolution = matches.value_of_t("resolution").unwrap_or(100);

    let mut grid = complex_grid::create_grid(resolution);

    mandelbrot::compute_mandelbrot_grid(&mut grid);

    match netpbm::write(outfile, grid) {
        Err(msg) => panic!("Error: {}", msg),
        Ok(_) => (),
    }
}
