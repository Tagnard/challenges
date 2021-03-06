#![allow(dead_code)]

use regex::Regex;

/// Title: White Spaces Between Lower and Uppercase Letters
/// Challenge URL: https://edabit.com/challenge/df9LtdceySMvqQJtW
/// Task: Write a function that adds spaces between the words.
fn insert_whitespace(sentence: &str) -> String {
    let re = Regex::new(r"(?:([A-Z]{1}[a-z]+)),?").unwrap();
    let r: Vec<&str> = re.captures_iter(sentence).map(|cap| cap.get(0).unwrap().as_str()).collect();
    return r.join(" ");
}

#[test]
fn validate_insert_whitespace() {
    assert_eq!(insert_whitespace("SheWalksToTheBeach"), "She Walks To The Beach");
    assert_eq!(insert_whitespace("MarvinTalksTooMuch"), "Marvin Talks Too Much");
    assert_eq!(insert_whitespace("TheGreatestUpsetInHistory"), "The Greatest Upset In History");
}

/// Title: Find the Bomb
/// Challenge URL: https://edabit.com/challenge/FbgicbJyLQan2pbde
/// Task: Create a function that finds the word "bomb" in the given string (not case sensitive). 
///       If found, return "Duck!!!", otherwise return "There is no bomb, relax."
fn bomb(sentence: &str) -> &str {
    return match sentence.find("bomb") {
        Some(..) => "Duck!!!",
        None => "There is no bomb, relax.",
    }
}

#[test]
fn validate_bomb() {
    assert_eq!(bomb("There is a bomb."), "Duck!!!");
    assert_eq!(bomb("Hey, did you think there is a bomb?"), "Duck!!!");
    assert_eq!(bomb("This goes boom!!!"), "There is no bomb, relax.");
}

/// Title: IPv4 Validation
/// Challenge URL: https://edabit.com/challenge/vk3NibPRqsR2RquTM
/// Task: Create a function that takes a string (IPv4 address in standard dot-decimal format) 
///       and returns true if the IP is valid or false if it's not.
fn ipv4(addr: &str) -> bool {
    let re = Regex::new(r"^0+").unwrap();

    let octets: Vec<&str> = addr.split(".").collect();
    if octets.len() != 4 { return false }
    for octet in octets {
        if re.is_match(octet) { return false }

        match octet.parse::<u8>() {
            Ok(..) => {},
            Err(..) => return false,
        }
    }

    return true;
}

#[test]
fn validate_ipv4() {
    assert_eq!(ipv4("1.2.3.4"), true);
    assert_eq!(ipv4("1.2.3"), false);
    assert_eq!(ipv4("1.2.3.4.5"), false);
    assert_eq!(ipv4("123.45.67.89"), true);
    assert_eq!(ipv4("123.456.78.90"), false);
    assert_eq!(ipv4("123.045.067.089"), false);
}

/// Title: Count Letters in a Word Search
/// Awnser to challenge https://edabit.com/challenge/bsPZtsX62zQmRHNjX
/// Task: Create a function that counts the number of times a particular letter shows up in the word search.
fn count_letters(letters: Vec<Vec<char>>, r#match: char) -> u32 {
    let mut c: u32 = 0;
    for l1 in letters.into_iter() {
        for l2 in l1.into_iter() {
            if l2 == r#match {
                c = c + 1;
            }
        }
    }
    return c;
}

#[test]
fn validate_count_letters() {
    assert_eq!(count_letters(vec![
        vec!['D', 'E', 'Y', 'H', 'A', 'D'],
        vec!['D', 'B', 'C', 'A', 'M', 'N'],
        vec!['F', 'G', 'G', 'R', 'S', 'R'],
        vec!['V', 'X', 'H', 'A', 'S', 'S']
    ], 'D'), 3);

    assert_eq!(count_letters(vec![
        vec!['D', 'E', 'Y', 'H', 'A', 'D'],
        vec!['D', 'B', 'C', 'A', 'M', 'N'],
        vec!['F', 'G', 'G', 'R', 'S', 'R'],
        vec!['V', 'X', 'H', 'A', 'S', 'S']
    ], 'H'), 2);
}

/// Title: Owofied a Sentence
/// Challenge URL https://edabit.com/challenge/G3hRLmmrcMaGAbf6F
/// Task: Create a function that takes a sentence and turns every "i" into "wi" and "e"
fn owofied(sentence: &str) -> String {
    let mut sentence: String = String::from(sentence).replace("i", "wi").replace("e", "we");
    sentence.push_str(" owo");
    return sentence.to_owned();
}

