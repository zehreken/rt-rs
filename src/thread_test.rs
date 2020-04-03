use std::thread;
use std::time::{Duration, Instant};

const SIZE: usize = 10000;

pub fn test_thread() {
    let main_now = Instant::now();

    let mut a = [0u32; SIZE];
    for i in 0..SIZE {
        a[i] = (SIZE - i) as u32;
    }

    let handle1 = thread::spawn(move || {
        let mut _a = a.clone();
        let now = Instant::now();
        bubble_sort(&mut _a);
        let then = Instant::now() - now;
        println!("in {} ms", then.as_millis());
    });

    let handle2 = thread::spawn(move || {
        let mut _a = a.clone();
        let now = Instant::now();
        bubble_sort(&mut _a);
        let then = Instant::now() - now;
        println!("in {} ms", then.as_millis());
    });

    let handle3 = thread::spawn(move || {
        let mut _a = a.clone();
        let now = Instant::now();
        bubble_sort(&mut _a);
        let then = Instant::now() - now;
        println!("in {} ms", then.as_millis());
    });

    let handle4 = thread::spawn(move || {
        let mut _a = a.clone();
        let now = Instant::now();
        bubble_sort(&mut _a);
        let then = Instant::now() - now;
        println!("in {} ms", then.as_millis());
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
    handle4.join().unwrap();

    let mut _a = a.clone();
    let mut _b = a.clone();
    let mut _c = a.clone();
    let mut _d = a.clone();
    let now = Instant::now();
    bubble_sort(&mut _a);
    bubble_sort(&mut _b);
    bubble_sort(&mut _c);
    bubble_sort(&mut _d);
    let then = Instant::now() - now;
    println!("in {} ms", then.as_millis());

    let then = Instant::now() - main_now;
    println!("total duration {} ms", then.as_millis());
}

fn bubble_sort(a: &mut [u32; SIZE]) {
    let n = a.len();
    for i in 0..n - 1 {
        for j in 0..n - i - 1 {
            if a[j] > a[j + 1] {
                let s = a[j];
                a[j] = a[j + 1];
                a[j + 1] = s;
            }
        }
    }
}
