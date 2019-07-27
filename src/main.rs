fn main() {
    println!("Answer 1: {}", problem_1(3, 5, 1000));
    println!("Answer 2: {}", problem_2(4000000));
    println!("Answer 3: {}", problem_3(600851475143));
    println!("Answer 4: {}", problem_4(3));
}

fn problem_1(a: i32, b: i32, max_val: i32) -> i32   {
    let mut o = 0;

    for i in 0..max_val {
        if (i%a == 0) | (i%b == 0) {
            println!("{}", i);
            o += i;
        }
    }

    return o
}

fn problem_2(max_val: i32) -> i32 {
    let mut f = 2;
    let mut f_1 = 1;
    let mut o = 0;
    while f <= max_val {
        if f%2 == 0 {
            o += f;
        }

        f = f + f_1;
        f_1 = f - f_1;
    }

    return o
}

fn problem_3(x: i64) -> i64{
    //find smallest prime factor
    let mut x_hold = x;
    let mut f = 0;
    while f < x_hold {
        f = get_smallest_prime_factor(x_hold);
        x_hold = x_hold/f;
    }

    return f
}

fn get_smallest_prime_factor(x: i64) -> i64 {
    let mut f = 2;
    while x % f != 0 {
        f += 1;
    }

    return f
}

fn problem_4(num_digits: u32) -> i32{
    let base: i32 = 10;
    let mut lp = 0;

    for i in base.pow(num_digits - 1)..(base.pow(num_digits)-1) {
        for j in base.pow(num_digits - 1)..(base.pow(num_digits)-1) {
            if is_number_palindrome(i * j) & ((i*j) > lp){
                lp = i*j
            }
        }
    }

    return lp
}

fn is_number_palindrome(x: i32) -> bool {
    let s = x.to_string();
    let nchar = s.chars().count();
    let s1 = &s[..(nchar/2)];
    let s2 = &s[(nchar-nchar/2)..(nchar)].chars().rev().collect::<String>();
    return s1 == s2
}