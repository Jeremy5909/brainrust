use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};
struct BrainFun {
    current_pos: i16,
    short_mode: bool
}
#[allow(dead_code)]

impl BrainFun {
    fn new(short_mode: bool) -> Self {
        BrainFun {
            current_pos: 1,
            short_mode
        }
    }

    fn negate_arrows(arrows: &str) -> String {
        arrows 
            .chars()
            .map(|arrow| match arrow {
                '>' => '<',
                '<' => '>',
                _ => arrow,
            })
            .collect()
    }

    fn is_prime(num: i16) -> bool {
        for i in 2..=num / 2 {
            return num % i != 0
        }
        true
    }

    fn add(&mut self, num: i16) -> String {
        let mut output = String::new();
        if num.abs() < 15 {
            if num > 0 {
                output.push_str(&"+".repeat(num as usize));
            } else if num < 0 {
                output.push_str(&"-".repeat((-num) as usize));
            }
        } else {
            let mut additional: i16 = 0;
            let mut temp_num: i16 =  num.abs();
            if Self::is_prime(temp_num) {
                additional = 1;
                temp_num -= 1;
            }
            let mut first_closest: i16 = (temp_num as f64).sqrt() as i16;
            while num % first_closest != 0 {
                first_closest -= 1;
            }
            let second_closest = temp_num/first_closest;
            output.push_str(&self.set_pos(self.current_pos + 1));
            output.push_str(&"+".repeat(first_closest as usize));
            output.push_str("[");
            output.push_str(&self.set_pos(self.current_pos - 1));

            if num > 0 {
                output.push_str(&"+".repeat(second_closest as usize));
            } else if num < 0 {
                output.push_str(&"-".repeat(second_closest as usize));
            }
            output.push_str(&self.set_pos(self.current_pos + 1));
            output.push_str("-]");
            output.push_str(&self.set_pos(self.current_pos - 1));
            output.push_str(&"+".repeat(additional as usize));
        }
        output
    }

    fn multiply_current(&mut self, num: i16) -> String {
        let mut output = String::new();
        output.push_str(&self.move_to(self.current_pos + 1));
        output.push_str(">[<");
        output.push_str(&"+".repeat(num as usize));
        output.push_str(">-]<");
        output
    }

    fn multiply_positions(&mut self, pos1: i16, pos2: i16) -> String {
        let mut output = String::new();
        let original_pos = self.current_pos;

        output.push_str(&self.set_pos(pos1));
        output.push_str(&self.duplicate_to(original_pos+2));
        output.push_str(&self.set_pos(pos2));
        output.push_str(&self.duplicate_to(original_pos+3));
        let pos1 = original_pos + 2;
        let pos2 = original_pos + 3;
        output.push_str(&self.set_pos(pos1));
        output.push_str("[");
        output.push_str(&self.set_pos(pos2));
        output.push_str("[");
        output.push_str(&self.set_pos(original_pos));
        output.push('+');
        output.push_str(&self.set_pos(original_pos + 1));
        output.push('+');
        output.push_str(&self.set_pos(pos2));
        output.push_str("-]");
        output.push_str(&self.set_pos(original_pos + 1));
        output.push('[');
        output.push_str(&self.set_pos(pos2));
        output.push('+');
        output.push_str(&self.set_pos(original_pos + 1));
        output.push('-');
        output.push(']');
        output.push_str(&self.set_pos(pos1));
        output.push('-');
        output.push(']');
        output.push_str(&self.set_pos(pos2));
        output.push_str(&Self::set(0));
        output.push_str(&self.set_pos(original_pos));
        
        output
    }

    fn set(num: i16) -> String {
        format!("[-]{}", &"+".repeat(num as usize))
    }

    fn set_pos(&mut self, pos: i16) -> String {
        let mut output = String::new();
        if pos > self.current_pos {
            output.push_str(&">".repeat((pos - self.current_pos) as usize));
        } else if pos < self.current_pos {
            output.push_str(&"<".repeat((self.current_pos - pos) as usize));
        }
        self.current_pos = pos;
        output
    }

    fn move_to(&mut self, target: i16) -> String {
        format!("[{}+{}<-]", self.set_pos(target), Self::negate_arrows(&self.set_pos(target)))
    }

    fn duplicate_to(&mut self, target: i16) -> String {
        let original_pos = self.current_pos;
        let mut output = String::new();
        output.push('[');
        output.push_str(&self.set_pos(target));
        output.push('+');
        output.push_str(&self.set_pos(self.current_pos + 1));
        output.push('+');
        output.push_str(&self.set_pos(original_pos));
        output.push_str("-]");
        output.push_str(&self.set_pos(target + 1));
        output.push('[');
        output.push_str(&self.set_pos(original_pos));
        output.push('+');
        output.push_str(&self.set_pos(target + 1));
        output.push_str("-]");
        output.push_str(&self.set_pos(original_pos));

        output
    }

