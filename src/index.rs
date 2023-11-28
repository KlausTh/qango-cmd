
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{Read, BufReader, Write, BufWriter, Error, ErrorKind};
use std::path::Path;
use qango::board::Board;
use qango::board::side::Side;
use crate::Configuration;

const MAGIC_NUMBER : &[u8;6] = b"QIndex"; // [0x51,0x49,0x6E,0x64,0x65,0x78]

struct Index {
    round : usize,
    side : Side,
    tree : BTreeSet<u64>
}

pub fn index() {
    let config = Configuration::get();
    let filename = "round_35b.dat";
    let fout = "round_35b.dat";

    println!("load file = {:?}", filename);
    let index = Index::load(&filename).unwrap();
    println!("index round: {}, side: {} ready", index.round, index.side);

    if config.is_debug() {
        index.tree.iter()
            .map(|id| (id, Board::from(*id)))
            .for_each(|(id, board)| println!("{}\n{}", id, board));
    }

    println!("save file = {:?}", fout);
    index.save(fout);
}

impl Index {
    pub fn init(round : usize, side : Side) -> Index {
        Index {
            round : round,
            side : side,
            tree : BTreeSet<u64>::new()
        }
    }

    pub fn load(filename : &str) -> Result<Index, Error> {
        let f1 = File::open(filename)?;
        let mut reader = BufReader::new(&f1);
        let mut buf = [0; 8];
        let mut tree : BTreeSet<u64> = BTreeSet::new();

        while reader.read_exact(&mut buf).is_ok() {
            let id = u64::from_ne_bytes(buf);

            tree.insert(id);
        }

        return Ok(Index {
            round : 35,
            side : Side::BLACK,
            tree : tree,
        });
    }

    pub fn save(&self, filename : &str) -> Result<(), Error> {
        let fout : File = File::create(&Path::new(filename))?;
        let mut writer = BufWriter::new(fout);

        writer.write_all(MAGIC_NUMBER)?;
        let bside : [u8;1] = [self.side.into()];
        writer.write_all(&bside)?;
        let bround : [u8;1] = [self.round as u8];
        writer.write_all(&bround)?;

        let finish = self.tree.iter()
            .map(|id| id.to_ne_bytes())
            .map(|bytes| writer.write_all(&bytes))
            .all(|err| err.is_ok());

        if finish {
            writer.flush()?;
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "could not write index tree"))
        }
    }
}