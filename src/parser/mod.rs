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
