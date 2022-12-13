use std::fs;
use std::str::Chars;
use std::cmp::Ordering;
use std::iter::{Iterator, Peekable, zip};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Number(u32),
    Empty,
}

fn parse_packet_pair(pair: &str) -> (Packet, Packet) {
    let mut iter = pair.split('\n');
    let left = iter.next().unwrap();
    let right = iter.next().unwrap();
    (parse_packet(left), parse_packet(right))
}

fn parse_packet(packet: &str) -> Packet {
    let mut iter = packet.chars().peekable();
    parse_list(&mut iter)
}

fn parse_list(mut iter: &mut Peekable<Chars<'_>>) -> Packet {
    let mut v: Vec<Packet> = vec![];
    iter.next(); // initial '['
    
    while let Some(c) = iter.peek() {
        match c {
            ']' => {
                iter.next();
                return Packet::List(v);
            },
            ',' => { 
                iter.next();
            },
            _   => {
                match parse_element(&mut iter) {
                    Packet::Empty => {},
                    p => { v.push(p); },
                }
            }
        }
    }

    panic!("unterminated list");
}

fn parse_element(iter: &mut Peekable<Chars<'_>>) -> Packet {
    match iter.peek() {
        Some(c) => match c {
            '[' => parse_list(iter),
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => parse_num(iter),
            _ => panic!("unexpected elem: {}", c),
        },
        _ => Packet::Empty,
    }
}

fn parse_num(iter: &mut Peekable<Chars<'_>>) -> Packet {
    let mut s: String = String::new();
    while let Some(c) = iter.peek() {
        match c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                s.push(*c);
                iter.next();
            },
            _ => {
                return Packet::Number(u32::from_str_radix(&s, 10).unwrap());
            },
        }
    }
    panic!("unterminated input");
}

fn fix_packet_pair(left: &Packet, right: &Packet) -> (Packet, Packet) {
    (fix_packet(&left, &right), fix_packet(&right, &left))
}

fn fix_packet(broken: &Packet, reference: &Packet) -> Packet {
    match broken {
        Packet::Number(n) => fix_number(*n, &reference),
        Packet::List(v) => fix_list(v, &reference),
        _ => panic!("unexpected packet"),
    }
}

fn fix_list(v: &Vec<Packet>, reference: &Packet) -> Packet {
    match reference {
        Packet::Number(_) => Packet::List(v.to_vec()),
        Packet::List(v2) => fix_each(v, v2),
        _ => panic!("unexpected packet"),
    }
}
fn fix_each(broken: &Vec<Packet>, reference: &Vec<Packet>) -> Packet {
    let mut fixed: Vec<_> = zip(broken.iter(), reference.iter())
        .map(|(b, f)| fix_packet(b, f))
        .collect();
    broken.iter().skip(fixed.len()).for_each(|e| fixed.push(e.clone()));
    Packet::List(fixed)
}

fn fix_number(n: u32, reference: &Packet) -> Packet {
    match reference {
        Packet::Number(_) => Packet::Number(n),
        Packet::List(v) => Packet::List(vec![fix_number_vec(n, &v)]),
        _ => panic!("unexpected packet"),
    }
}

fn fix_number_vec(n: u32, reference: &Vec<Packet>) -> Packet {
    if reference.is_empty() {
        Packet::Number(n)
    } else {
        fix_number(n, &reference[0])
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Packet::Number(nl) => {
                match other {
                    Packet::Number(nr) => nl.cmp(nr),
                    Packet::List(_) => {
                        let (left, right) = fix_packet_pair(self, other);
                        left.cmp(&right)
                    },
                    t => panic!("unexpected token: {:?}", t),
                }
            },
            Packet::List(vl) => {
                match other {
                    Packet::List(vr) => {
                        let elem_order = zip(vl.iter(), vr.iter())
                            .map(|(el, er)| el.cmp(er))
                            .skip_while(|&o| o == Ordering::Equal)
                            .nth(0)
                            .unwrap_or(Ordering::Equal);
                        match elem_order {
                            Ordering::Equal => vl.len().cmp(&vr.len()),
                            o => o,
                        }
                    },
                    Packet::Number(_) => {
                        let (left, right) = fix_packet_pair(self, other);
                        left.cmp(&right)
                    },
                    t => panic!("unexpected token: {:?}", t),
                }
            },
            t => panic!("unexpected token: {:?}", t),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn is_ordered(pair: &(Packet, Packet)) -> bool {
    match pair.0.cmp(&pair.1) {
        Ordering::Less | Ordering::Equal => true,
        Ordering::Greater => false,
    }
}

pub fn solve() {
    let input = fs::read_to_string("resources/day13.txt").unwrap();
    let ordered_pairs: Vec<_> = input.split("\n\n")
        .map(parse_packet_pair)
        .collect();

    let ordered_index_sum: usize = ordered_pairs.iter()
        .enumerate()
        .filter(|(_, pair)| is_ordered(&pair))
        .map(|(i, _)| i+1)
        .sum();

    println!("{}", ordered_index_sum);

    let mut packets: Vec<_> = ordered_pairs.into_iter()
        .flat_map(|pair| [pair.0, pair.1].into_iter())
        .collect();

    let dp1 = Packet::List(vec![ Packet::List(vec![ Packet::Number(2) ])]);
    let dp2 = Packet::List(vec![ Packet::List(vec![ Packet::Number(6) ])]);
    packets.push(dp1.clone());
    packets.push(dp2.clone());

    packets.sort();

    let dp1_idx = packets.iter().position(|p| *p == dp1).unwrap() + 1;
    let dp2_idx = packets.iter().position(|p| *p == dp2).unwrap() + 1;

    println!("{}", dp1_idx * dp2_idx);
}
