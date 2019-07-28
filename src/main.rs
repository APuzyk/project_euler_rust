use std::cmp;


fn main() {
    println!("Answer 1: {}", problem_1(3, 5, 1000));
    println!("Answer 2: {}", problem_2(4000000));
    println!("Answer 3: {}", problem_3(600851475143));
    println!("Answer 4: {}", problem_4(3));
    println!("Answer 5: {}", problem_5(20));
    println!("Answer 6: {}", problem_6(100));
    println!("Answer 7: {}", problem_7(10001));
    println!("Answer 8: {}", problem_8(13, "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450".to_string()));
}

fn problem_1(a: i32, b: i32, max_val: i32) -> i32   {
    let mut o = 0;

    for i in 0..max_val {
        if (i%a == 0) | (i%b == 0) {
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
            if (i*j) > lp {
                if is_number_palindrome(i * j) {
                    lp = i * j
                }
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

fn problem_5(x: i64) -> i64 {
    let mut o = 1;
    for i in 1..(x+1) {
        o = (o * i)/get_gcd(o, i)
    }
    return o
}

fn get_gcd(a: i64, b: i64) -> i64 {
    let smaller = cmp::min(a, b);

    for i in (1..(smaller+1)).rev() {
        if (a%i == 0) & (b%i == 0) {
            return i;
        }
    }
    return 1
}

fn problem_6(x: i64) -> i64 {
    let mut s1: i64 = 0;
    let mut s2: i64 = 0;
    for i in 1..(x+1) {
        s1 += i.pow(2);
        s2 += i;
    }

    return s2.pow(2) - s1;
}

fn problem_7(x: i64) -> i64 {
    let mut curr_prime = 1;
    let mut o = 2;
    while curr_prime != x {
        o += 1;
        if is_prime(o){
            curr_prime += 1;
        }
    }

    return o
}

fn is_prime(x: i64) -> bool {
    let mut i = 2;
    let mut max_val= (x/2) + 1;
    while i < max_val {
        if x%i == 0 {
            return false
        }
        max_val = (x/i) + 1;
        i += 1;
    }
    return true
}

fn problem_8(num_adjacent: usize, num_as_string: String) -> u64 {
    let num_vec: Vec<u32> = num_as_string.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut o:u64 = 0;
    let mut test:u64 = 1;
    for i in 0..(num_vec.len()-num_adjacent) {
        for i in num_vec[i..(i+(num_adjacent))].iter() {
            test = *i as u64 * test;
        }
        if test > o {
            o = test;
        }
        test = 1;
    }
    return o
}
