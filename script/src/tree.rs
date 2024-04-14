use crate::EWord;
use blake2::{digest::Update, Blake2b512, Digest};

fn s(x: &[u8]) -> String {
    std::str::from_utf8(x).unwrap().to_string()
}

pub enum Node {
    Block {
        number: usize,
        contracts: Vec<Node>,
    },
    Contract {
        address: [u8; 20],
        storage: Vec<Node>,
    },
    Variable {
        name: String,
        value: EWord,
    },
    Struct {
        name: String,
        fields: Vec<Node>,
    },
    Mapping {
        name: String,
        entries: Vec<Node>,
    },
    Entry {
        key: EWord,
        entry: Box<Node>,
    },
}

impl Node {
    fn to_id(&self) -> u8 {
        match self {
            Node::Variable { .. } => 1,
            Node::Struct { .. } => 2,
            Node::Mapping { .. } => 3,
            _ => panic!("nope"),
        }
    }

    pub fn serialize<W: std::io::Write>(&self, out: &mut W) {
        match self {
            Node::Block { number, contracts } => {
                out.write_all(&number.to_le_bytes()).unwrap();
                out.write_all(&contracts.len().to_le_bytes()).unwrap();
                for c in contracts {
                    c.serialize(out);
                }
            }
            Node::Contract { address, storage } => {
                out.write_all(address).unwrap();
                out.write_all(&storage.len().to_le_bytes()).unwrap();
                for s in storage {
                    s.serialize(out);
                }
            }
            Node::Variable { name, value } => {
                out.write_all(&[self.to_id()]).unwrap();
                out.write_all(&name.len().to_le_bytes()).unwrap();
                out.write_all(name.as_bytes()).unwrap();
                out.write_all(value).unwrap();
            }
            Node::Struct { name, fields } => {
                out.write_all(&[self.to_id()]).unwrap();
                todo!()
            }
            Node::Mapping { name, entries } => {
                out.write_all(&[self.to_id()]).unwrap();
                out.write_all(&name.len().to_le_bytes()).unwrap();
                out.write_all(name.as_bytes()).unwrap();
                out.write_all(&entries.len().to_le_bytes()).unwrap();
                for e in entries {
                    e.serialize(out);
                }
            }
            Node::Entry { key, entry } => {
                out.write_all(key).unwrap();
                entry.serialize(out);
            }
        }
    }

    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = Blake2b512::new();
        self._hash(&mut hasher);
        hasher.finalize().to_vec()
    }

    fn _hash(&self, h: &mut Blake2b512) {
        match self {
            Node::Block { number, contracts } => {
                Digest::update(h, &number.to_be_bytes());
                for x in contracts {
                    x._hash(h);
                }
            }
            Node::Contract { address, storage } => {
                Digest::update(h, address);
                for s in storage {
                    s._hash(h)
                }
            }
            Node::Variable { name, value } => {
                Digest::update(h, format!("{}{}", name, s(value)).as_bytes());
            }
            Node::Struct { name, fields } => {
                Digest::update(h, name);
                for f in fields {
                    f._hash(h)
                }
            }
            Node::Mapping { name, entries } => {
                Digest::update(h, name.as_bytes());
                for e in entries {
                    e._hash(h)
                }
            }
            Node::Entry { key, entry } => {
                Digest::update(h, s(key));
                entry._hash(h);
            }
        }
    }

    pub fn pretty(&self) {
        let mut r = String::new();
        self._pretty(0, &mut r);
        println!("{r}");
    }

    fn _pretty(&self, depth: usize, r: &mut String) {
        let indent = " ".repeat(depth);
        r.push_str(&indent);

        match self {
            Node::Block { number, contracts } => {
                r.push_str(&format!("Block #{number}\n"));
                for c in contracts {
                    c._pretty(depth + 2, r);
                }
            }
            Node::Contract { address, storage } => {
                r.push_str(&format!("Contract@{}\n", s(address)));
                for s in storage {
                    s._pretty(depth + 2, r);
                }
            }
            Node::Variable { name, value } => {
                r.push_str(&format!("{name} -> {}\n", s(value)));
            }
            Node::Struct { name, fields } => todo!(),
            Node::Mapping { name, entries } => {
                r.push_str(&format!("{name} :=\n"));
                for e in entries {
                    e._pretty(depth + 2, r);
                }
            }
            Node::Entry { key, entry } => {
                r.push_str(&format!("{} := \n", s(key)));
                entry._pretty(depth + 1, r);
            }
        }
    }
}
