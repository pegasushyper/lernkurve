use std::process;
use text_io::read;

fn get_coefficients(start: f64, time_frame: f64, hour_goal: f64) -> (f64, f64, f64) {
    let st = start * time_frame;

    let t3 = |num| 3.0 * num;

    let a = (t3(st) - t3(hour_goal)) / (2.0*time_frame.powf(3.0));

    let tn3 = |num| -3.0 * num;

    let b = (tn3(st) - tn3(hour_goal)) / time_frame.powf(2.0);

    let c = start;

    (a, b, c)
}

fn main() {
    print!("h/d you want to start with ");
    let start: f64 = read!("{}\n");

    print!("h/d maximum ");
    let limit: f64 = read!("{}\n");
    if start - limit >= 0.0 {
        eprintln!("limit must be greater than starting time");
        process::exit(1);
    }

    print!("how long will the streak go ");
    let time_frame: f64 = read!("{}\n");
    if time_frame <= 0.0 {
        eprintln!("time frame must be positive");
        process::exit(1);
    }

    print!("what total amount of hours you want to reach ");
    let hour_goal: f64 = read!("{}\n");

    if start - limit >= 0.0 {
        eprintln!("limit is greater than starting time");
        process::exit(1);
    }
    if (start*time_frame - hour_goal) / time_frame.powf(3.0) >= 0.0 {
        eprintln!("hour goal is set too low. should be at least {}", start*time_frame);
        process::exit(1);
    }
    if time_frame <= 0.0 {
        eprintln!("time frame must be positive");
        process::exit(1);
    }
    if (start*time_frame + 2.0*limit*time_frame - 3.0*hour_goal) / time_frame.powf(3.0) < 0.0 {
        eprintln!("hour goal is set too high. should be below {}", (start*time_frame + 2.0*limit*time_frame) / 3.0);
        process::exit(1);
    }

    print!("how many increments ");
    let steps: u8 = read!("{}\n");

    print!("want additional output Y/n ");
    let add_out: String = read!("{}\n");

    println!("---");

    let (a, b, c) = get_coefficients(start, time_frame, hour_goal);

    let apply_f = |x: f64| a*x.powf(2.0) + b*x + c;

    let apply_F = |x: f64| (a / 3.0)*x.powf(3.0) + (b / 2.0)*x.powf(2.0) + c*x;

    // https://math.stackexchange.com/questions/1653970/average-y-from-a-range-of-x-in-a-parabola
    
    let flatten = |start, end| (apply_F(end) - apply_F(start)) / (end - start);

    if add_out != "n" {
        println!("a: {}\nb: {}\nc: {}", a, b, c);
        println!("LaTEX formula: f(x) = {}x²+{}x+{}", a, b, c);
        println!("average h/d: {}", flatten(0.0, time_frame));
        println!("---");
    }

    let step_size = time_frame / f64::from(steps);
    println!("| days | average hours/day |");
    println!("| ---- | ----------------- |");
    for i in 0..steps {
        let begin = f64::from(i) * step_size;
        let finish = f64::from(i+1) * step_size;

        println!("| {} - {} | {} |", begin, finish, flatten(begin, finish));
    }
}
