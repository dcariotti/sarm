pub struct CSPR {
    n: bool,
    z: bool,
    c: bool,
    v: bool,
}

impl CSPR {
    pub fn new() -> Self {
        CSPR {
            n: false,
            z: false,
            c: false,
            v: false,
        }
    }
}

pub struct Registers {
    pub r : [i32; 12],
    ip: i32,
    sp: i32,
    lr: i32,
    pc: u32,
    nzcv: CSPR,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            r: [0; 12],
            ip: 0,
            sp: 0,
            lr: 0,
            pc: 0,
            nzcv: CSPR::new(),
        }
    }
}
