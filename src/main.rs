use text_io::read;

fn get_coefficients(start: f64, limit: f64, time_frame: f64, hour_goal: f64) -> (f64, f64, f64) {
    let st = start * time_frame;
    let lt = limit * time_frame;

    let t3 = |num| 3.0 * num;

    let a = (t3(st) + t3(lt) - t3(2.0*hour_goal)) / time_frame.powf(3.0);

    let tn2 = |num| -2.0 * num;

    let b = (tn2(2.0*st) + tn2(lt) - tn2(3.0*hour_goal)) / time_frame.powf(2.0);

    let c = start.into();

    (a, b, c)
}

fn main() {
    print!("h/d you want to start with ");
    let start: f64 = read!("{}\n");

    print!("h/d maximum ");
    let limit: f64 = read!("{}\n");

    print!("how long will the streak go ");
    let time_frame: f64 = read!("{}\n");

    print!("what total amount of hours you want to reach ");
    let hour_goal: f64 = read!("{}\n");

    print!("how many increments ");
    let steps: u8 = read!("{}\n");

    print!("want additional output Y/n ");
    let add_out: String = read!("{}\n");

    if start * time_frame > hour_goal {
        println!("Your hour goal is set too low. This might lead to weird results. With your minimum h/d you want to go above {}", start*time_frame);
    }

    println!("---");

    let (a, b, c) = get_coefficients(start, limit, time_frame, hour_goal);

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
