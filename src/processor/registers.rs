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
    pub r1: i32,
    r2: i32,
    r3: i32,
    r4: i32,
    r5: i32,
    r6: i32,
    r7: i32,
    r8: i32,
    r9: i32,
    r10: i32,
    r11: i32,
    r12: i32,
    ip: i32,
    sp: i32,
    lr: i32,
    pc: u32,
    nzcv: CSPR,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            ip: 0,
            sp: 0,
            lr: 0,
            pc: 0,
            nzcv: CSPR::new(),
        }
    }
}
