use std::collections::HashMap;

///
/// Trie
///
#[derive(Default, Debug)]
struct Trie {
    is_leaf: bool,
    children: HashMap<char, Trie>,
}

impl Trie {
    fn insert_str(&mut self, key: &str) {
        let mut node = self;
        // inverse as data stream got reversely
        for ch in key.chars().rev() {
            if !node.children.contains_key(&ch) {
                node.children.insert(ch, Trie::default());
            }
            node = node.children.get_mut(&ch).unwrap();
        }
        node.is_leaf = true;
    }

    fn insert(&mut self, key: String) {
        self.insert_str(&key);
    }

    fn query_str(&self, key: &str) -> bool {
        let mut node = self;
        for ch in key.chars().rev() {
            let n = node.children.get(&ch);
            if let Some(node) = n {
                if node.is_leaf {
                    return true;
                }
            } else {
                return false;
            }
            node = n.unwrap();
        }

        false
    }

    fn query(&self, key: String) -> bool {
        self.query_str(&key)
    }
}

///
/// Ring
///
#[derive(Debug)]
struct RingBuffer {
    buffer: Vec<char>,
    len: usize,
    pos: usize,
}

/// Cursor
#[derive(Debug)]
struct BackwardCursor {
    idx: usize,
    len: usize,
}

impl Default for BackwardCursor {
    fn default() -> Self {
        BackwardCursor { idx: 0, len: 1024 }
    }
}

impl BackwardCursor {
    fn next(&mut self) -> usize {
        self.idx += self.len - 1;
        self.idx &= self.len - 1;
        self.idx
    }
}

impl Default for RingBuffer {
    fn default() -> Self {
        RingBuffer {
            buffer: vec![' '; 1024],
            len: 1024,
            pos: 0,
        }
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

    fn cursor(&self) -> BackwardCursor {
        BackwardCursor {
            idx: self.pos,
            len: self.len,
        }
    }

    fn get(&self, index: usize) -> char {
        self.buffer[index]
    }
}

///
/// StreamAlerter
///
#[derive(Debug, Default)]
struct StreamAlerter {
    ring: RingBuffer,
    trie: Trie,
}
impl StreamAlerter {
    fn new(keys: Vec<String>) -> Self {
        let mut trie = Trie::default();
        let mut max_len = 0;
        for key in keys {
            let v = key.chars().count();
            if v > max_len {
                max_len = v;
            }
            trie.insert(key);
        }
        let ring = RingBuffer::new(max_len);
        StreamAlerter { ring, trie }
    }

    fn query(&mut self, ch: char) -> bool {
        self.ring.insert(ch);
        let mut node = &self.trie;
        let mut cursor = self.ring.cursor();
        loop {
            let c = self.ring.get(cursor.next());
            let n = node.children.get(&c);
            if let Some(node) = n {
                if node.is_leaf {
                    return true;
                }
            } else {
                return false;
            }
            node = n.unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie() {
        let mut trie = Trie::default();
        assert!(!trie.query_str(""));
        trie.insert_str("中文");
        trie.insert_str("Yes");
        trie.insert_str("邱张华");
        assert!(trie.query_str("中文"));
        assert!(!trie.query_str("中"));
        assert!(trie.query_str("Yes"));
        assert!(!trie.query_str("邱"));
        assert!(trie.query_str("邱张华"));
    }

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

    #[test]
    fn test_stream_alerter() {
        let mut sa = StreamAlerter::new(vec![
            "赌博".to_string(),
            "游戏".to_string(),
            "摇头丸".to_string(),
            "XXX".to_string(),
        ]);
        assert!(!sa.query('a'));
        assert!(!sa.query('赌'));
        assert!(sa.query('博'));
        assert!(!sa.query('游'));
        assert!(sa.query('戏'));
        assert!(!sa.query('摇'));
        assert!(!sa.query('头'));
        assert!(sa.query('丸'));
        assert!(!sa.query('X'));
        assert!(!sa.query('X'));
        assert!(sa.query('X'));
    }

    #[test]
    fn test_stream_alerter_02() {
        use rand::distributions::Uniform;
        use rand::prelude::*;

        let mut rng = rand::thread_rng();
        let mut sa = StreamAlerter::new(vec!["abc".to_string(), "xyz".to_string()]);

        let mut count = 0;
        let uniform = Uniform::new(0u8, 26u8);
        for _i in 0..1_000_000 {
            let ch = (b'a' + uniform.sample(&mut rng)) as char;
            if sa.query(ch) {
                count += 1;
            }
        }
        // for Rust, after compiling to release version, 1_000_000_000 random will about 8 seconds.
        // but unit test is much slower to about 1 / 100, so I can't do performance test here
        assert_ne!(count, 0)
    }
}
