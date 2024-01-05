use clap::*;
use std::collections::HashMap;
#[derive(Debug, Clone)]
struct SpamFilter {
    spam_word_counts: HashMap<String, usize>,
    ham_word_counts: HashMap<String, usize>,
    spam_message_count: usize,
    ham_message_count: usize,
}

impl SpamFilter {
    fn new() -> Self {
        SpamFilter {
            spam_word_counts: HashMap::new(),
            ham_word_counts: HashMap::new(),
            spam_message_count: 0,
            ham_message_count: 0,
        }
    }

    fn train(&mut self, message: &str, is_spam: bool) {
        let words: Vec<&str> = message.split_whitespace().collect();

        for word in words {
            let word_count = if is_spam {
                self.spam_word_counts.entry(word.to_string()).or_insert(0)
            } else {
                self.ham_word_counts.entry(word.to_string()).or_insert(0)
            };
            *word_count += 1;
        }

        if is_spam {
            self.spam_message_count += 1;
        } else {
            self.ham_message_count += 1;
        }
    }

    fn predict(&self, message: &str) -> bool {
        let words: Vec<&str> = message.split_whitespace().collect();
        let spam_probability =
            self.calculate_probability(&words, &self.spam_word_counts, self.spam_message_count);
        let ham_probability =
            self.calculate_probability(&words, &self.ham_word_counts, self.ham_message_count);

        spam_probability > ham_probability
    }

    fn calculate_probability(
        &self,
        words: &[&str],
        word_counts: &HashMap<String, usize>,
        message_count: usize,
    ) -> f64 {
        let prior_probability =
            message_count as f64 / (self.spam_message_count + self.ham_message_count) as f64;
        let mut probability = prior_probability.ln(); // prevent underflow

        for word in words {
            if let Some(word_count) = word_counts.get(*word) {
                probability += (word_count + 1) as f64 / (message_count + 2) as f64;
            }
        }

        probability
    }
}
fn main() {
    let matches = App::new("Bayesian Spam Filter CLI")
        .version("1.0")
        .author("eramoss")
        .about("Train and predict with a Bayesian spam filter.")
        .arg(
            Arg::with_name("train")
                .short("t")
                .long("train")
                .value_name("FILE")
                .help("Trains the spam filter with the content of the specified file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("predict")
                .short("p")
                .long("predict")
                .value_name("MESSAGE")
                .help("Predicts whether the given message is spam")
                .takes_value(true),
        )
        .get_matches();

    let mut spam_filter = SpamFilter::new();

    if let Some(train_file) = matches.value_of("train") {
        match std::fs::read_to_string(train_file) {
            Ok(content) => {
                for line in content.lines() {
                    // Assume each line is a message, and the last character indicates the label (0 for ham, 1 for spam)
                    let is_spam = line.ends_with("1");
                    let message = line.trim_end_matches("01").trim();

                    spam_filter.train(message, is_spam);
                }
                println!("Training completed.");
            }
            Err(err) => eprintln!("Error reading training file: {}", err),
        }
    }

    if let Some(message) = matches.value_of("predict") {
        let is_spam = spam_filter.predict(message);
        if is_spam {
            println!("The message is likely spam.");
        } else {
            println!("The message is likely not spam.");
        }
    }

    // If no arguments are provided, run in interactive mode
    if matches.args.is_empty() {
        eprintln!(
            "No arguments provided.\n\nUsage:\n\tbayes_spam_ml --predict <MESSAGE> --train <FILE>"
        );
    }
}
