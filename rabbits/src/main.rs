// https://www.reddit.com/r/dailyprogrammer/comments/7s888w/20180122_challenge_348_easy_the_rabbit_problem/

use std::{thread, time, io};

fn init(male: i128, female: i128, needed_alive: i128) -> ([i128; 96], [i128; 96], i128){
    
    let mut m: [i128; 96] = [0; 96];
    let mut f: [i128; 96] = [0; 96];
    m[2] = male;
    f[2] = female;
    return (m, f, needed_alive);
}


// reproduce, by letting all females 4 months and over have 9 female and 5 male descendants
// fn reproduce(m_pop: [i128; 96], f_pop: [i128; 96]) -> ([i128; 96], [i128;96], i128, i128) {
fn reproduce(m_pop: [i128; 96], f_pop: [i128; 96]) -> (i128, i128) {
    let mut res_m = 0;
    let mut res_f = 0;
    let f_iter = f_pop.iter().enumerate();
    for (i, count) in f_iter {
        if i < 95 && i > 3{
            // res_f[0] = res_f[0] + count * 9;
            // res_m[0] = res_m[0] + count * 5;
            res_m = res_m + count * 5;
            res_f = res_f + count * 9;
        }
    }
    return (res_m, res_f);
}

// age the bunny population by shifting the array one to the right
fn age(m_pop: [i128; 96], f_pop: [i128; 96], new_males: i128, new_females: i128) -> ([i128; 96], [i128; 96]){
    let mut res_m: [i128; 96] = [0; 96];
    let mut res_f: [i128; 96] = [0; 96];

    res_m[0] = new_males;
    res_f[0] = new_females;

    let f_iter = f_pop.iter().enumerate();
    let m_iter = m_pop.iter().enumerate();
    for (i, count) in f_iter {
        if i < 95{
            res_f[i+1] = *count;
        }
    }
    for (i, count) in m_iter {
        if i < 95{
            res_m[i+1] = *count;
        }
    }
    return (res_m, res_f);
}

fn main() {

    // this whole part is just for reading from stdin...

    let mut input_m = String::new();
    let mut input_f = String::new();
    let mut input_n = String::new();

    println!("Enter the number of male rabbits to start with:");
    io::stdin().read_line(&mut input_m).expect("failed to read input");
    println!("Enter the number of female rabbits to start with:");
    io::stdin().read_line(&mut input_f).expect("failed to read input");
    println!("Enter the number of rabbits you want to reach:");
    io::stdin().read_line(&mut input_n).expect("failed to read input");

    let trim_m = input_m.trim();
    let trim_f = input_f.trim();
    let trim_n = input_n.trim();

    let trims = vec!(trim_m, trim_f, trim_n);
    let mut mf: Vec<i128> = vec!(10, 10, 10000);
    for (i, trim) in trims.iter().enumerate() {
        let g = &trim.parse::<i128>().unwrap_or(20);
        mf[i] = *g;
    }

    // . up until here

    let (mut m, mut f, n_a) = init(mf[0], mf[1], mf[2]);
    let mut sum_male: i128 = m.iter().sum();
    let mut sum_female: i128 = f.iter().sum();
    let mut sum = sum_male + sum_female;
    // println!("There are now {} rabbits alive!", sum);
    let mut i = 0;
     while sum < n_a {
        // this handling of tuples sucks, there's gotta be a better way...
        let (r_m, r_f) = reproduce(m, f);
        let (r_m, r_f) = age(m, f, r_m, r_f);
        m = r_m;
        f = r_f;
        sum_male = m.iter().sum();
        sum_female = f.iter().sum();
        sum = sum_male + sum_female;
        println!("There are now {} rabbits alive! {} males and {} females", sum, sum_male, sum_female);
        thread::sleep(time::Duration::from_millis(500));
        i += 1;
    }
     println!("It took {} months!", i);
}
