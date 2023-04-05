use std::time::Instant;

use wordle::*;
use structopt::StructOpt;
use std::path::PathBuf;


#[derive(Debug, StructOpt)]
#[structopt(name = "wordle", about = "Wordle solver")]
struct Opt {
    #[structopt(long, help = "Compute the starter word, otherwise use 'lares'")]
    compute_starter: bool,

    #[structopt(parse(from_os_str), default_value = "words.json", help = "The json file to look for all words")]
    words: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let words = load(opt.words);
    let words_n_mask: Vec<(Word, WordMask)> = words
        .into_iter()
        .map(|x| (x, compute_wordmask(x)))
        .collect();

    play(words_n_mask, opt.compute_starter);
}

fn play(all_words: Vec<(Word, WordMask)>, compute_starter: bool) {
    let mut possible_words = all_words.clone();
    let mut current_word = if compute_starter {
        let now = Instant::now();
        let best_starter = compute_best_word(&all_words, &possible_words);
        println!("Best starter computed to be: {} in {:1}s", word_to_str(best_starter), now.elapsed().as_secs_f32());
        best_starter
    } else {
        str_to_word("lares")
    };
    println!(
        "Welcome to the wordle solver. A pattern input must be 
    'm' for yellow, 'c' for green, and 'x' for black. 
    eg. mxxxc , xmxxx
    "
    );
    loop {
        println!("Guess {} and enter pattern", word_to_str(current_word));
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let now = Instant::now();
        let pat = str_to_pat(&line.trim());
        let n_pos_pr = possible_words.len();
        possible_words = words_consistent(possible_words, pat, current_word);
        let n_pos = possible_words.len();
        println!(
            "{} -> {} word reduction in {:.2}us",
            n_pos_pr,
            n_pos,
            now.elapsed().as_micros()
        );
        if n_pos < 10 {
            for (w, _) in possible_words.iter() {
                println!("{}", word_to_str(*w));
            }
        }
        if n_pos == 1 {
            println!("Word is {}", word_to_str(possible_words[0].0));
            break;
        }
        current_word = compute_best_word(&all_words, &possible_words)
    }
}
