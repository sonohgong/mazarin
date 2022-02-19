pub struct ComplexGrid {
    pub data: Vec<u8>,
    pub rows: usize,
    pub cols: usize,
    pub row_bytes: usize,
    pub c0_imag: f64,
    pub c0_real: f64,
    pub delta: f64,
}

pub fn create_grid(res: usize) -> ComplexGrid {
    // Define a square area with a starting point c0
    // and the imaginary/real range.
    let c0_imag: f64 = -1.0;
    let c0_real: f64 = -1.5;
    let imag_range: usize = 2;
    let real_range: usize = 2;

    let delta = 1.0_f64 / (res as f64);

    let rows = imag_range * res;
    let cols = real_range * res;
    let row_bytes = cols / 8;

    return ComplexGrid {
        data: vec![0; rows * row_bytes],
        rows,
        cols,
        row_bytes,
        c0_imag,
        c0_real,
        delta,
    };
}
