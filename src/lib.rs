#[derive(Debug)]
struct RingBuffer {
    buffer: Vec<char>,
    len: usize,
    pos: usize,
}

#[derive(Debug)]
struct BackCursor {
    idx: usize,
    len: usize,
}

impl Default for BackCursor {
    fn default() -> Self {
        BackCursor { idx: 0, len: 1024 }
    }
}

impl BackCursor {
    fn next(&mut self) -> usize {
        self.idx += self.len - 1;
        self.idx &= self.len - 1;
        self.idx
    }
}

impl RingBuffer {
    fn new(n: usize) -> Self {
        let mut len = 2;
        while len < n {
            len += len;
        }
        let pos = 0;
        let buffer = vec![' '; len];
        RingBuffer { buffer, len, pos }
    }

    fn insert(&mut self, ch: char) {
        self.buffer[self.pos] = ch;
        self.pos += 1;
        self.pos &= self.len - 1;
    }

    fn cursor(&self) -> BackCursor {
        BackCursor {
            idx: self.pos,
            len: self.len,
        }
    }

    fn get(&self, index: usize) -> char {
        self.buffer[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_ring() {
        let mut ring = RingBuffer::new(3);
        ring.insert('a');
        ring.insert('b');
        ring.insert('c');
        let mut cursor = ring.cursor();
        assert_eq!(ring.get(cursor.next()), 'c');
        assert_eq!(ring.get(cursor.next()), 'b');
        assert_eq!(ring.get(cursor.next()), 'a');
        assert_eq!(ring.get(cursor.next()), ' ');
        ring.insert('d');
        let mut cursor = ring.cursor();
        assert_eq!(ring.get(cursor.next()), 'd');
        assert_eq!(ring.get(cursor.next()), 'c');
        assert_eq!(ring.get(cursor.next()), 'b');
        assert_eq!(ring.get(cursor.next()), 'a');
        ring.insert('x');
        let mut cursor = ring.cursor();
        assert_eq!(ring.get(cursor.next()), 'x');
        assert_eq!(ring.get(cursor.next()), 'd');
        assert_eq!(ring.get(cursor.next()), 'c');
        assert_eq!(ring.get(cursor.next()), 'b');
    }
}
