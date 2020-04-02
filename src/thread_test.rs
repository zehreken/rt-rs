use std::thread;
use std::time::{Duration, Instant};

const SIZE: usize = 10000;

pub fn test_thread() {
    let handle1 = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from 1", i);
            thread::sleep(Duration::from_micros(1));
        }
    });

    let handle2 = thread::spawn(|| {
        for i in 1..20 {
            println!("hi number {} from 2", i);
            thread::sleep(Duration::from_micros(1));
        }
    });
    handle2.join().unwrap();
    handle1.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from 0", i);
        thread::sleep(Duration::from_micros(1));
    }

    let mut a = [0u32; SIZE];
    for i in 0..SIZE {
        a[i] = (SIZE - i) as u32;
    }

    let now = Instant::now();
    bubble_sort(&mut a);
    let then = Instant::now() - now;
    println!("in {}", then.as_millis());
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
