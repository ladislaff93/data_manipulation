


struct CPU {
    registers: [u8;16], // one hexadecimal num can access all position of register 0-F, so all opcodes fit into u16  0000 0000->register 0000->register 0000
    position_in_memory: usize, // pointer on the memory
    memory: [u8; 0x1000], // 4096
    stack_pointer: usize, // pointer on the stack
    stack: [u16; 16]
}
impl CPU {
    fn new() -> Self {
        let mut cpu = CPU {
            position_in_memory: 0,  
            registers: [0;16], // registers are boxes for storing data 
            memory: [0;4096],
            stack_pointer: 0,
            stack: [0;16]
        };
        cpu.registers[0xF] = 0;
        cpu
    }

    fn read_opcodes(&self) -> u16 { // opcode = operation code
        let pointer = self.position_in_memory; 
        let op_byte_1 = self.memory[pointer] as u16;
        let op_byte_2 = self.memory[pointer+1] as u16;
        (op_byte_1 << 8) | op_byte_2
    }

    fn add_xy(&mut self, x:u8, y:u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize]; // add value from register to other register
        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;
        if overflow {
            self.registers[0xF] = 1;
        }
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcodes();
            self.position_in_memory += 2;

            let c = (opcode & 0xF000) >> 12;
            let x = (opcode & 0x0F00) >> 8;
            let y = (opcode & 0x00F0) >> 4;
            let d = (opcode & 0x000F) >> 0;

            let nnn = opcode & 0x0FFF;
            // let kk = opcode & 0x00FF;

            match opcode {
                0x0000 => {return; },
                0x00EE => self.return_(), // execute the return opcode
                0x8000..=0x8FFF => self.add_xy(x as u8, y as u8), // c == opcode group, x == register, y == register, d == opcode subtype
                0x2000..=0x2FFF => self.call(nnn), // execute the load memory into stack
                _ => todo!("opcode {:04x}", opcode)
            };
        }
    }
    fn call(&mut self, addr: u16) {
        if self.stack_pointer > self.stack.len() {
            panic!("stack overflow")
        }
        self.stack[self.stack_pointer] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
        println!("Stack LookUp After Call Operation: {:?}", self.stack)
    }

    fn return_(&mut self) {
        if self.stack_pointer == self.stack.len() {
            panic!("stack underflow")
        }
        self.stack[self.stack_pointer] = 0; // clean up the stack position
        self.stack_pointer -= 1;
        self.position_in_memory = self.stack[self.stack_pointer] as usize;
        println!("Stack LookUp After Return Operation: {:?}", self.stack)
    }

}

fn main() {
    let mut cpu = CPU::new(); // we initialize the CPU
    cpu.registers[0] = 5; 
    cpu.registers[1] = 10; // load the variables into the register
    
    let fn_add_twice_a_b = {
        cpu.memory[0x000]=0x21; cpu.memory[0x001]=0x00; // jump to the address 100 and load it onto stack
        cpu.memory[0x002]=0x21; cpu.memory[0x003]=0x00; // jump to the address 100 and load it onto stack
        cpu.memory[0x004]=0x00; cpu.memory[0x005]=0x00; // func return

        cpu.memory[0x100]=0x80; cpu.memory[0x101]=0x14; // execute the additing
        cpu.memory[0x102]=0x80; cpu.memory[0x103]=0x14; // execute the additing
        cpu.memory[0x104]=0x00; cpu.memory[0x105]=0xEE; // stack return  
    };

    cpu.run(); // execute instructions
    assert!(cpu.registers[0] == 45); // check the result
}