#[test]
fn validate_owofied() {
    assert_eq!(owofied("I'm gonna ride 'til I can't no more"), "I'm gonna rwidwe 'twil I can't no morwe owo");
    assert_eq!(owofied("Do you ever feel like a plastic bag"), "Do you wevwer fwewel lwikwe a plastwic bag owo");
    assert_eq!(owofied("Cause baby you're a firework"), "Causwe baby you'rwe a fwirwework owo");
}

/// Title: Absolute Sum
/// Challenge URL: https://edabit.com/challenge/jBFpjEG3tvsjTZbD4
/// Task: Take an array of integers (positive or negative or both) and return the sum of the absolute value of each element.
fn get_abs_sum(arr: Vec<i32>) -> Option<i32> {
    if arr.len() == 1 {
        return Some(arr[0].abs());
    }

    return arr.into_iter().reduce(|a, b| a.abs() + b.abs());
}

#[test]
fn validate_get_abs_sum() {
    assert_eq!(get_abs_sum(vec![2, -1, 4, 8, 10]).unwrap(), 25);
    assert_eq!(get_abs_sum(vec![-3, -4, -10, -2, -3]).unwrap(), 22);
    assert_eq!(get_abs_sum(vec![2, 4, 6, 8, 10]).unwrap(), 30);
    assert_eq!(get_abs_sum(vec![-1]).unwrap(), 1);
}

/// Title: Perfect Square Patch
/// Challenge URL: https://edabit.com/challenge/omTRzwvBibk4etBkx
/// Task: Create a function that takes an integer and outputs an n x n square solely consisting of the integer n.
fn square_patch(n: usize) -> Vec<Vec<usize>> {
    return vec![vec![n; n]; if n == 0 { 1 } else { n }];
}

#[test]
fn validate_square_patch() {
    assert_eq!(square_patch(3), vec![
        vec![3, 3, 3],
        vec![3, 3, 3],
        vec![3, 3, 3]
    ]);

    assert_eq!(square_patch(5), vec![
        vec![5, 5, 5, 5, 5],
        vec![5, 5, 5, 5, 5],
        vec![5, 5, 5, 5, 5],
        vec![5, 5, 5, 5, 5],
        vec![5, 5, 5, 5, 5],
    ]);

    assert_eq!(square_patch(1), vec![
        vec![1],
    ]);

    assert_eq!(square_patch(0), vec![
        vec![],
    ]);
}

/// Title: Valid Hex Code
/// Challenge URL: https://edabit.com/challenge/ZhEBoaEfMcK6vT7Kx
/// Task: Write a function that validated hex codes to the format #000000
fn is_valid_hex_code(hex: &str) -> bool {
    let re = Regex::new(r"^#[a-fA-F0-9]{6}$").unwrap();
    return re.is_match(hex);
} 

#[test]
fn validate_is_valid_hex_code() {
    assert_eq!(is_valid_hex_code("#CD5C5C"), true);
    assert_eq!(is_valid_hex_code("#EAECEE"), true);
    assert_eq!(is_valid_hex_code("#eaecee"), true);
    assert_eq!(is_valid_hex_code("#CD5C58C"), false);
    assert_eq!(is_valid_hex_code("#CD5C5Z"), false);
    assert_eq!(is_valid_hex_code("#CD5C&C"), false);
    assert_eq!(is_valid_hex_code("CD5C5C"), false);
}

/// Title: Toy Car Workshop
/// Challenge URL: https://edabit.com/challenge/Ccoo3SHJwv4qCLKQb
/// Task: Write a function that returns the abount of toy cars
///       that can be build with the parts provided.
fn cars(mut wheels: u32, mut chassis: u32, mut figures: u32) -> u32 {
    let mut c: u32 = 0;
    while wheels > 4 && chassis > 1 && figures > 2 {
        wheels = wheels - 4;
        chassis = chassis - 1;
        figures = figures - 2;
        c = c + 1;
    }

    return c;
}

#[test]
fn validate_cars() {
    assert_eq!(cars(2, 48, 76), 0);
    assert_eq!(cars(43, 15, 87), 10);
    assert_eq!(cars(88, 37, 17), 8);
}

