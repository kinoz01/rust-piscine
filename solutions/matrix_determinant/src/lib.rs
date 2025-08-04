pub fn matrix_determinant(m: [[isize; 3]; 3]) -> isize {
    let [a, b, c] = m[0];
    let [d, e, f] = m[1];
    let [g, h, i] = m[2];
    a * (e * i - h * f) - b * (d * i - g * f) + c * (d * h - g * e)
}
