use std::borrow::{Borrow, BorrowMut};
use std::fs::File;
use std::io::{BufRead, BufReader};
use bitvec::bits;
use bitvec::field::BitField;
use bitvec::order::Msb0;
use bitvec::slice::BitSlice;
use bitvec::vec::BitVec;

fn load(filename: &str) -> Vec<u8> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    let lines: Vec<String> = buf.lines()
        .map(
            |l| l.unwrap()
        )
        .collect();

    let chars: Vec<char> = lines[0]
        .chars()
        .collect();

    let vec: Vec<u8> = chars
        .chunks(2)
        .map(|str| {
            let byte = String::from_iter(str);

            u8::from_str_radix(byte.as_str(), 16).unwrap()
        })
        .collect();
    vec
}

#[derive(Debug)]
struct ParserState<'a> {
    bitstream: &'a BitSlice<Msb0, u8>,
    current: usize
}

#[derive(Debug)]
struct PacketHeader {
    version: u8,
    ptype: u8
}

#[derive(Debug)]
struct LiteralPacket {
    value: u64
}

impl LiteralPacket {
    fn from_bitstream(parser: &mut ParserState) -> LiteralPacket {
        let mut current = parser.current;
        let mut value: u64 = 0;
        loop {
            let group: u8 = parser.bitstream[current..current+5].load_be();
            value <<= 4;
            value |= (group & 0b1111) as u64;
            current += 5;

            //  Last group found
            if (group & 0b10000) == 0 {
                break;
            }
        }
        parser.current = current;

        LiteralPacket {
            value,
        }
    }
}

#[derive(Debug)]
struct OperatorPacket {
    packets: Vec<Packet>
}

impl OperatorPacket {
    fn from_bitstream(parser: &mut ParserState) -> OperatorPacket {
        let length = parser.bitstream[parser.current];
        parser.current += 1;

        let mut packets: Vec<Packet> = vec![];

        if length == false {
            let bit_length: u16 = parser.bitstream[parser.current..parser.current+15].load_be();
            parser.current += 15;

            let bitstream = &parser.bitstream[parser.current..parser.current+(bit_length as usize)];
            parser.current += bit_length as usize;
            dbg!(bitstream.borrow());

            let mut state = ParserState {
                bitstream: bitstream.borrow(),
                current: 0
            };
            while state.current < bit_length as usize {
                let packet = Packet::from_bitstream(state.borrow_mut());
                packets.push(packet);
            }
        } else {
            let packet_count: u16 =  parser.bitstream[parser.current..parser.current+11].load_be();
            parser.current += 11;

            for _ in 0..packet_count {
                let packet = Packet::from_bitstream(parser.borrow_mut());
                packets.push(packet);
            }
        }

        OperatorPacket{
            packets
        }
    }
}


#[derive(Debug)]
enum Packet {
    Literal (PacketHeader, LiteralPacket),
    Operator (PacketHeader, OperatorPacket),
    Invalid
}

impl Packet {
    fn from_bitstream(parser: &mut ParserState) -> Packet {
        if parser.current+6 >= parser.bitstream.len() {
            return Packet::Invalid;
        }

        let version: u8 = parser.bitstream[parser.current..parser.current+3].load_be();
        let ptype: u8 = parser.bitstream[parser.current+3..parser.current+6].load_be();
        parser.current += 6;
        match ptype {
            4 => {
                Packet::Literal(
                    PacketHeader{version, ptype},
                    LiteralPacket::from_bitstream(parser.borrow_mut())
                )
            },

            _ => {
                Packet::Operator(
                    PacketHeader{version, ptype},
                    OperatorPacket::from_bitstream(parser.borrow_mut())
                )
            }
        }
    }

    fn value(&self)-> u64 {
        match self {
            Packet::Literal(header, literal) => {
                return literal.value;
            }
            Packet::Operator(header, op) => {
                match header.ptype {
                    0 => {
                        let mut sum = 0;
                        for child in op.packets.iter() {
                            sum += child.value();
                        }
                        return sum;
                    }
                    1 => {
                        assert!(op.packets.len() >= 1);
                        let mut product = 1;
                        for child in op.packets.iter() {
                            product *= child.value();
                        }
                        return product;
                    }
                    2 => {
                        let mut min: Option<u64> = None;
                        for child in op.packets.iter() {
                            let value = child.value();
                            if min.is_none() {
                                min = Some(value);
                            } else if value < min.unwrap() {
                                min = Some(value)
                            }
                        }
                        return min.unwrap();
                    }
                    3 => {
                        let mut max: Option<u64> = None;
                        for child in op.packets.iter() {
                            let value = child.value();
                            if max.is_none() {
                                max = Some(value);
                            } else if value > max.unwrap() {
                                max = Some(value)
                            }
                        }
                        return max.unwrap();
                    }
                    5 => {
                        assert_eq!(op.packets.len(), 2);
                        return if op.packets[0].value() > op.packets[1].value() {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        assert_eq!(op.packets.len(), 2);
                        return if op.packets[0].value() < op.packets[1].value() {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        assert_eq!(op.packets.len(), 2);
                        return if op.packets[0].value() == op.packets[1].value() {
                            1
                        } else {
                            0
                        }
                    }
                    _ => {
                        panic!("Invalid packet type ID");
                    }
                }
            }
            _ => {
                panic!("Invalid packet type");
            }
        }
    }

}

struct StreamParser {
    stream: Vec<u8>
}

impl StreamParser {
    pub fn from_vec(vec: Vec<u8>) -> StreamParser {
        StreamParser  {
            stream: vec
        }
    }

    fn calculate_version_sum(root: &Packet) -> usize {
        match root {
            Packet::Literal(header, _) => {
                return header.version as usize;
            }
            Packet::Operator(header, op) => {
                let mut sum = header.version as usize;
                for child in op.packets.iter() {
                    let child_score = StreamParser::calculate_version_sum(child);
                    sum += child_score;
                }
                return sum;
            }
            _ => {
                panic!();
            }
        }
    }

    fn dump(&self) {
        let bits = BitSlice::<Msb0, _>::from_slice(&self.stream).unwrap();
        let mut state = ParserState {
            bitstream: &bits,
            current: 0
        };

        let packet = Packet::from_bitstream(state.borrow_mut());
        dbg!(packet.borrow());

        let sum = StreamParser::calculate_version_sum(packet.borrow());
        println!("Version sum: {}", sum);

        let value = packet.value();
        println!("Root packet value: {}", value);
    }
}


fn main() {
    let input = load("input.txt");
    let parser = StreamParser::from_vec(input);
    parser.dump();
}
