use itertools as it;
use std::iter::Range;

pub enum Block {
    Stone,
    Air,
}

pub struct ChunkIterator<'a> {
    chunk: &'a Chunk,
    prod: it::FlatTuples<it::Product<(uint, uint), it::Product<uint, Range<uint>, Range<uint>>, Range<uint>>>,
}

impl<'a> Iterator<((uint, uint, uint), Block)> for ChunkIterator<'a> {
    fn next(&mut self) -> Option<((uint, uint, uint), Block)> {
        match self.prod.next() {
            Some((x, y, z)) => Some(((x, y, z), self.chunk.data[x][y][z])),
            None => None,
        }
    }

}

pub struct Chunk {
    data: [[[Block, ..16], ..16], ..16],
}

impl Chunk {
    pub fn new() -> Chunk {
        let mut data = [[[Air, ..16], ..16], ..16];
        for (x, y, z) in iproduct!(range(0u, 16u), range(0u, 3u), range(0u, 16u)) {
            data[x][y][z] = Stone;
        }
        Chunk { data: data }
    }

    pub fn blocks<'a>(&'a self) -> ChunkIterator<'a> {
        ChunkIterator {
            chunk: self,
            prod: iproduct!(range(0u, 16u), range(0u, 16u), range(0u, 16u)),
        }
    }
}

