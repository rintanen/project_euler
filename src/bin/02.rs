fn main() {
    let mut prev = 1;
    let mut cur =  1;
    let mut sum: u32 = 0;

    loop {
        cur = prev + cur;
        prev = cur - prev;

        if cur >= 4_000_000 {
            break;
        }
        
        if cur % 2 == 0 {
            sum += cur;
        }
    }
    println!("{}", sum);
}