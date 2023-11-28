
use std::collections::LinkedList; 
use std::path::Path;

const MAGIC_NUMBER : &[u8;6] = b"QIndex"; // [0x51,0x49,0x6E,0x64,0x65,0x78]

struct Search {
    start : u64,
    end : u64,
    size : usize,
    cache : LinkedList<SearchBlock>,
}

struct CacheBlock {
    start : u64,
    end : u64,
    elements : &[u64],
}

impl Search {
    pub fn init(indexfile : &Path) -> Self {
        let f1 = File::open(filename)?;
        let mut reader = BufReader::new(&f1);
        let mut size;

        let magic_number : [u8;6];
        size = reader.read(&mut magic_number);
        let bside : [u8;1] = [self.side.into()];
        size = reader.read(bside);
        let bround : [u8;1] = [self.round as u8];
        reader.

        let mut buf = [0; 8];
        let mut tree : BTreeSet<u64> = BTreeSet::new();

        while reader.read_exact(&mut buf).is_ok() {
            let id = u64::from_ne_bytes(buf);

            tree.insert(id);
        }

        Search {
            
        }
    }

    pub fn find(&Self, key : u64) -> bool {
        false
    }
}