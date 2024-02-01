fn main() {
    let empty = String::new();
    let johan = String::from("johan");
    let johan_caps = String::from("Johan");
    let isabelle = String::from("isabelle");
    let isabelle_caps = String::from("Isabelle");

    display_pig_latin_conversion(&empty);
    display_pig_latin_conversion(&johan);
    display_pig_latin_conversion(&johan_caps);
    display_pig_latin_conversion(&isabelle);
    display_pig_latin_conversion(&isabelle_caps);
}

fn is_vowel(char: &char) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y', 'A', 'E', 'I', 'O', 'U', 'Y'];
    vowels.contains(char)
}

fn is_consonant(char: &char) -> bool {
    char.is_alphabetic() && !is_vowel(char)
}

fn word_to_pig_latin(word: &String) -> String {
    let consonant_word_suffix = "ay";
    let vowel_word_suffix = "hay";

    let first_char = match word.chars().next() {
        None => return String::from(word),
        Some(value) => value,
    };

    if is_vowel(&first_char) {
        format!("{word}-{vowel_word_suffix}")
    } else if is_consonant(&first_char) {
        format!("{}-{first_char}{consonant_word_suffix}", &word[1..])
    } else {
        String::from(word)
    }
}

fn display_pig_latin_conversion(word: &String) {
    println!("{word} --> {}", word_to_pig_latin(word))
}
