use std::collections::{HashMap, VecDeque};

pub fn solve() {
    let input = include_str!("../18.txt").trim();
    let lines: Vec<&str> = input.split('\n').collect();

    let mut cpu = Cpu::new(0, &lines);
    cpu.run(None);

    let mut cpu0 = Cpu::new(0, &lines);
    let mut cpu1 = Cpu::new(1, &lines);

    loop {
        let last_sent = (cpu0.send_count, cpu1.send_count);
        cpu0.run(Some(&mut cpu1));
        cpu1.run(Some(&mut cpu0));

        if (cpu0.send_count, cpu1.send_count) == last_sent {
            // Nothing happened - deadlock
            break;
        }
    }

    println!("18 part two: {}", cpu1.send_count);
}

struct Cpu<'a> {
    program: &'a Vec<&'a str>,
    ip: usize,
    registers: HashMap<&'a str, i64>,
    queue: VecDeque<i64>,
    send_count: u32,
}

impl<'a> Cpu<'a> {
    fn new(id: usize, program: &'a Vec<&'a str>) -> Self {
        let mut registers = HashMap::<&str, i64>::new();
        registers.insert("p", id as i64);
        Cpu {
            program,
            ip: 0,
            registers,
            queue: VecDeque::new(),
            send_count: 0,
        }
    }

    fn message(&mut self, val: i64) {
        self.queue.push_back(val);
    }

    fn run(&mut self, mut other_cpu: Option<&mut Cpu>) {
        let mut sounds = Vec::<i64>::new();
        loop {
            if self.ip >= self.program.len() {
                return;
            }

            let mut words = self.program.get(self.ip).unwrap().split_whitespace();
            let op = words.next().unwrap();
            let reg = words.next().unwrap();

            match op {
                "snd" => {
                    let val = get_val(reg, &self.registers);
                    if let Some(ref mut other) = other_cpu {
                        other.message(val);
                        self.send_count += 1;
                    } else {
                        sounds.push(val);
                    }
                }
                "rcv" => {
                    if other_cpu.is_some() {
                        if let Some(val) = self.queue.pop_front() {
                            self.registers.insert(reg, val);
                        } else {
                            // ip unchanged, so we'll come back here when re-run
                            return;
                        }
                    } else if get_val(reg, &self.registers) > 0 {
                        println!("18 part one: {}", sounds.pop().unwrap());
                        return;
                    }
                }
                _ => {
                    let val = get_val(words.next().unwrap(), &self.registers);
                    match op {
                        "set" => {
                            self.registers.insert(reg, val);
                        }
                        "add" => {
                            let a = *self.registers.get(reg).unwrap_or(&0);
                            self.registers.insert(reg, val + a);
                        }
                        "mul" => {
                            let a = *self.registers.get(reg).unwrap_or(&0);
                            self.registers.insert(reg, val * a);
                        }
                        "mod" => {
                            let a = *self.registers.get(reg).unwrap_or(&0);
                            self.registers.insert(reg, a % val);
                        }
                        "jgz" => {
                            if get_val(reg, &self.registers) > 0 {
                                self.ip = (self.ip as i64 + val).try_into().unwrap(); // Assume never goes -ve
                                self.ip -= 1; // Because we always +1 at the end of the loop
                            }
                        }
                        _bad => panic!("Unrecognized op: {}", _bad),
                    }
                }
            }

            self.ip += 1;
        }
    }
}

fn get_val(x: &str, registers: &HashMap<&str, i64>) -> i64 {
    if let Ok(v) = x.parse::<i64>() {
        v
    } else {
        *registers.get(x).unwrap_or(&0)
    }
}
