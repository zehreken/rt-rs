use std::thread;
use std::time::{Duration, Instant};

const SIZE: usize = 10000;

pub fn test_thread() {
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
    // handle2.join().unwrap();
    // handle1.join().unwrap();

    let mut _a = a.clone();
    let mut _b = a.clone();
    let now = Instant::now();
    bubble_sort(&mut _a);
    bubble_sort(&mut _b);
    let then = Instant::now() - now;
    println!("in {} ms", then.as_millis());
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
