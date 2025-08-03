pub fn matrix_determinant(m: [[isize; 3]; 3]) -> isize {
    let [a, b, c] = m[0];
    let [d, e, f] = m[1];
    let [g, h, i] = m[2];

    a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)
}
