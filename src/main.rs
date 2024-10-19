use std::collections::LinkedList;
use std::fs;
use std::io::Read;
use std::str::Chars;

const ALLOWED_CHARS: [char; 8] = ['+', '-', '[', ']', '>', '<', ',', '.'];

#[derive(PartialEq, Debug)]
enum Command {
    Plus,
    Minus,
    LeftLoop,
    RightLoop,
    Left,
    Right,
    Output,
    Input,
}

#[derive(Debug)]
struct CollapsedCommands {
    command: Command,
    amount: usize,
}

struct Lexer<'a> {
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    fn new(contents: &'a str) -> Lexer<'a> {
        Lexer {
            chars: contents.chars(),
        }
    }

    fn next_char(&mut self) -> Option<char> {
        while let Some(current_char) = self.chars.next() {
            if ALLOWED_CHARS.contains(&current_char) {
                return Some(current_char);
            }
        }
        None
    }
}

fn read_file(file_path: &str) -> String {
    let mut file = fs::File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    contents
}

fn parse_input(input: &str) -> Vec<CollapsedCommands> {
    let mut lexer = Lexer::new(&input);
    let mut commands: Vec<CollapsedCommands> = Vec::new();
    let mut loop_stack: LinkedList<usize> = LinkedList::new();
    while let Some(current_char) = lexer.next_char() {
        let command = match current_char {
            '+' => Command::Plus,
            '-' => Command::Minus,
            '[' => Command::LeftLoop,
            ']' => Command::RightLoop,
            '>' => Command::Right,
            '<' => Command::Left,
            '.' => Command::Output,
            ',' => Command::Input,
            _ => panic!("Invalid character"),
        };

        match command {
            Command::LeftLoop => {
                loop_stack.push_back(commands.len());
                commands.push(CollapsedCommands { command, amount: 1 });
            }
            Command::RightLoop => {
                let open_loop_index = loop_stack.pop_back().expect("no opening loop bracket");
                commands[open_loop_index].amount = commands.len();
                commands.push(CollapsedCommands {
                    command,
                    amount: open_loop_index,
                });
            }
            _ => {
                match commands.last_mut() {
                    Some(last_command) if command == last_command.command => {
                        last_command.amount += 1;
                    }
                    _ => {
                        commands.push(CollapsedCommands { command, amount: 1 });
                    }
                };
            }
        };
    }
    commands
}

fn process_input(commands: Vec<CollapsedCommands>) -> String {
    let mut memory = [0; 30_000];
    let mut memory_index = 0;
    let mut index = 0;
    let mut output = String::new();
    while index < commands.len() {
        let command = &commands[index];
        match command.command {
            Command::Plus => {
                memory[memory_index] += command.amount;
            }
            Command::Minus => {
                memory[memory_index] -= command.amount;
            }
            Command::LeftLoop if memory[memory_index] == 0 => {
                index = command.amount;
            }
            Command::LeftLoop => {}
            Command::RightLoop => {
                index = command.amount - 1;
            }
            Command::Right => {
                memory_index = (memory_index + command.amount) % 30_000;
            }
            Command::Left => {
                memory_index = (memory_index + 30_000 - command.amount) % 30_000;
            }
            Command::Output => {
                let mut i = 0;
                while i < command.amount {
                    output.push(memory[memory_index] as u8 as char);
                    i += 1;
                }
            }
            Command::Input => {
                // Handle input from the user
                let mut buffer = [0];
                std::io::stdin()
                    .read_exact(&mut buffer)
                    .expect("Failed to read input");
                memory[memory_index] = buffer[0] as usize;
            }
        }
        index += 1;
    }
    output
}

fn main() {
    let file_contents = read_file("src/testfile.bf");
    let commands: Vec<CollapsedCommands> = parse_input(&file_contents);
    let results = process_input(commands);
    println!("{}", results);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_good_weather_lexer() {
        const CONTENTS: &str = "-+.,[]<>";
        let expected_output: Vec<char> = vec!['-', '+', '.', ',', '[', ']', '<', '>'];
        let mut lexer = Lexer::new(CONTENTS);
        let mut output_array = Vec::new();
        while let Some(output) = lexer.next_char() {
            output_array.push(output);
        }

        assert_eq!(expected_output.to_vec(), output_array);
    }

    #[test]
    fn test_non_alphabet_filtered_out_lexer() {
        const CONTENTS: &str = "Hello world +- [blahblahblah] --> >< ,";
        let expected_output = vec!['+', '-', '[', ']', '-', '-', '>', '>', '<', ','];
        let mut lexer = Lexer::new(CONTENTS);
        let mut output_array = Vec::new();
        while let Some(output) = lexer.next_char() {
            output_array.push(output);
        }

        assert_eq!(expected_output.to_vec(), output_array);
    }

    #[test]
    fn test_process_simple_input() {
        let input: Vec<CollapsedCommands> = vec![
            CollapsedCommands {
                command: Command::Plus,
                amount: 65,
            },
            CollapsedCommands {
                command: Command::Output,
                amount: 1,
            },
        ];

        let output = process_input(input);
        assert_eq!(output, "A");
    }

    #[test]
    fn test_multiple_characters_print() {
        let input: Vec<CollapsedCommands> = vec![
            CollapsedCommands {
                command: Command::Plus,
                amount: 66,
            },
            CollapsedCommands {
                command: Command::Right,
                amount: 1,
            },
            CollapsedCommands {
                command: Command::Plus,
                amount: 65,
            },
            CollapsedCommands {
                command: Command::Left,
                amount: 1,
            },
            CollapsedCommands {
                command: Command::Right,
                amount: 1,
            },
            CollapsedCommands {
                command: Command::Output,
                amount: 1,
            },
            CollapsedCommands {
                command: Command::Left,
                amount: 1,
            },
            CollapsedCommands {
                command: Command::Output,
                amount: 2,
            },
        ];

        let output = process_input(input);
        assert_eq!(output, "ABB");
    }

    #[test]
    fn test_multiple_collapsed_jumps() {
        let input: Vec<CollapsedCommands> = vec![
            CollapsedCommands {
                command: Command::Plus,
                amount: 65,
            },
            CollapsedCommands {
                command: Command::Right,
                amount: 2,
            },
            CollapsedCommands {
                command: Command::Left,
                amount: 1,
            },
            CollapsedCommands {
                command: Command::Left,
                amount: 1,
            },
            CollapsedCommands {
                command: Command::Output,
                amount: 1,
            },
        ];

        let output = process_input(input);
        assert_eq!(output, "A");
    }
}
