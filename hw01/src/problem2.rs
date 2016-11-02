// Problem 2

/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let o1 = (mat1.len(), mat1[0].len());
    let o2 = (mat2.len(), mat2[0].len());

    assert_eq!(o1.1, o2.0);

    let mut result = vec![vec![0.; o2.1]; o1.0];

    for i in 0..o1.0 {
        for j in 0..o2.1 {
            for k in 0..o1.1 {
                result[i][j] += mat1[i][k] * mat2[k][j];
            }
        }

    }

    result
}