/// Title: Tic Tac Toe
/// Challenge URL: https://edabit.com/challenge/rscwis53jKokoKRYo
/// Task: Create a function that takes an array of char inputs from a Tic Tac Toe game. 
///       Inputs will be taken from player1 as "X", player2 as "O", and empty spaces as "#". 
///       The program will return the winner or tie results.
fn tic_tac_toe(board: Vec<Vec<char>>) -> &'static str {
    for i in 0..=2 {
        if board[i][0] == 'X' && board[i][1] == 'X' && board[i][2] == 'X' { return "Player 1 wins" }
        if board[0][i] == 'X' && board[1][i] == 'X' && board[2][i] == 'X' { return "Player 1 wins" }

        if board[i][0] == 'O' && board[i][1] == 'O' && board[i][2] == 'O' { return "Player 2 wins" }
        if board[0][i] == 'O' && board[1][i] == 'O' && board[2][i] == 'O' { return "Player 2 wins" }
    }

    if board[0][0] == 'X' && board[1][1] == 'X' && board[2][2] == 'X' { return "Player 1 wins" }
    if board[2][0] == 'X' && board[1][1] == 'X' && board[1][2] == 'X' { return "Player 1 wins" }

    if board[0][0] == 'O' && board[1][1] == 'O' && board[2][2] == 'O' { return "Player 2 wins" }
    if board[2][0] == 'O' && board[1][1] == 'O' && board[1][2] == 'O' { return "Player 2 wins" }

    return "It's a Tie"
} 

#[test]
fn validate_tic_tac_toe() {
    assert_eq!(tic_tac_toe(vec![
        vec!['X', 'O', 'O'],
        vec!['O', 'X', 'O'],
        vec!['O', '#', 'X']
    ]), "Player 1 wins");

    assert_eq!(tic_tac_toe(vec![
        vec!['X', 'O', 'O'],
        vec!['O', 'X', 'O'],
        vec!['X', '#', 'O']
    ]), "Player 2 wins");

    assert_eq!(tic_tac_toe(vec![
        vec!['X', 'O', 'O'],
        vec!['O', 'X', 'O'],
        vec!['X', '#', 'O']
    ]), "Player 2 wins");
}

/// Title: Burrrrrrrp
/// Challenge URL: https://edabit.com/challenge/vxpP4nnDhRr2Yc3Lo
/// Task: Create a function that returns the string "Burp" with the amount of "r's" determined by the input parameters of the function.
fn long_burp(num_r: usize) -> String {
    return format!("Bu{:r<width$}p", "r", width = num_r);
}

#[test]
fn validate_long_burp() {
    assert_eq!(long_burp(3), "Burrrp");
    assert_eq!(long_burp(5), "Burrrrrp");
    assert_eq!(long_burp(9), "Burrrrrrrrrp");
}

/// Title: Broken Bridge
/// Challenge URL: https://edabit.com/challenge/udrQ2ckXP9cXNXiSk
/// Task: Create a function which validates whether a bridge is safe to walk on (i.e. has no gaps in it to fall through).
fn is_safe_bridge(bridge: &str) -> bool {
    for c in bridge.chars() {
        if c == ' ' {
            return false;
        }
    }
    return true;
}

#[test]
fn validate_is_safe_bridge() {
    assert_eq!(is_safe_bridge("####"), true);
    assert_eq!(is_safe_bridge("## ####"), false);
    assert_eq!(is_safe_bridge("#"), true);
}

/// Title: Accumulating Array
/// Challenge URL: https://edabit.com/challenge/TmL3qiZE7c25TLmSj
/// Task: Create a function that takes in an array and returns an array of the accumulating sum.
fn accumulating_array(arr: Vec<u32>) -> Vec<u32> {
    let mut ret: Vec<u32> = Vec::with_capacity(0);
    arr.into_iter().for_each(|n| {
        if ret.len() == 0 {
            ret.push(n)
        } else {
            ret.push(ret[ret.len()-1] + n);
        }
    });
    return ret;
}

