fn main() {
    /*
    Find the difference between the sum of the squares
    of the first one hundred natural numbers and the square of the sum
     */
    let n = 100;
    let sum: u32 = (n * n + n) / 2;
    let sum_of_squares: u32 = n * (n + 1) * (2 * n + 1) / 6;
    let diff = sum * sum - sum_of_squares;
    println!("{diff}");
}