    fn text(&mut self, raw_text: &str) -> String {
        let mut output = String::new();
        const ASCII_CLOSENESS: i16 = 20;
        let original_pos = self.current_pos;

        let mut group_values = Vec::<i16>::new();
        let mut char_to_group_pos = HashMap::<char, usize>::new();
        
        // Search for suitable group numbers
        for character in raw_text.chars() {
            let mut char_has_group = false;

            for (group_value_index, &group_value) in group_values.iter().enumerate() {
                if (group_value as i16 - character as i16).abs() <= ASCII_CLOSENESS {
                    char_has_group = true;

                    // Assign character's value to i
                    char_to_group_pos.insert(character, group_value_index);
                    break;
                }
            }
            // If suitable group number not found, create new group
            if !char_has_group {
                group_values.push(character as i16);
                char_to_group_pos.insert(character, group_values.len() - 1);
            }
        }

        // Printing setting group numbers
        for group_value in &group_values {
            output.push_str(&self.add(*group_value));
            output.push_str(&self.set_pos(self.current_pos + 1));
            if !self.short_mode {
                output.push_str(&format!(" // Initialize {}\n", group_value.to_string().to_string()));
            }
        }

        // Printing
        for character in raw_text.chars().to_owned() {
            let group_pos = char_to_group_pos[&character];
            
            output.push_str(&self.set_pos(group_pos as i16 + original_pos));
            let diff_between_chars = character as i16 - group_values[group_pos];

            group_values[group_pos] += diff_between_chars; // Set group value to right num
            // Print
            if diff_between_chars > 0 {
                output.push_str(&"+".repeat(diff_between_chars as usize));
                if !self.short_mode {
                    output.push_str(&format!(" // Add {}\n", diff_between_chars.to_string()));
                }
            } else if diff_between_chars < 0 {
                output.push_str(&"-".repeat(diff_between_chars.abs() as usize));
                if !self.short_mode {
                    output.push_str(&format!(" // Subtract {}\n", (-diff_between_chars).to_string()));
                }
            }

            output.push_str(".");

            if !self.short_mode {
                output.push_str(" // Print\n");
            }
        }

        // Delete group numbers
        for i in (original_pos..=self.current_pos).rev() {
            output.push_str(&self.set_pos(i));
            output.push_str(&Self::set(0));
        }

        if !self.short_mode {
            output.push_str(" // Reset initial value to zero\n");
        }

        output
    
    }


    fn cleanup_bf(input: &str) -> String{
        let mut input = input.to_owned();
        while input.contains("<>") || input.contains("><") {
            input = input.replace("><", "").replace("<>", "");
        }
        input
    }

    fn process_command(&mut self, command: &str) -> String {
        let mut output = String::new();
        let mut parts = command.split_whitespace();
        if let Some(action) = parts.next() {
            match action {
                "ADD" => {
                    let num = parts.next().unwrap().parse::<i16>().unwrap();
                    output.push_str(&self.add(num));
                },
                "MULT" => {
                    let num1 = parts.next().unwrap().parse::<i16>().unwrap();
                    if let Some(num2) = parts.next() {
                        output.push_str(&self.multiply_positions(num1, num2.parse::<i16>().unwrap()));
                    } else {
                        output.push_str(&self.multiply_current(num1));
                    }
                },
                "GOTO" => {
                    let num_str = parts.next().unwrap();
                    if let Ok(num) = num_str.parse::<i16>() {
                        output.push_str(&self.set_pos(num));
                    } else {
                        if num_str == "RIGHT" {
                            output.push_str(&self.set_pos(self.current_pos + 1));
                        } else if num_str == "LEFT" {
                            output.push_str(&self.set_pos(self.current_pos - 1));
                        }
                    }
                },
                "SET" => {
                    let num = parts.next().unwrap().parse::<i16>().unwrap();
                    output.push_str(&self.set_pos(num));
                },
                "MOVE" => {
                    let num_str = parts.next().unwrap();
                    if let Ok(num) = num_str.parse::<i16>() {
                        output.push_str(&self.set_pos(num));
                    } else {
                        if num_str == "RIGHT" {
                            output.push_str(&self.move_to(self.current_pos + 1));
                        } else if num_str == "LEFT" {
                            output.push_str(&self.move_to(self.current_pos - 1));
                        }
                    }
                },
                "CLONE" => {
                    let num_str = parts.next().unwrap();
                    if let Ok(num) = num_str.parse::<i16>() {
                        output.push_str(&self.set_pos(num));
                    } else {
                        if num_str == "RIGHT" {
                            output.push_str(&self.duplicate_to(self.current_pos + 1));
                        } else if num_str == "LEFT" {
                            output.push_str(&self.duplicate_to(self.current_pos - 1));
                        }
                    }
                },
                "TEXT" => {
                    let text = parts.collect::<Vec<&str>>().join(" ");
                    output = self.text(&text);
                },
                "__" => {
                    let text = parts.collect::<Vec<&str>>().join(" ");
                    output.push_str(&text);
                },
                "//" => {}
                _ => {eprintln!("Unknown command: {}", action)}
            }
        }
        Self::cleanup_bf(&output)
    }

    fn execute_from_file(&mut self, file_path: &str) -> String {
        let mut output = String::new();
        if let Ok(file) = File::open(file_path) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    output.push_str(&self.process_command(&line));
                }
            }
        } else {
            eprintln!("Failed to open file: {}", file_path)
        }
        output
    }
}

fn main() {
    let mut bf = BrainFun::new(true);
    println!("{}", bf.execute_from_file("./code.rf"));
}
