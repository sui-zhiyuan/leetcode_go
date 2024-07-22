pub fn does_alice_win(s: String) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let vowel_count = s.chars().filter(|&c| vowels.contains(&c)).count();

    vowel_count % 2 == 1 || vowel_count > 0
}
