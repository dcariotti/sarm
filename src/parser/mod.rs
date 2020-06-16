pub enum CommandType {
    Label,
    Mov,
    Add,
}

pub enum CommandArgsType {
    Nil,
    Label,
    NotLabel,
}

pub struct CommandArgs {
    arg_type: CommandArgsType,
    args_vec: Vec<String>,
}

pub struct Row {
    pub command: CommandType,
    pub args: CommandArgs,
}

impl Row {
    pub fn new() -> Self {
        let command = CommandArgs {
            arg_type: CommandArgsType::Nil,
            args_vec: Vec::<String>::new(),
        };

        Row {
            command: CommandType::Label,
            args: command,
        }
    }
}

#[test]
fn check_row() {
    let r: Row = Row {
        command: CommandType::Mov,
        args: CommandArgs {
            arg_type: CommandArgsType::NotLabel,
            args_vec: vec!["r0".to_string(), "r1".to_string()],
        },
    };
    assert_eq!(r.args.args_vec[0], "r0");
}

fn remove_spaces(line: String) -> String {
    let mut it = line.chars().peekable();
    let mut splitted_string = "".to_string();
    while let Some(&c) = it.peek() {
        match c {
            ' ' | '\t' => {
                it.next();
            }
            ch => {
                splitted_string.push(ch);
                it.next();
                if let Some(&t) = it.peek() {
                    match t {
                        ' ' | '\t' => {
                            splitted_string.push(' ');
                        }
                        ch => {
                            splitted_string.push(ch);
                        }
                    }
                }
                it.next();
            }
        }
    }

    splitted_string

}

pub fn parse_line(line: String) -> Result<Row, String> {
    let mut row = Row::new();

    let mut splitted_string = remove_spaces(line);

    splitted_string = splitted_string.to_lowercase();
    let mut words : Vec<&str> = splitted_string.split(" ").collect();

    if words.len() > 1 {
        if words[1].ends_with(",") {
            words[1] = words[1].trim_end_matches(',');
        }
    }

    println!("{}, {:?}", words.len(), words);

    Ok(row)
}
