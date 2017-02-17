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
        for _ in 0..size {
            ret.sieve.push(true);
        }
        ret.sieve[0] = false;
        ret.sieve[1] = false;
        ret.sieve[2] = true;
        ret
    }
}

impl Iterator for SimpleSieve {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let mut i = self.current_index;
        while i < self.sieve.len() && !self.sieve[i] {
            i+=1;
        }
        self.current_index = i;
        if i < self.sieve.len() {
            //found a prime
            let prime = i;
            while i < self.sieve.len() {
                self.sieve[i] = false;
                i += prime;
            }
            Some(prime)
        } else {
            None
        }
    }
}

fn main() {
    for prime in SimpleSieve::new(1000) {
        println!("{:?}",prime);
    }
}
