extern crate rand;

use rand::{thread_rng, Rng};

use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

use counter::Counter;

pub mod counter;

struct MarkovBuilder<'a> {
    n: usize,
    ngrams: HashMap<&'a [&'a str], Counter<&'a str>>
}

impl<'a> MarkovBuilder<'a> {
    fn new() -> Self { Self::with_n(2) }

    fn with_n(n: usize) -> Self {
        MarkovBuilder { n: n, ngrams: HashMap::new() }
    }

    fn train<'b: 'a>(&mut self, words: &'b [&'b str]) {
        let windows = words.windows(self.n);
        let following = words.iter().skip(self.n);

        for (window, next) in windows.zip(following) {
            let counters = self.ngrams.entry(window).or_insert(Counter::new());
            (*counters).increment(next);
        }
    }

    pub fn generate(&self, size: usize) -> String {
        let mut distributions = HashMap::new();
        for (ngram, counter) in &self.ngrams {
            let total_counts = counter.total();
            let mut cfd = 0.0;
            let mut dist = Vec::new();

            for (&word, count) in counter.counts() {
                let prob = (*count as f64) / (total_counts as f64);
                cfd += prob;
                dist.push((word, cfd));
            }
            distributions.insert(ngram, dist);
        }

        let mut rng = thread_rng();
        let mut current = self.choose_seed(&mut rng);
        let mut generated_words = current.clone();

        for _ in 0..size {
            let choice = {
                let dist = distributions.get(&&current[..]).unwrap();
                pick_from_distribution(&mut rng, dist)
            };
            match choice {
                Some(word) => {
                    generated_words.push(word);
                    current.remove(0);
                    current.push(word);
                },
                None => break
            }
        }
        generated_words.join(" ")
    }

    fn choose_seed<R: Rng>(&self, rng: &mut R) -> Vec<&str> {
        let windows = self.ngrams.keys().collect::<Vec<_>>();
        let window = rng.choose(&windows).map(|w| *w).unwrap();

        let mut seed = Vec::new();
        seed.extend_from_slice(window);
        seed
    }
}

fn pick_from_distribution<'b, R: Rng>(rng: &mut R, distribution: &[(&'b str, f64)])
  -> Option<&'b str> {
    use rand::Closed01;

    if distribution.len() == 0 {
        None
    } else {
        let Closed01(p) = rng.gen::<Closed01<f64>>();
        distribution
            .iter()
            .find(|&&(_, cfd)| p <= cfd)
            .map(|&(word, _)| word)
    }
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("usage: markov <input>");
        return;
    }

    let mut input = String::new();
    match &args[1][..] {
      "-" => {
          let stdin = io::stdin();
          stdin.lock().read_to_string(&mut input).unwrap();
      },
      filename => {
          let mut file = File::open(filename).unwrap();
          file.read_to_string(&mut input).unwrap();
      }
    };

    let words = input.split_whitespace().collect::<Vec<&str>>();
    let mut markov_builder = MarkovBuilder::with_n(3);
    markov_builder.train(&words[..]);
    let generated = markov_builder.generate(200);
    println!("{}", generated);
}
