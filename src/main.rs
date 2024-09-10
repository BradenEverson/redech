use redech::matrix::Matrix;

fn main() {
    let mut matrix: Matrix<3, 3> =
        [[8.9, -99.0, 12.1], [1.0, 0.4, 11.1], [100.5, 20.0, 1.2]].into();

    println!("Original Matrix:\n\n{}", matrix);

    matrix.reduced_echelon_form();

    println!("Matrix in Reduced Row Echelon Form:\n\n{}", matrix);
}
