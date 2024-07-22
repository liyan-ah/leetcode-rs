impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        fn is_vowel(s: char) -> bool {
            match s {
                'a' | 'e' | 'i' | 'o' | 'u' => return true,
                'A' | 'E' | 'I' | 'O' | 'U' => return true,
                _ => return false,
            }
        }
        let mut arr: Vec<char> = s.chars().collect();
        let (mut i, mut j) = (0, arr.len() - 1);
        while i < j {
            while i < j && !is_vowel(arr[i]) {
                i += 1;
            }
            while i < j && !is_vowel(arr[j]) {
                j -= 1;
            }
            if i >= j {
                break;
            }
            arr.swap(i, j);
            i += 1;
            j -= 1;
        }
        arr.iter().collect()
    }
}
