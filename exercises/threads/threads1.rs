// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)


use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::Mutex;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = JobStatus { jobs_completed: 0 };
    let arc = Arc::new(Mutex::new(status.jobs_completed));
    for i in 0..3 {
        let arc = arc.clone();
         thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
             println!("finished sleep {} ",&i);
            let mut jobsc =   arc.lock().unwrap();
            *jobsc+= 1;
        });
    };

    while (*arc.lock().unwrap()) < 3 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
        println!("completed... ");

    }
}
