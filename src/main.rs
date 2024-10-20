use std::collections::{HashMap, LinkedList};
use std::fs;
use std::io::Read;
use std::str::Chars;

const ALLOWED_CHARS: [char; 8] = ['+', '-', '[', ']', '>', '<', ',', '.'];

#[derive(PartialEq, Debug, Clone)]
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

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
enum CompiledCommand {
    Plus,
    Minus,
    LeftLoop,
    RightLoop,
    Left,
    Right,
    Output,
    Input,
    SetZero,
}

#[derive(Debug, PartialEq, Clone)]
struct CollapsedCommands {
    command: Command,
    amount: usize,
}

#[derive(Debug, PartialEq)]
struct CompiledCollapsedCommand {
    command: CompiledCommand,
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

fn process_input(commands: Vec<CompiledCollapsedCommand>) -> String {
    let mut memory = [0u8; 30_000];
    let mut memory_index = 15_000;
    let mut index = 0;
    let mut output = Vec::new();
    //let mut processed_commands_map: HashMap<CompiledCommand, usize> = HashMap::new();
    while index < commands.len() {
        let command = &commands[index];
        //*processed_commands_map
        //.entry(command.command.clone())
        //.or_insert(0) += 1;
        match command.command {
            CompiledCommand::RightLoop => {
                if memory[memory_index] != 0 {
                    index = command.amount;
                }
            }
            CompiledCommand::Right => {
                memory_index += command.amount;
                if memory_index >= 30_000 {
                    memory_index = memory_index % 30_000;
                }
            }
            CompiledCommand::Left => {
                if memory_index < command.amount {
                    memory_index += 30_000 - command.amount;
                } else {
                    memory_index -= command.amount;
                }
            }
            CompiledCommand::LeftLoop => {
                if memory[memory_index] == 0 {
                    index = command.amount;
                }
            }
            CompiledCommand::Plus => {
                memory[memory_index] = memory[memory_index].wrapping_add(command.amount as u8);
            }
            CompiledCommand::Minus => {
                memory[memory_index] = memory[memory_index].wrapping_sub(command.amount as u8);
            }
            CompiledCommand::SetZero => {
                memory[memory_index] = 0;
                index += 2;
            }
            CompiledCommand::Output => {
                for _ in 0..command.amount {
                    output.push(memory[memory_index]);
                }
            }
            CompiledCommand::Input => {
                // Handle input from the user
                let mut buffer = [0];
                std::io::stdin()
                    .read_exact(&mut buffer)
                    .expect("Failed to read input");
                memory[memory_index] = buffer[0];
            }
        }
        index += 1;
    }
    //print!("{:?}", processed_commands_map);
    String::from_utf8(output).expect("Invalid UTF-8")
}

fn compile(commands: Vec<CollapsedCommands>) -> Vec<CompiledCollapsedCommand> {
    let mut compiled_commands = Vec::new();
    let mut index = 0;

    while index < commands.len() {
        let current_command = &commands[index];
        match current_command {
            CollapsedCommands {
                command: Command::LeftLoop,
                amount: _,
            } if index + 2 < commands.len()
                && commands[index + 1]
                    == CollapsedCommands {
                        command: Command::Minus,
                        amount: 1,
                    }
                && commands[index + 2].command == Command::RightLoop =>
            {
                compiled_commands.push(CompiledCollapsedCommand {
                    command: CompiledCommand::SetZero,
                    amount: 1,
                });
                compiled_commands.push(CompiledCollapsedCommand {
                    command: CompiledCommand::SetZero,
                    amount: 1,
                });
                compiled_commands.push(CompiledCollapsedCommand {
                    command: CompiledCommand::SetZero,
                    amount: 1,
                });
                index = index + 3;
            }
            CollapsedCommands {
                command: Command::LeftLoop,
                amount,
            } => {
                compiled_commands.push(CompiledCollapsedCommand {
                    command: CompiledCommand::LeftLoop,
                    amount: *amount,
                });
                index += 1;
            }
            CollapsedCommands {
                command: Command::RightLoop,
                amount,
            } => {
                compiled_commands.push(CompiledCollapsedCommand {
                    command: CompiledCommand::RightLoop,
                    amount: *amount,
                });
                index += 1;
            }
            CollapsedCommands {
                command: Command::Plus,
                amount,
            } => {
                compiled_commands.push(CompiledCollapsedCommand {
                    command: CompiledCommand::Plus,
                    amount: *amount,
                });
                index += 1;
            }
            CollapsedCommands {
                command: Command::Minus,
                amount,
            } => {
                compiled_commands.push(CompiledCollapsedCommand {
                    command: CompiledCommand::Minus,
                    amount: *amount,
                });
                index += 1;
            }
            CollapsedCommands {
                command: Command::Right,
                amount,
            } => {
                compiled_commands.push(CompiledCollapsedCommand {
                    command: CompiledCommand::Right,
                    amount: *amount,
                });
                index += 1;
            }
            CollapsedCommands {
                command: Command::Left,
                amount,
            } => {
                compiled_commands.push(CompiledCollapsedCommand {
                    command: CompiledCommand::Left,
                    amount: *amount,
                });
                index += 1;
            }
            CollapsedCommands {
                command: Command::Output,
                amount,
            } => {
                compiled_commands.push(CompiledCollapsedCommand {
                    command: CompiledCommand::Output,
                    amount: *amount,
                });
                index += 1;
            }
            CollapsedCommands {
                command: Command::Input,
                amount,
            } => {
                compiled_commands.push(CompiledCollapsedCommand {
                    command: CompiledCommand::Input,
                    amount: *amount,
                });
                index += 1;
            }
        }
    }

    compiled_commands
}

fn main() {
    let start = std::time::Instant::now();

    let file_read_start = std::time::Instant::now();
    let file_contents = read_file("src/testfile.bf");
    let file_read_duration = file_read_start.elapsed();

    let parse_start = std::time::Instant::now();
    let commands: Vec<CollapsedCommands> = parse_input(&file_contents);
    let parse_duration = parse_start.elapsed();

    let compile_start = std::time::Instant::now();
    let compiled_commands = compile(commands);
    let compile_duration = compile_start.elapsed();

    let execution_start = std::time::Instant::now();
    let results = process_input(compiled_commands);
    let execution_duration = execution_start.elapsed();

    let duration = start.elapsed();

    println!("{}", results);
    println!("File read duration: {:?}", file_read_duration);
    println!("Parse duration: {:?}", parse_duration);
    println!("Compile duration: {:?}", compile_duration);
    println!("Execution duration: {:?}", execution_duration);
    println!("Time elapsed: {:?}", duration);
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
        let input: Vec<CompiledCollapsedCommand> = vec![
            CompiledCollapsedCommand {
                command: CompiledCommand::Plus,
                amount: 65,
            },
            CompiledCollapsedCommand {
                command: CompiledCommand::Output,
                amount: 1,
            },
        ];

        let output = process_input(input);
        assert_eq!(output, "A");
    }

