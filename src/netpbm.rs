use std::fs::File;
use std::io::Write;

use crate::complex_grid;

pub fn write(outfile: &str, grid: complex_grid::ComplexGrid) -> Result<(), std::io::Error> {
    let mut f = File::create(outfile)?;
    write!(f, "P4\n{} {}\n", grid.cols, grid.rows)?;
    f.write_all(&grid.data)?;
    return Ok(());
}
