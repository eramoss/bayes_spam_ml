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
        let mut probability = prior_probability.ln();

        for word in words {
            if let Some(word_count) = word_counts.get(*word) {
                probability += (word_count + 1) as f64 / (message_count + 2) as f64;
            }
        }

        probability
    }
}

fn main() {
    let mut spam_filter = SpamFilter::new();

    spam_filter.train("Buy cheap watches", true);
    spam_filter.train("Hello, how are you?", false);

    let message = "Great deals on luxury watches!";
    let is_spam = spam_filter.predict(message);

    if is_spam {
        println!("The message is likely spam.");
    } else {
        println!("The message is likely not spam.");
    }
}
