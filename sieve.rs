mod prime {
    struct SimpleSieve {
        sieve: Vec<bool>,
        current_index: usize,
        last_prime: usize,
    }
    impl SimpleSieve {
        fn new(size: usize) -> SimpleSieve {
            let mut ret = SimpleSieve {
                sieve: Vec::with_capacity(if size < 3 { 3 } else { size }),
                current_index: 0,
                last_prime: 0,
            };
            ret.sieve[2] = true;
            ret.
            ret
        }
        fn next(&mut self) -> Option<usize> {
            let mut i = self.current_index;
            match self.sieve.iter().skip(i).position(true) {
                Some(prime_offset) => {
                    while i < self.sieve
                    self.sieve.
                },
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
