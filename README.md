# RISC-V (RV32I) Emulator

**WARNING: STILL UNDER DEVELOPMENT! WOULDN'T EXPECT THIS TO BE FINISHED ANYTIME SOON. IT'S JUST A HOBBY PROJECT LOL.**

This is a software emulator for the 32-bit integer instruction set of the [RISC-V](https://riscv.org/) architecture written in Rust. It reads a compiled binary in RISC-V instruction format, loads it into simulated memory, and runs a fetch-decode-execute loop.

## Core-components

The following components will be implemented in the emulator:

### Memory Model: 
For memory a simulated byte array using Vec<u8> (Dynamic Memory Allocation) will be used
### CPU State: 
A struct holding 32 general-purpose registers ([u32; 32]), a program counter (PC), and memory references
### Execution Loop: 
Reading 32-bit instructions, extracting opcodes and operands via bitwise masking, and executing the corresponding logic.

**Note**: This emulator will not include pipeling, caching, or other performance optimizations. It will focus on correctness and simplicity.

## RV32I Instruction Set

### Core Fields

- **OPCODE (00-06)** : instruction type
- **RD     (07-11)** : destination register
- **FUNCT3 (12-14)** : 3bit function modifier
- **RS1    (15-19)** : source register 1
- **RS2    (20-24)** : source register 2
- **FUNCT7 (25-31)** : 7bit function modifier

### Instruction Fetching & Formating

- This architecture fetches instructions in 32-bit (4-byte) chunks, and all instructions must be aligned to 4 bytes. An instruction-address-misaligned exception is generated on a taken branch or unconditional jump if the target address is not IALIGN-bit aligned. For RV32I, IALIGN is 4, so the target address must be a multiple of 4. This exception is reported on the branch or jump instruction, not on the target instruction.

- The instruction format varies based on the type of instruction (R-type, I-type, S-type, U-type, B-type, J-type), and the fields are arranged differently for each type. The opcode field is always located in bits 0-6, while the other fields are positioned according to the instruction type.

- The following layout bits are depicted from MSB -> LSB (i.e., bit 31 is the most significant bit and bit 0 is the least significant bit):

#### 1. R-Type (Register-Register)
- Used in arithemtic and logical operations: ADD, SUB, AND, OR, SLL
- **LAYOUT**: `| FUNCT7 _____ | RS2 | RS1 | FUNCT3 | RD ________ | OPCODE |`

#### 2. I-Type (Register-Immediate)
- Used for immediate arithmetic and logical operations: ADDI, LW, JALR
- **LAYOUT**: `| IMM[11:0] ________ | RS1 | FUNCT3 | RD ________ | OPCODE |`

#### 3. S-Type (Store)
- Used for storing data from register to memory: SW, SH, SB
- **LAYOUT**: `| IMM[11:5] __ | RS2 | RS1 | FUNCT3 | IMM[4:0] __ | OPCODE |`

#### 4. U-Type (Upper-Immediate)
- Used for loading 20-bit immediate values into the upper bits of a register: LUI, AUIPC
- **LAYOUT**: `| IMM[31:12] ______________________ | RD ________ | OPCODE |`

#### 5. B-Type (Branch)
- Used for conditional branching: BEQ, BNE, BLT, BGE
- **LAYOUT**: `| IMM[12|10:5] | RS2 | RS1 | FUNCT3 | IMM[4:1|11] | OPCODE |`

#### 6. J-Type (Jump)
- Used for unconditional jumps: JAL
- **LAYOUT**: `| IMM[20|10:1|11|19:12] ___________ | RD ________ | OPCODE |`

## Register Conventions

### Integer Registers

In accordance with RISC-V RV32I ABI conventions, the 32 general-purpose registers have specific aliases for better readability:

- **x0**     : zero
- **x1**     : ra (return address)
- **x2**     : sp (stack pointer)
- **x3**     : gp (global pointer)
- **x4**     : tp (thread pointer)
- **x5-7**   : t0-t2 (temporary registers)
- **x8**     : s0/fp (frame pointer)
- **x9**     : s1 (saved register 1)
- **x10-17** : a0-a7 (argument registers)
- **x18-27** : s2-s11 (saved registers)
- **x28-31** : t3-t6 (temporary registers)

## Assembly Syntax

### Instruction Syntax
#### R-Type: 
OP RD, RS1, RS2
- `ADD x5, x1, x2`

#### I-Type: 
OP RD, RS1, IMM
- `ADDI x5, x1, 10`

#### S-Type: 
OP RS2, IMM(RS1)
- `SW x5, 0(x1)`

#### U-Type: 
OP RD, IMM
- `LUI x5, 0x12345`

#### B-Type: 
OP RS1, RS2, IMM
- `BEQ x1, x2, 16`

#### J-Type: 
OP RD, IMM
- `JAL x1, 32`