#[test]
fn validate_accumulating_array() {
    assert_eq!(accumulating_array(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
    assert_eq!(accumulating_array(vec![1, 5, 7]), vec![1, 6, 13]);
    assert_eq!(accumulating_array(vec![1, 0, 1, 0, 1]), vec![1, 1, 2, 2, 3]);
    assert_eq!(accumulating_array(vec![]), vec![]);
}

/// Title: Letters Only
/// Challenge URL: https://edabit.com/challenge/azvb6mtiQvRGo6XTf
/// Task: Check if the given string consists of only letters and spaces and if every letter is in lower case.
fn letters_only(letters: &str) -> bool {
    let re = Regex::new(r"^[a-z ]+$").unwrap();
    return re.is_match(letters);
}

#[test]
fn validate_letters_only() {
    assert_eq!(letters_only("JAVACRIPT"), false);
    assert_eq!(letters_only("javascript"), true);
    assert_eq!(letters_only("12321313"), false);
    assert_eq!(letters_only("i have spaces"), true);
    assert_eq!(letters_only("i have numbers(1-10)"), false);
    assert_eq!(letters_only(""), false);
}

/// Title: Reverse Coding Challenge #1
/// Challenge URL: https://edabit.com/challenge/gtdyy97TTDPWkei9d
/// Task: Create a function that satisfy the relationship between the inputs and outputs.
fn rcc1(letters: &str) -> String {
    let re = Regex::new(r"(?:(([A-Z]{1})([0-9]+))),?").unwrap();
    return re.captures_iter(letters).map(|cap| {
        let letter = cap.get(2).unwrap().as_str();
        let number = cap.get(3).unwrap().as_str();
        return vec![letter;number.parse::<usize>().unwrap()].join("");
    }).collect();
}

#[test]
fn validate_rcc1() {
    assert_eq!(rcc1("A4B5C2"), "AAAABBBBBCC");
    assert_eq!(rcc1("C2F1E5"), "CCFEEEEE");
    assert_eq!(rcc1("T4S2V2"), "TTTTSSVV");
    assert_eq!(rcc1("A1B2C3D4"), "ABBCCCDDDD");
}

/// Title: Caesar's Cipher
/// Challenge URL: https://edabit.com/challenge/GmPfqu2jmLDBD2NYS
/// Task: Create a function that takes a string s (text to be encrypted) 
///       and an integer k (the rotation factor). It should return an encrypted string.
fn caesar_cipher(letters: &str, shift: usize) -> String { 
    letters.chars().map(|c| {
        // Shift uppercase
        if (c as u8) > 64 && (c as u8) < 91 {
            if (c as u8 + shift as u8) > 90 {
                return (c as u8 + shift as u8  - 26u8) as char
            } else {
                return (c as u8 + shift as u8) as char
            }
        // Shift lowerchase
        } else if (c as u8) > 96 && (c as u8) < 123 {
            if (c as u8 + shift as u8) > 122 {
                return (c as u8 + shift as u8  - 26u8) as char
            } else {
                return (c as u8 + shift as u8) as char
            }
        } else {
            return c
        }
    }).collect()
}

#[test]
fn validate_caesar_cipher() {
    assert_eq!(caesar_cipher("Always-Look-on-the-Bright-Side-of-Life", 5), "Fqbfdx-Qttp-ts-ymj-Gwnlmy-Xnij-tk-Qnkj");
    assert_eq!(caesar_cipher("A friend in need is a friend indeed", 20), "U zlcyhx ch hyyx cm u zlcyhx chxyyx");
    assert_eq!(caesar_cipher("XxYyZz", 6), "DdEeFf");
}

/// Title: Decompose Address
/// Challenge URL: https://edabit.com/challenge/ah9SjMJzFmNLD54W9
/// Task: Create a function that decomposes an address string into an array of five substrings
fn decompose_address(address: &str) -> Vec<&str> {
    let re = Regex::new(r"^(\d+) (\w+ \w+) ([^,]+), ([A-Z]{2}) (\d+)$").unwrap();
    let captures = re.captures(address).unwrap();
    vec![
        captures.get(1).unwrap().as_str(),
        captures.get(2).unwrap().as_str(),
        captures.get(3).unwrap().as_str(),
        captures.get(4).unwrap().as_str(),
        captures.get(5).unwrap().as_str(),
    ]
}

#[test]
fn validate_decompose_address() {
    assert_eq!(decompose_address("557 Farmer Rd Corner, MT 59105"), vec!["557", "Farmer Rd", "Corner", "MT", "59105"]);
    assert_eq!(decompose_address("3315 Vanity St Beverly Hills, CA 90210"), vec!["3315", "Vanity St", "Beverly Hills", "CA", "90210"]);
    assert_eq!(decompose_address("8919 Scarecrow Ct Idaho Falls, ID 80193"), vec!["8919", "Scarecrow Ct", "Idaho Falls", "ID", "80193"]);
}

/// Title: Message from Space
/// Challenge URL: https://edabit.com/challenge/58iEEYqxFdnkBjEiA
/// Task: Create a function that takes encrypted message str and returns the decrypted message.
pub fn space_message(message: &str) -> String {
    let re = Regex::new(r"(\w+|\[(\d)(\w+)\])").unwrap();
    re.captures_iter(message).map(|m| {
        match m.get(0).unwrap().as_str().find("[") {
            Some(..) => {
                return vec![m.get(3).unwrap().as_str(); m.get(2).unwrap().as_str().parse::<usize>().unwrap()].join("")
            },
            None => return m.get(0).unwrap().as_str().to_string()
        }
    }).collect()
}

#[test]
fn validate_space_message() {
    assert_eq!(space_message("ABCD"), "ABCD");
    assert_eq!(space_message("AB[3CD]"), "ABCDCDCD");
    assert_eq!(space_message("IF[2E]LG[5O]D"), "IFEELGOOOOOD");
}

/// Title: Amplify the Multiples of Four
/// Challenge URL: https://edabit.com/challenge/kKvXt3DiTrGmvFCWq
/// Task: Create a function that takes an integer and returns an array from 1 to the given number, where:
///       1. If the number can be divided evenly by 4, amplify it by 10 (i.e. return 10 times the number).
///       2. If the number cannot be divided evenly by 4, simply return the number.
pub fn amplify(n: u32) -> Vec<u32> {
    (1..=n).into_iter().map(|num| if num % 4 == 0 { num * 10 } else { num }).collect()
}

#[test]
fn validate_amplify() {
    assert_eq!(amplify(4), vec![1, 2, 3, 40]);
    assert_eq!(amplify(3), vec![1, 2, 3]);
    assert_eq!(amplify(25), vec![1, 2, 3, 40, 5, 6, 7, 80, 9, 10, 11, 120, 13, 14, 15, 160, 17, 18, 19, 200, 21, 22, 23, 240, 25]);
}

/// Title: Return the Four Letter Strings
/// Challenge URL: https://edabit.com/challenge/3HEY2ZfqMXyZsA87i
/// Task: Create a function that takes a vector (array) of strings. 
///       Return all words in the vector (array) that are exactly four letters.
pub fn is_four_letters(words: Vec<&str>) -> Vec<&str> {
    words.into_iter().filter(|word| word.len() == 4).collect()
}

#[test]
fn validate_is_four_letters() {
    assert_eq!(is_four_letters(vec!["Tomato", "Potato", "Pair"]), vec!["Pair"]);
    assert_eq!(is_four_letters(vec!["Kangaroo", "Bear", "Fox"]), vec!["Bear"]);
    assert_eq!(is_four_letters(vec!["Ryan", "Kieran", "Jason", "Matt"]), vec!["Ryan", "Matt"]);
}

/// Title: Compare by ASCII Codes
/// Challenge URL: https://edabit.com/challenge/2otrC5z9iFQm4Zczb
/// Task: Create a function that compares two words based on the sum of their ASCII codes 
///       and returns the word with the smaller ASCII sum.
pub fn ascii_sort(words: Vec<&str>) -> &str {
    let mut word_one: u32 = 0;
    let mut word_two: u32 = 0;
    
    words[0].chars().for_each(|c| word_one += (c as u8) as u32);
    words[1].chars().for_each(|c| word_two += (c as u8) as u32);

    println!("w1: {}, w2: {}", word_one, word_two);

    if word_one < word_two {
        return words[0]
    } else {
        return words[1]
    } 
}

#[test]
fn validate_ascii_sort() {
    assert_eq!(ascii_sort(vec!["hey", "man"]), "man");
    assert_eq!(ascii_sort(vec!["majorly", "then"]), "then");
    assert_eq!(ascii_sort(vec!["victory", "careless"]), "victory");
}

/// Title: Convert to Hex
/// Challenge URL: https://edabit.com/challenge/2otrC5z9iFQm4Zczb
/// Task: Create a function that takes a strings characters as ASCII 
///       and returns each characters hexadecimal value as a string.
pub fn to_hex(sentence: &str) -> String {
    sentence.chars().map(|c| format!("{:x}", c as u8)).collect::<Vec<String>>().join(" ")
}

#[test]
fn validate_to_hex() {
    assert_eq!(to_hex("hello world"), "68 65 6c 6c 6f 20 77 6f 72 6c 64");
    assert_eq!(to_hex("Big Boi"), "42 69 67 20 42 6f 69");
    assert_eq!(to_hex("Marty Poppinson"), "4d 61 72 74 79 20 50 6f 70 70 69 6e 73 6f 6e");
}