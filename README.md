# RISC-V (RV32I) Emulator

**WARNING: STILL UNDER DEVELOPMENT! WOULDN'T EXPECT THIS TO BE FINISHED ANYTIME SOON. IT'S JUST A HOBBY PROJECT LOL.**

This is a software emulator for the 32-bit integer instruction set of the [RISC-V](https://riscv.org/) architecture written in Rust. It reads a compiled binary in RISC-V instruction format, loads it into simulated memory, and runs a fetch-decode-execute loop.

## Core-components

The following components will be implemented in the emulator:

### Memory Model: 
For memory a simulated byte array using Vec<u32> (Dynamic Memory Allocation) will be used
### CPU State: 
A struct holding 32 general-purpose registers ([u32; 32]), a program counter (PC)
### Execution Loop: 
A loop that fetches instructions from memory, decodes them, and executes them by modifying the CPU state accordingly.

**Note**: This emulator will not include pipeling, caching, or other performance optimizations. It will focus on correctness and simplicity.

## RV32I Instruction Set

### Core Fields

- **OPCODE (00-06) [07]** : instruction type
- **RD     (07-11) [05]** : destination register
- **FUNCT3 (12-14) [03]** : 3bit function modifier
- **RS1    (15-19) [05]** : source register 1
- **RS2    (20-24) [05]** : source register 2
- **FUNCT7 (25-31) [07]** : 7bit function modifier

### Instruction Fetching & Formating

- This architecture fetches instructions in 32-bit (4-byte) chunks, and all instructions must be aligned to 4 bytes. An instruction-address-misaligned exception is generated on a taken branch or unconditional jump if the target address is not IALIGN-bit aligned. For RV32I, IALIGN is 4, so the target address must be a multiple of 4. This exception is reported on the branch or jump instruction, not on the target instruction.

- The instruction format varies based on the type of instruction (R-type, I-type, S-type, U-type, B-type, J-type), and the fields are arranged differently for each type. The opcode field is always located in bits 0-6, while the other fields are positioned according to the instruction type.

- The following layout bits are depicted from MSB -> LSB (i.e., bit 31 is the most significant bit and bit 0 is the least significant bit):

<table>
  <tr>
    <th>Type</th>
    <th>31 - 25</th>
    <th>24 - 20</th>
    <th>19 - 15</th>
    <th>14 - 12</th>
    <th>11 - 07</th>
    <th>06 - 00</th>
  </tr>
</table>

#### 1. R-Type (Register-Register)
- Used in arithemtic and logical operations: ADD, SUB, AND, OR, SLL
<table>
  <tr>
    <td>R-Type</td>
    <td>FUNCT7</td>
    <td>RS2</td>
    <td>RS1</td>
    <td>FUNCT3</td>
    <td>RD</td>
    <td>OPCODE</td>
  </tr>
</table>

#### 2. I-Type (Register-Immediate)
- Used for immediate arithmetic and logical operations: ADDI, LW, JALR
<table>
  <tr>
    <td>I-Type</td>
    <td colspan="2">IMM[11:0]</td>
    <td>RS1</td>
    <td>FUNCT3</td>
    <td>RD</td>
    <td>OPCODE</td>
  </tr>
</table>

#### 3. S-Type (Store)
- Used for storing data from register to memory: SW, SH, SB
<table>
  <tr>
    <td>S-Type</td>
    <td>IMM[11:5]</td>
    <td>RS2</td>
    <td>RS1</td>
    <td>FUNCT3</td>
    <td>IMM[4:0]</td>
    <td>OPCODE</td>
  </tr>
</table>

#### 4. U-Type (Upper-Immediate)
- Used for loading 20-bit immediate values into the upper bits of a register: LUI, AUIPC
<table>
  <tr>
    <td>U-Type</td>
    <td colspan="4">IMM[31:12]</td>
    <td>RD</td>
    <td>OPCODE</td>
  </tr>
</table>

#### 5. B-Type (Branch)
- Used for conditional branching: BEQ, BNE, BLT, BGE
<table>
  <tr>
    <td>B-Type</td>
    <td>IMM[12 | 10:5]</td>
    <td>RS2</td>
    <td>RS1</td>
    <td>FUNCT3</td>
    <td>IMM[4:1 | 11]</td>
    <td>OPCODE</td>
  </tr>
</table>

#### 6. J-Type (Jump)
- Used for unconditional jumps: JAL
<table>
  <tr>
    <td>J-Type</td>
    <td colspan="4">IMM[20 | 10:1 | 11 | 19:12]</td>
    <td>RD</td>
    <td>OPCODE</td>
  </tr>
</table>

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
- **x10-11** : a0-a1 (argument registers/ return values)
- **x12-17** : a2-a7 (argument registers)
- **x18-27** : s2-s11 (saved registers)
- **x28-31** : t3-t6 (temporary registers)

## Assembly Syntax

### Instruction Syntax
R32I contains exactly 40 instructions.

#### R-Type: 
OP RD, RS1, RS2
- ADD  - Add 
- SUB  - Subtract	
- SLL	 - Shift Left Logical
- SLT	 - Set Less Than (signed)
- SLTU - Set Less Than (unsigned)
- XOR  - Exclusive OR 	
- SRL  - Shift Right Logical	
- SRA	 - Shift Right Arithmetic
- OR   - Inclusive OR 
- AND  - Logical AND	

#### I-Type: 
OP RD, RS1, IMM
- ADDI	 - Add Immediate
- SLTI   - Set Less Than Immediate (signed)	
- SLTIU  - Set Less Than Immediate (unsigned)
- XORI	 - XOR Immediate
- ORI	   - OR Immediate
- ANDI	 - AND Immediate
- SLLI   - Shift Left Logical Immediate
- SRLI   - Shift Right Logical Immediate
- SRAI   - Shift Right Arithmetic Immediate
- JALR   - Jump and Link Register
- LB     - Load Byte
- LH     - Load Halfword
- LW     - Load Word
- LBU    - Load Byte Unsigned
- LHU    - Load Halfword Unsigned
- FENCE  - Memory Ordering Fence
- ECALL  - Environment Call
- EBREAK - Environment Breakpoint

#### S-Type: 
OP RS2, IMM(RS1)
- SB - Store Byte
- SH - Store Halfword
- SW - Store Word

#### U-Type: 
OP RD, IMM
- LUI   - Load Upper Immediate
- AUIPC - Add Upper Immediate to PC

#### B-Type: 
OP RS1, RS2, IMM
- BEQ  - Branch if Equal
- BNE  - Branch if Not Equal
- BLT  - Branch if Less Than (signed)
- BGE  - Branch if Greater or Equal (signed)
- BLTU - Branch if Less Than (unsigned)
- BGEU - Branch if Greater or Equal (unsigned)

#### J-Type: 
OP RD, IMM
- JAL - Jump and Link
