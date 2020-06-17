use std::process::Command;

fn main() {
    match std::env::args().nth(1) {
        Some(n) => {
            let n = n.parse().unwrap();
            let mut v = Vec::<u8>::with_capacity(n);
            unsafe {
                let mut i = 0;
                let mut j = 0;
                while i < n {
                    *v.as_mut_ptr().add(i) = 2u8;
                    i += 4096;
                    j += 1;
                }
                println!("hits: {}", j);
            }
        }
        None => {
            let mut n = 10687086592usize;
            let me = std::env::current_exe().unwrap();
            loop {
                println!("attempt {}", n);
                let s = Command::new(&me).arg(&n.to_string()).status().unwrap();
                println!("{:?}", s);
                assert!(s.success());
                n += 100 << 20;
            }
        }
    }
}
