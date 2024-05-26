mod chunk;

use chunk::{Chunk, OpCode};

fn main() {
    let mut first_chunk = Chunk::new();

    first_chunk.write_chunk(OpCode::Constant, 1);
}
