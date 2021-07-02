struct ExtMatrix {
    first_row: usize,
    mtx: [[i64; 3]; 3],
}

impl ExtMatrix {
    fn new(val1: i64, val2: i64) -> ExtMatrix {
        let mtx = [[val1, 1, 0], [val2, 0, 1], [0, 0, 0]];
        ExtMatrix { first_row: 0, mtx }
    }

    fn step(&mut self) {
        let next_row: usize = (self.first_row + 1) % 3;
        let spare_row: usize = (self.first_row + 2) % 3;
        let q = self.mtx[self.first_row][0] / self.mtx[next_row][0];
        for i in 0..3 {
            self.mtx[spare_row][i] = self.mtx[self.first_row][i] - q * self.mtx[next_row][i];
        }
        self.first_row = (self.first_row + 1) % 3;
    }

    fn finished(&self) -> bool {
        self.mtx[(self.first_row + 1) % 3][0] == 0
    }

    fn result(&self) -> (i64, i64, i64) {
        let row = &self.mtx[self.first_row];
        (row[0], row[1], row[2])
    }
}

pub fn calc_euclidean_ext(val1: i64, val2: i64) -> (i64, i64, i64) {
    let mut mtx = ExtMatrix::new(val1, val2);
    while !mtx.finished() {
        mtx.step();
    }
    mtx.result()
}
