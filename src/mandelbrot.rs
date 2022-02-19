use crate::complex_grid;

pub fn compute_mandelbrot_grid(grid: &mut complex_grid::ComplexGrid) {
    for n in 0..grid.rows {
        let ci = grid.c0_imag + (n as f64) * grid.delta;
        for m in 0..grid.cols {
            let cr = grid.c0_real + (m as f64) * grid.delta;

            let bytepos = m >> 3;
            let bitpos = 7 - m % 8;

            let bit = escapes(cr, ci);

            grid.data[n * grid.row_bytes + bytepos] |= bit << bitpos;
        }
    }
}

fn escapes(cr: f64, ci: f64) -> u8 {
    let mut zr = cr;
    let mut zi = ci;
    let mut tr: f64;
    let mut ti: f64;
    for _ in 0..50 {
        tr = zr * zr - zi * zi + cr;
        ti = 2.0_f64 * zr * zi + ci;
        if tr * tr + ti * ti > 4.0_f64 {
            return 0;
        }
        zr = tr;
        zi = ti;
    }
    return 1;
}
