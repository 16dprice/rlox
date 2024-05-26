mod chunk;
mod debug;
mod value;

use chunk::{Chunk, OpCode};
use debug::disassemble_chunk;

fn main() {
    let mut first_chunk = Chunk::new();

    first_chunk.write_code(OpCode::Constant as u8, 1);

    let constant_index = first_chunk.write_constant(1.5);
    first_chunk.write_code(constant_index as u8, 1);

    first_chunk.write_code(OpCode::Negate as u8, 1);

    disassemble_chunk(&first_chunk, "First Chunk!");
}
