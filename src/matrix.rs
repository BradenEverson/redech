use std::fmt::Display;

#[derive(Debug)]
pub struct Matrix<const ROWS: usize, const COLS: usize> {
    data: [[f64; COLS]; ROWS],
}

impl<const ROWS: usize, const COLS: usize> From<[[f64; COLS]; ROWS]> for Matrix<ROWS, COLS> {
    fn from(value: [[f64; COLS]; ROWS]) -> Self {
        Self::new(value)
    }
}

impl<const ROWS: usize, const COLS: usize> Display for Matrix<ROWS, COLS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for item in row {
                let item = if *item == -0.0 {
                    0.0
                } else {
                    *item
                };

                write!(f, "\t{:?}", item)?
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl<const ROWS: usize, const COLS: usize> Matrix<ROWS, COLS> {
    pub fn new(data: [[f64; COLS]; ROWS]) -> Matrix<ROWS, COLS> {
        Matrix { data }
    }

    pub fn reduced_echelon_form(&mut self) {
        let mut lead = 0;
        for r in 0..ROWS {
            if lead >= COLS {
                break;
            }

            let mut i = r;
            while self.data[i][lead] == 0.0 {
                i += 1;
                if i == ROWS {
                    i = r;
                    lead += 1;
                    if lead == COLS {
                        return;
                    }
                }
            }

            if i != r {
                println!("Swapping row {} with row {}\n", i + 1, r + 1);
                self.data.swap(i, r);
                println!("{}", self);
            }

            let lead_value = self.data[r][lead];
            if lead_value != 0.0 {
                println!("Scaling row {} to make leading value 1\n", r + 1);
                for j in 0..ROWS {
                    self.data[r][j] /= lead_value;
                }
                println!("{}", self);
            }

            for i in 0..ROWS {
                if i != r {
                    let lead_factor = self.data[i][lead];
                    if lead_factor != 0.0 {
                        println!(
                            "Subtracting row {} {} times from row {}\n",
                            r + 1, lead_factor, i + 1
                            );
                        for j in 0..COLS {
                            self.data[i][j] -= lead_factor * self.data[r][j];
                        }
                        println!("{}", self);
                    }
                }
            }

            lead += 1;
        }
    }
}
