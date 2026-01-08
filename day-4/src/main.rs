use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input");

    let mut result = 0;
    let lines: Vec<&str> = input.lines().collect();

    for (line_index, line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        for (char_index, char) in chars.iter().enumerate() {
            let mut adjecent = 0;
            if char != &'@' {
                continue;
            }

            if char_index == 0 {
                if chars[char_index + 1] == '@' {
                    adjecent = adjecent + 1
                }
            } else if char_index == chars.len() - 1 {
                if chars[char_index - 1] == '@' {
                    adjecent = adjecent + 1
                }
            } else {
                if chars[char_index - 1] == '@' {
                    adjecent = adjecent + 1
                }
                if chars[char_index + 1] == '@' {
                    adjecent = adjecent + 1
                }
            }

            if line_index == 0 {
                let next_chars: Vec<char> = lines[line_index + 1].chars().collect();
                if char_index == 0 {
                    if next_chars[char_index] == '@' {
                        adjecent = adjecent + 1
                    }
                    if next_chars[char_index + 1] == '@' {
                        adjecent = adjecent + 1
                    }
                } else if char_index == chars.len() - 1 {
                    if next_chars[char_index - 1] == '@' {
                        adjecent = adjecent + 1
                    }
                    if next_chars[char_index] == '@' {
                        adjecent = adjecent + 1
                    }
                } else {
                    if next_chars[char_index - 1] == '@' {
                        adjecent = adjecent + 1
                    }
                    if next_chars[char_index] == '@' {
                        adjecent = adjecent + 1
                    }
                    if next_chars[char_index + 1] == '@' {
                        adjecent = adjecent + 1
                    }
                }
            } else if line_index == lines.len() - 1 {
                let prev_chars: Vec<char> = lines[line_index - 1].chars().collect();
                if char_index == 0 {
                    if prev_chars[char_index] == '@' {
                        adjecent = adjecent + 1
                    }
                    if prev_chars[char_index + 1] == '@' {
                        adjecent = adjecent + 1
                    }
                } else if char_index == chars.len() - 1 {
                    if prev_chars[char_index - 1] == '@' {
                        adjecent = adjecent + 1
                    }
                    if prev_chars[char_index] == '@' {
                        adjecent = adjecent + 1
                    }
                } else {
                    if prev_chars[char_index - 1] == '@' {
                        adjecent = adjecent + 1
                    }
                    if prev_chars[char_index] == '@' {
                        adjecent = adjecent + 1
                    }
                    if prev_chars[char_index + 1] == '@' {
                        adjecent = adjecent + 1
                    }
                }
            } else {
                let prev_chars: Vec<char> = lines[line_index - 1].chars().collect();
                let next_chars: Vec<char> = lines[line_index + 1].chars().collect();
                if char_index == 0 {
                    if prev_chars[char_index] == '@' {
                        adjecent = adjecent + 1
                    }
                    if prev_chars[char_index + 1] == '@' {
                        adjecent = adjecent + 1
                    }
                    if next_chars[char_index] == '@' {
                        adjecent = adjecent + 1
                    }
                    if next_chars[char_index + 1] == '@' {
                        adjecent = adjecent + 1
                    }
                } else if char_index == chars.len() - 1 {
                    if prev_chars[char_index - 1] == '@' {
                        adjecent = adjecent + 1
                    }
                    if prev_chars[char_index] == '@' {
                        adjecent = adjecent + 1
                    }
                    if next_chars[char_index - 1] == '@' {
                        adjecent = adjecent + 1
                    }
                    if next_chars[char_index] == '@' {
                        adjecent = adjecent + 1
                    }
                } else {
                    if prev_chars[char_index - 1] == '@' {
                        adjecent = adjecent + 1
                    }
                    if prev_chars[char_index] == '@' {
                        adjecent = adjecent + 1
                    }
                    if prev_chars[char_index + 1] == '@' {
                        adjecent = adjecent + 1
                    }
                    if next_chars[char_index - 1] == '@' {
                        adjecent = adjecent + 1
                    }
                    if next_chars[char_index] == '@' {
                        adjecent = adjecent + 1
                    }
                    if next_chars[char_index + 1] == '@' {
                        adjecent = adjecent + 1
                    }
                }
            }

            if adjecent < 4 {
                result = result + 1
            }
        }
    }
    println!("Its {}", result);
}
