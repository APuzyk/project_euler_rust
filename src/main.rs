use std::cmp;


fn main() {
    /*
    println!("Answer 1: {}", problem_1(3, 5, 1000));
    println!("Answer 2: {}", problem_2(4000000));
    println!("Answer 3: {}", problem_3(600851475143));
    println!("Answer 4: {}", problem_4(3));
    println!("Answer 5: {}", problem_5(20));
    println!("Answer 6: {}", problem_6(100));
    println!("Answer 7: {}", problem_7(10001));
    println!("Answer 8: {}", problem_8(13, "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450".to_string()));
    println!("Answer 9: {}", problem_9(1000));
    println!("Answer 10: {}", problem_10(2000000));
    println!("Answer 11: {}", problem_11(vec![
        vec![08,02,22,97,38,15,00,40,00,75,04,05,07,78,52,12,50,77,91,08],
        vec![49,49,99,40,17,81,18,57,60,87,17,40,98,43,69,48,04,56,62,00],
        vec![81,49,31,73,55,79,14,29,93,71,40,67,53,88,30,03,49,13,36,65],
        vec![52,70,95,23,04,60,11,42,69,24,68,56,01,32,56,71,37,02,36,91],
        vec![22,31,16,71,51,67,63,89,41,92,36,54,22,40,40,28,66,33,13,80],
        vec![24,47,32,60,99,03,45,02,44,75,33,53,78,36,84,20,35,17,12,50],
        vec![32,98,81,28,64,23,67,10,26,38,40,67,59,54,70,66,18,38,64,70],
        vec![67,26,20,68,02,62,12,20,95,63,94,39,63,08,40,91,66,49,94,21],
        vec![24,55,58,05,66,73,99,26,97,17,78,78,96,83,14,88,34,89,63,72],
        vec![21,36,23,09,75,00,76,44,20,45,35,14,00,61,33,97,34,31,33,95],
        vec![78,17,53,28,22,75,31,67,15,94,03,80,04,62,16,14,09,53,56,92],
        vec![16,39,05,42,96,35,31,47,55,58,88,24,00,17,54,24,36,29,85,57],
        vec![86,56,00,48,35,71,89,07,05,44,44,37,44,60,21,58,51,54,17,58],
        vec![19,80,81,68,05,94,47,69,28,73,92,13,86,52,17,77,04,89,55,40],
        vec![04,52,08,83,97,35,99,16,07,97,57,32,16,26,26,79,33,27,98,66],
        vec![88,36,68,87,57,62,20,72,03,46,33,67,46,55,12,32,63,93,53,69],
        vec![04,42,16,73,38,25,39,11,24,94,72,18,08,46,29,32,40,62,76,36],
        vec![20,69,36,41,72,30,23,88,34,62,99,69,82,67,59,85,74,04,36,16],
        vec![20,73,35,29,78,31,90,01,74,31,49,71,48,86,81,16,23,57,05,54],
        vec![01,70,54,71,83,51,54,69,16,92,33,48,61,43,52,01,89,19,67,48]]));
        */
    println!("Answer 12: {}", problem_12())
}

/*
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

fn problem_9(sum: i32) -> i32{
    let mut a:i32 = 0;
    let mut b:i32 = 0;
    let mut c:i32 = 0;
    for i in 0..sum {
        for j in (i+1)..sum{
            for k in (j+1)..sum {
                if i + j + k == sum {
                    if i.pow(2) + j.pow(2) == k.pow(2) {
                        a = i;
                        b = j;
                        c = k;
                        break;
                    }
                }
            }
        }
    }

    return vec![a, b, c].iter().product()
}

fn problem_10(max_val: i64) -> i64 {
    let mut o:i64 = 0;
    for i in 2..max_val {
        if is_prime(i) {
            o += i;
        }
    }

    return o
}

fn problem_11(mat: Vec<Vec<i32>>) -> i128 {
    let mut o:i128;
    let run_len = 4;

    //check left to right
    o = check_left_to_right(&mat, run_len);
    o = cmp::max(o, check_up_and_down(&mat, run_len));
    o = cmp::max(o, check_down_to_right(&mat, run_len));
    o = cmp::max(o, check_down_to_left(&mat, run_len));

    return o
}

fn check_left_to_right(mat: &Vec<Vec<i32>>, run_len: usize) -> i128 {
    let mut check_prod= 1;
    let mut o= 0;
    for i in mat.iter() {
        for j in 0..(i.len()-run_len + 1) {
            for k in 0..run_len{
                check_prod = check_prod * i[j+k] as i128;
            }

            if check_prod > o {
                o = check_prod;
            }
            check_prod = 1;
        }
    }
    return o

}

fn check_up_and_down(mat: &Vec<Vec<i32>>, run_len: usize) -> i128 {
    let mut check_prod = 1;
    let mut o = 0;
    // check up and down
    for i in 0..(mat[0].len()) {
        for j in 0..(mat.len()-run_len + 1) {
            //check product
            for k in 0..run_len{
                check_prod = check_prod * mat[(j+k)][i] as i128;
            }
            if check_prod > o {
                o = check_prod;
            }
            check_prod = 1;
        }
    }

    return o
}

fn check_down_to_right(mat: &Vec<Vec<i32>>, run_len: usize) -> i128 {
    let mut check_prod = 1;
    let mut o = 0;
    for i in 0..(mat.len()-run_len + 1) {
        for j in 0..(mat[i].len() - run_len + 1) {
            for k in 0..run_len {
                check_prod = check_prod * mat[i + k][j + k] as i128;
            }
            if check_prod > o {
                o = check_prod;
            }
            check_prod = 1;
        }
    }
    return o
}

fn check_down_to_left(mat: &Vec<Vec<i32>>, run_len: usize) -> i128 {
    let mut check_prod = 0;
    let mut o = 0;

    for i in 0..(mat.len()-run_len + 1) {
        for j in 0..(mat[i].len() - run_len + 1) {
            for k in (0..run_len).rev() {
                check_prod = check_prod * mat[i + (run_len - k - 1)][j + k] as i128;
            }
            if check_prod > o {
                o = check_prod;
            }
            check_prod = 1;
        }
    }

    return o
}
*/

