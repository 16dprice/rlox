use crate::chunk::{Chunk, OpCode};
use crate::value::print_value;

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("CHUNK OFFSET - {:0>4} | ", offset);
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("LINE -    | ");
    } else {
        print!("LINE - {:0>4} ", chunk.lines[offset]);
    }

    let instruction = chunk.code[offset];

    if instruction == OpCode::Return as u8 {
        println!("RETURN");
        return offset + 1;
    } else if instruction == OpCode::Constant as u8 {
        let constant = &chunk.constants[chunk.code[offset + 1] as usize];
        print!("'");
        print_value(constant);
        println!("'");

        return offset + 2;
    } else {
        println!("Unkown opcode {:0>4}", instruction);
        return offset + 1;
    }

    // printf("CHUNK OFFSET - %04d | ", offset);

    // if (offset > 0 && chunk->lines[offset] == chunk->lines[offset - 1])
    // {
    //     printf("LINE -    | ");
    // }
    // else
    // {
    //     printf("LINE - %04d ", chunk->lines[offset]);
    // }
}

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

// void disassembleChunk(Chunk *chunk, const char *name)
// {
//     printf("== %s ==\n", name);

//     for (int offset = 0; offset < chunk->count;)
//     {
//         offset = disassembleInstruction(chunk, offset);
//     }
// }