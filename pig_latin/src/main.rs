fn main() {
    let word_1 = String::from("Hello");
    let word_2 = String::from("Wubululu");
    let word_3 = String::from("Popokatepetl");
    pig_latin(word_1);
    pig_latin(word_2);
    pig_latin(word_3);

    fn pig_latin(input: String) {
        let word = input.to_lowercase();
        let mut chars = word.chars();
        let first_char = chars.next().unwrap();
        let rest_chars: String = chars.collect();

        println!("{}-{}ay", rest_chars, first_char);
    }
}
