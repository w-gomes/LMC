use std::io;

// Little Man Computer simulator.

#[derive(Debug)]
enum OpCode {
    Add,
    Sub,
    Sta,
    Lda,
    Bra,
    Brz,
    Brp,
    Inp,
    Out,
    Hlt,
}

#[derive(Debug)]
enum Flag {
    Neg,
    Pos,
}

#[derive(Debug)]
struct Computer {
    mem: Vec<i16>,
    acc: i16,
    inp: i16,
    out: i16,
    pc: i16,
    flag: Flag,
}

impl Computer {
    fn new() -> Self {
        Computer {
            mem: vec![0; 100],
            acc: 0,
            inp: 0,
            out: 0,
            pc: 0,
            flag: Flag::Pos,
        }
    }

    fn set_flag(&mut self) {
        if self.acc < 0 {
            self.flag = Flag::Neg;
        } else {
            self.flag = Flag::Pos;
        }
    }

    fn get_input(&mut self) {
        println!("{}", "Enter the input: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input.");
        if let Ok(input) = input.trim().parse::<i16>() {
            self.acc = input;
        }
    }

    fn output(&self) {
        println!("Output: {}", self.acc);
    }

    fn run(&mut self) {
        loop {
            let (op, mem) = self.get_instr(self.mem[self.pc as usize]);
            self.pc += 1;

            match op {
                OpCode::Add => {
                    self.acc += self.mem[mem as usize] % 999;
                    self.set_flag();
                }
                OpCode::Sub => {
                    self.acc -= self.mem[mem as usize];
                    self.set_flag();
                }
                OpCode::Sta => self.mem[mem as usize] = self.acc,
                OpCode::Lda => self.acc = self.mem[mem as usize],
                OpCode::Bra => self.pc = mem,
                OpCode::Brz => {
                    if self.acc == 0 {
                        self.pc = mem;
                    }
                }
                OpCode::Brp => match self.flag {
                    Flag::Pos => self.pc = mem,
                    _ => {}
                },
                OpCode::Inp => self.get_input(),
                OpCode::Out => self.output(),
                OpCode::Hlt => {
                    println!("Computer is halted.");
                    break;
                }
            }
        }
    }

    fn get_instr(&self, ins: i16) -> (OpCode, i16) {
        let temp_op = ins as f32 / 100.0;
        let op = temp_op.floor() as i16;
        let mem = ins - (op * 100);

        let opcode = match op {
            1 => OpCode::Add,
            2 => OpCode::Sub,
            3 => OpCode::Sta,
            5 => OpCode::Lda,
            6 => OpCode::Bra,
            7 => OpCode::Brz,
            8 => OpCode::Brp,
            9 => match mem {
                1 => OpCode::Inp,
                2 => OpCode::Out,
                _ => panic!("Bad Opcode"),
            },
            0 => OpCode::Hlt,
            _ => panic!("Bad Opcode"),
        };

        (opcode, mem)
    }

    fn load_ins(&mut self, insts: Vec<i16>) {
        let mut index = 0;

        for elem in &insts {
            self.mem[index] = *elem;
            index += 1;
        }
    }
}

fn main() {
    // computes the difference between two numbers
    let insts1 = vec![901, 308, 901, 309, 508, 209, 902, 000];

    // computes the highest number between three numbers...?
    let insts2 = vec![
        901, 318, 901, 319, 901, 320, 219, 810, 519, 320, 520, 218, 815, 518, 320, 520, 902, 000,
    ];

    // computes the highest number between two numbers
    let insts3 = vec![901, 310, 901, 210, 807, 510, 608, 110, 902, 000];

    let mut comp = Computer::new();
    comp.load_ins(insts2);
    comp.run();
}
