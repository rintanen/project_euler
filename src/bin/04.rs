use num_integer::div_rem;

fn find_next_palindrome(num: u32) -> u32 {
    let zeros_at_middle = num
        .to_string()
        .chars()
        .skip(1)
        .take(4)
        .map(|c| { 
            match c {
                '0' => c,
                _ => '1'

            }
        })
        .collect::  <String>();
        
    match zeros_at_middle.as_str() {
        "1111" => num - 1100,
        "1001" => num - 110,
        "0110" => num - 1100,
        "0000" => num - 11,
        _ => 0,
    }
}


fn solve() -> String {
    // 999 * 998 = 998002 => 996699 is the biggest palindrome smaller than that
    let mut target: u32 = 996699;
    // 100*100 = 10000 => smallest product of 3digit numbers
    while target >= 10000 {
        // divide with 3 digit numbers starting from 999 
        for divider in (100..=999).rev() {
            let (div, rem) = div_rem(target, divider);
            // if division is even and the value is 3 digits we found solution
            if rem == 0 && (div >= 100 && div <= 999) {
                return String::from(format!("{} * {} = {}", div, divider, target));
            }
        }
        target = find_next_palindrome(target);
    }
    String::from("Didn't find solution")
}


fn main() {
    println!("{}", solve());
}