    #[test]
    fn test_multiple_characters_print() {
        let input: Vec<CompiledCollapsedCommand> = vec![
            CompiledCollapsedCommand {
                command: CompiledCommand::Plus,
                amount: 66,
            },
            CompiledCollapsedCommand {
                command: CompiledCommand::Right,
                amount: 1,
            },
            CompiledCollapsedCommand {
                command: CompiledCommand::Plus,
                amount: 65,
            },
            CompiledCollapsedCommand {
                command: CompiledCommand::Left,
                amount: 1,
            },
            CompiledCollapsedCommand {
                command: CompiledCommand::Right,
                amount: 1,
            },
            CompiledCollapsedCommand {
                command: CompiledCommand::Output,
                amount: 1,
            },
            CompiledCollapsedCommand {
                command: CompiledCommand::Left,
                amount: 1,
            },
            CompiledCollapsedCommand {
                command: CompiledCommand::Output,
                amount: 2,
            },
        ];

        let output = process_input(input);
        assert_eq!(output, "ABB");
    }

    #[test]
    fn test_multiple_collapsed_jumps() {
        let input: Vec<CompiledCollapsedCommand> = vec![
            CompiledCollapsedCommand {
                command: CompiledCommand::Plus,
                amount: 65,
            },
            CompiledCollapsedCommand {
                command: CompiledCommand::Right,
                amount: 2,
            },
            CompiledCollapsedCommand {
                command: CompiledCommand::Left,
                amount: 1,
            },
            CompiledCollapsedCommand {
                command: CompiledCommand::Left,
                amount: 1,
            },
            CompiledCollapsedCommand {
                command: CompiledCommand::Output,
                amount: 1,
            },
        ];

        let output = process_input(input);
        assert_eq!(output, "A");
    }

    #[test]
    fn test_set_zero() {
        let input: Vec<CollapsedCommands> = vec![
            CollapsedCommands {
                command: Command::LeftLoop,
                amount: 1,
            },
            CollapsedCommands {
                command: Command::Minus,
                amount: 1,
            },
            CollapsedCommands {
                command: Command::RightLoop,
                amount: 1,
            },
        ];
        let expected_output: Vec<CompiledCollapsedCommand> = vec![CompiledCollapsedCommand {
            command: CompiledCommand::SetZero,
            amount: 1,
        }];

        let compiled_input = compile(input);
        assert_eq!(compiled_input, expected_output);
    }

    #[test]
    fn test_complex_set_zero() {
        let input: Vec<CollapsedCommands> = vec![
            CollapsedCommands {
                command: Command::LeftLoop,
                amount: 1,
            },
            CollapsedCommands {
                command: Command::LeftLoop,
                amount: 1,
            },
            CollapsedCommands {
                command: Command::Minus,
                amount: 1,
            },
            CollapsedCommands {
                command: Command::RightLoop,
                amount: 1,
            },
            CollapsedCommands {
                command: Command::RightLoop,
                amount: 1,
            },
        ];

        let expected_output: Vec<CompiledCollapsedCommand> = vec![
            CompiledCollapsedCommand {
                command: CompiledCommand::LeftLoop,
                amount: 1,
            },
            CompiledCollapsedCommand {
                command: CompiledCommand::SetZero,
                amount: 1,
            },
            CompiledCollapsedCommand {
                command: CompiledCommand::RightLoop,
                amount: 1,
            },
        ];
        let compiled_input = compile(input);
        assert_eq!(compiled_input, expected_output);
    }

    #[test]
    fn test_loops_correctly_parsed() {
        let input = "++[->+<]";
        let expected_output: Vec<CollapsedCommands> = vec![
            CollapsedCommands {
                command: Command::Plus,
                amount: 2,
            },
            CollapsedCommands {
                command: Command::LeftLoop,
                amount: 7,
            },
            CollapsedCommands {
                command: Command::Minus,
                amount: 1,
            },
            CollapsedCommands {
                command: Command::Right,
                amount: 1,
            },
            CollapsedCommands {
                command: Command::Plus,
                amount: 1,
            },
            CollapsedCommands {
                command: Command::Left,
                amount: 1,
            },
            CollapsedCommands {
                command: Command::RightLoop,
                amount: 2,
            },
        ];

        let parsed_input = parse_input(input);
        assert_eq!(parsed_input, expected_output);
    }
}
