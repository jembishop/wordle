use wordle::*;

fn main() {
    let words = load();
    let words_n_mask: Vec<(Word, WordMask)> = words
        .into_iter()
        .map(|x| (x, compute_wordmask(x)))
        .collect();
    play(words_n_mask);
}

fn play(all_words: Vec<(Word, WordMask)>) {
    let mut current_word = str_to_word("lares");
    let mut possible_words = all_words.clone();
    println!("Welcome to wordle solver. A pattern input must be 
    'm' for yellow, 'c' for green, and 'x' for black. 
    eg. mxxxc , xmxxx
    ");
    loop {
        println!("Guess {} and enter pattern", word_to_str(current_word));
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let pat = str_to_pat(&line.trim());
        let n_pos_pr = possible_words.len();
        possible_words = words_consistent(possible_words, pat, current_word);
        let n_pos = possible_words.len();
        println!("{} -> {} word reduction", n_pos_pr, n_pos);
        if n_pos < 10 {
            for (w, _) in possible_words.iter() {
                println!("{}", word_to_str(*w));
            }
        }
        if n_pos == 1 {
            println!("Word is {}", word_to_str(possible_words[0].0));
            break
        }
        current_word = compute_best_word(&all_words, &possible_words) 
    } 
}
