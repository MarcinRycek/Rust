struct Matrix {
    height: usize,
    width: usize,
    data: Vec<Vec<f64>>,
}

impl Matrix {
    fn new(height: usize, width: usize, filler: f64) -> Self {
        Self {
            height,
            width,
            data: vec![vec![filler; width]; height],
        }
    }

    fn zeros(height: usize, width: usize) -> Self {
        Self::new(height, width, 0.0)
    }

    fn identity(size: usize) -> Self {
        let mut res = Self::zeros(size, size);
        for i in 0..size {
            res.data[i][i] = 1.0;
        }
        res
    }

    fn display(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                print!("{}, ", self.data[i][j]);
            }
            println!();
        }
    }

    fn get_element(&self, row: usize, col: usize) -> f64 {
        self.data[row][col]
    }

    fn set_element(&mut self, row: usize, col: usize, value: f64) {
        self.data[row][col] = value;
    }

    fn sum(matrix1: &Self, matrix2: &Self) -> Option<Self> {
        if matrix1.height != matrix2.height || matrix1.width != matrix2.width {
            return None;
        }

        let mut res = Self::zeros(matrix1.height, matrix1.width);
        for i in 0..matrix1.height {
            for j in 0..matrix1.width {
                res.data[i][j] = matrix1.data[i][j] + matrix2.data[i][j];
            }
        }
        Some(res)
    }
}

fn main() {
    let ones = Matrix::new(4, 4, 1.0);
    println!("{} {}", ones.height == 4, ones.width == 4);
    ones.display();

    let zeros = Matrix::zeros(4, 4);
    zeros.display();
}
