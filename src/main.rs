use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    println!(
        "{:?}",
        simple_assembler(vec![
            "mov c 12",
            "mov b 0",
            "mov a 200",
            "dec a",
            "inc b",
            "jnz a -2",
            "dec c",
            "mov a b",
            "jnz c -5",
            "jnz 0 1",
            "mov c a",
        ])
    );
}

fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {
    VM::new(program).evaluate()
}

#[derive(Clone, Debug)]
enum Instruction {
    Mov(String, Value),
    Inc(String),
    Dec(String),
    Jnz(Value, Value),
    Unsupported,
}

#[derive(Clone, Debug)]
enum Value {
    Number(i64),
    Register(String),
}

struct VM {
    stack_pointer: i64,
    instructions: Vec<Instruction>,
    registers: HashMap<String, i64>,
}

impl From<&String> for Value {
    fn from(value: &String) -> Self {
        if let Ok(res) = i64::from_str(value) {
            Value::Number(res)
        } else {
            Value::Register(value.clone())
        }
    }
}

#[derive(Clone, Debug)]
struct RawInstruction(Instruction, Register);

impl From<Vec<String>> for RawInstruction {
    fn from(raw: Vec<String>) -> Self {
        let source = &raw[1];
        let instruction = raw.first().map(AsRef::as_ref);
        let instr = if let Some("mov") = instruction {
            Instruction::Mov(source.clone(), (&raw[2]).into())
        } else if let Some("inc") = instruction {
            Instruction::Inc(source.clone())
        } else if let Some("dec") = instruction {
            Instruction::Dec(source.clone())
        } else if let Some("jnz") = instruction {
            Instruction::Jnz((&raw[1]).into(), (&raw[2]).into())
        } else {
            Instruction::Unsupported
        };
        RawInstruction(instr, Register(source.clone(), 0))
    }
}

impl VM {
    fn new(raw_instructions: Vec<&str>) -> Self {
        let instructions = raw_instructions
            .into_iter()
            .map(|elements| elements.to_lowercase())
            .map(|instr| {
                instr
                    .split_ascii_whitespace()
                    .map(ToOwned::to_owned)
                    .collect::<Vec<_>>()
            })
            .map(Into::<RawInstruction>::into)
            .fold((vec![], HashMap::new()), |mut acc, raw_instr| {
                acc.0.push(raw_instr.0);
                if i64::from_str(&(raw_instr.1).0).is_err() {
                    acc.1.insert((raw_instr.1).0, (raw_instr.1).1);
                }
                acc
            });
        println!("{:?}", instructions);
        Self {
            stack_pointer: 0,
            instructions: instructions.0,
            registers: instructions.1,
        }
    }

    fn evaluate(&mut self) -> HashMap<String, i64> {
        while (self.stack_pointer as usize) < self.instructions.len() {
            let instr = self
                .instructions
                .get(self.stack_pointer as usize)
                .unwrap()
                .clone();
            instr.evaluate(self);
        }
        self.registers.clone()
    }
}

#[derive(Clone, Debug)]
struct Register(String, i64);

impl Instruction {
    fn evaluate(&self, vm: &mut VM) {
        if let Instruction::Mov(register, value) = self {
            vm.stack_pointer += 1;
            let val = Self::get_value(value, vm);
            vm.registers.insert(register.clone(), val);
        } else if let Instruction::Inc(register) = self {
            vm.stack_pointer += 1;
            vm.registers
                .insert(register.clone(), vm.registers[register] + 1);
        } else if let Instruction::Dec(register) = self {
            vm.stack_pointer += 1;
            vm.registers
                .insert(register.clone(), vm.registers[register] - 1);
        } else if let Instruction::Jnz(register, instr) = self {
            let value = Self::get_value(register, vm);
            let value_jump = Self::get_value(instr, vm);
            if value != 0 {
                vm.stack_pointer += value_jump
            } else {
                vm.stack_pointer += 1
            }
        }
    }

    #[inline]
    fn get_value(value: &Value, vm: &mut VM) -> i64 {
        match value {
            Value::Number(num) => *num,
            Value::Register(register) => vm.registers[register],
        }
    }
}
