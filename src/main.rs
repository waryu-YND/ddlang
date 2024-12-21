use std::{collections::VecDeque, env, fmt::Display, io, ops, path};

fn main() {
  if let Some(filepath) = env::args().nth(1) {
    let filepath = path::Path::new(&filepath);
    let contents = std::fs::read_to_string(filepath).expect("Failed to read file");
    execute(contents);
  }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum Item {
  Int(i32),
  Float(f64),
  Char(u16),
}

impl Display for Item {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Item::Int(number) => write!(f, "{}", number),
      Item::Float(number) => write!(f, "{:.6}", number),
      Item::Char(number) => write!(
        f,
        "{}",
        char::decode_utf16(vec![*number])
          .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
          .collect::<String>()
      ),
    }
  }
}

impl ops::Add<Item> for Item {
  type Output = Item;

  fn add(self, rhs: Item) -> Self::Output {
    match self {
      Item::Int(l) => match rhs {
        Item::Int(r) => Item::Int(l + r),
        Item::Float(r) => Item::Float(l as f64 + r),
        Item::Char(r) => Item::Int(l + r as i32),
      },
      Item::Float(l) => match rhs {
        Item::Int(r) => Item::Float(l + r as f64),
        Item::Float(r) => Item::Float(l + r),
        Item::Char(r) => Item::Float(l + r as f64),
      },
      Item::Char(l) => match rhs {
        Item::Int(r) => Item::Int(l as i32 + r),
        Item::Float(r) => Item::Float(l as f64 + r),
        Item::Char(r) => Item::Char(l + r),
      },
    }
  }
}

impl ops::Sub<Item> for Item {
  type Output = Item;

  fn sub(self, rhs: Item) -> Self::Output {
    match self {
      Item::Int(l) => match rhs {
        Item::Int(r) => Item::Int(l - r),
        Item::Float(r) => Item::Float(l as f64 - r),
        Item::Char(r) => Item::Int(l - r as i32),
      },
      Item::Float(l) => match rhs {
        Item::Int(r) => Item::Float(l - r as f64),
        Item::Float(r) => Item::Float(l - r),
        Item::Char(r) => Item::Float(l - r as f64),
      },
      Item::Char(l) => match rhs {
        Item::Int(r) => Item::Int(l as i32 - r),
        Item::Float(r) => Item::Float(l as f64 - r),
        Item::Char(r) => Item::Char(l - r),
      },
    }
  }
}

impl ops::Mul<Item> for Item {
  type Output = Item;

  fn mul(self, rhs: Item) -> Self::Output {
    match self {
      Item::Int(l) => match rhs {
        Item::Int(r) => Item::Int(l * r),
        Item::Float(r) => Item::Float(l as f64 * r),
        Item::Char(r) => Item::Int(l * r as i32),
      },
      Item::Float(l) => match rhs {
        Item::Int(r) => Item::Float(l * r as f64),
        Item::Float(r) => Item::Float(l * r),
        Item::Char(r) => Item::Float(l * r as f64),
      },
      Item::Char(l) => match rhs {
        Item::Int(r) => Item::Int(l as i32 * r),
        Item::Float(r) => Item::Float(l as f64 * r),
        Item::Char(r) => Item::Char(l * r),
      },
    }
  }
}

impl ops::Div<Item> for Item {
  type Output = Item;

  fn div(self, rhs: Item) -> Self::Output {
    match self {
      Item::Int(l) => match rhs {
        Item::Int(r) => Item::Int(l / r),
        Item::Float(r) => Item::Float(l as f64 / r),
        Item::Char(r) => Item::Int(l / r as i32),
      },
      Item::Float(l) => match rhs {
        Item::Int(r) => Item::Float(l / r as f64),
        Item::Float(r) => Item::Float(l / r),
        Item::Char(r) => Item::Float(l / r as f64),
      },
      Item::Char(l) => match rhs {
        Item::Int(r) => Item::Int(l as i32 / r),
        Item::Float(r) => Item::Float(l as f64 / r),
        Item::Char(r) => Item::Char(l / r),
      },
    }
  }
}

impl Item {
  fn into(self, mode: Mode) -> Item {
    match mode {
      Mode::Int => match self {
        Item::Int(n) => Item::Int(n),
        Item::Float(n) => Item::Int(n as i32),
        Item::Char(n) => Item::Int(n as i32),
      },
      Mode::Float => match self {
        Item::Int(n) => Item::Float(n as f64),
        Item::Float(n) => Item::Float(n),
        Item::Char(n) => Item::Float(n as f64),
      },
      Mode::Char => match self {
        Item::Int(n) => Item::Char(n as u16),
        Item::Float(n) => Item::Char(n as u16),
        Item::Char(n) => Item::Char(n),
      },
    }
  }

  fn get_int(self) -> i32 {
    match self {
      Item::Int(number) => number,
      Item::Float(number) => number as i32,
      Item::Char(number) => number as i32,
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Mode {
  Int,
  Float,
  Char,
}

fn execute<T: AsRef<str>>(code: T) {
  let code: Vec<char> = code
    .as_ref()
    .chars()
    .filter(|c| c == &'木' || c == &'野' || c == &'ど' || c == &'松')
    .collect();
  let mut data_stack: Vec<Item> = Vec::new();
  let mut input_stack: VecDeque<u16> = VecDeque::new();
  let mut mode: Mode = Mode::Int;
  let mut index: usize = 0;
  while let Some(c) = code.get(index) {
    if c == &'ど' {
      let mut count = 0;
      while let Some(dd) = code.get(index + count) {
        if dd != &'ど' {
          break;
        }
        count += 1;
      }
      index += count;
      match mode {
        Mode::Int => data_stack.push(Item::Int(count as i32)),
        Mode::Float => data_stack.push(Item::Float(count as f64)),
        Mode::Char => data_stack.push(Item::Char(count as u16)),
      }
      continue;
    }
    match (c, code.get(index + 1)) {
      (&'木', Some(&'木')) => {
        // スタック回転
        let Some(rot) = data_stack.pop() else {
          eprintln!("木木 accessed out of range.");
          index += 2;
          continue;
        };
        let Some(depth) = data_stack.pop() else {
          eprintln!("木木 accessed out of range.");
          data_stack.push(rot);
          index += 2;
          continue;
        };
        let mut rolled: Vec<Item> = Vec::new();
        for _ in 0..depth.get_int() {
          let Some(data) = data_stack.pop() else {
            eprintln!("木木 accessed out of range.");
            for _ in 0..rolled.len() {
              data_stack.push(rolled.pop().unwrap());
            }
            data_stack.push(depth);
            data_stack.push(rot);
            index += 2;
            continue;
          };
          rolled.push(data);
        }
        let mut result: Vec<Item> = Vec::new();
        for i in 0..depth.get_int() {
          result.push(rolled[((i + rot.get_int()) % depth.get_int()) as usize]);
        }
        for _ in 0..depth.get_int() {
          data_stack.push(result.pop().unwrap());
        }
        index += 2;
      }
      (&'木', Some(&'野')) => {
        // 入力
        match mode {
          Mode::Int => {
            let mut buffer = String::new();
            io::stdin()
              .read_line(&mut buffer)
              .expect("Failed to read line.");
            data_stack.push(Item::Int(
              buffer.trim().parse().expect("Input is not integer."),
            ));
          }
          Mode::Float => {
            let mut buffer = String::new();
            io::stdin()
              .read_line(&mut buffer)
              .expect("Failed to read line.");
            data_stack.push(Item::Float(
              buffer.trim().parse().expect("Input is not float."),
            ));
          }
          Mode::Char => {
            if input_stack.is_empty() {
              let mut buffer = String::new();
              io::stdin()
                .read_line(&mut buffer)
                .expect("Failed to read line.");
              input_stack = buffer.encode_utf16().collect();
            }
            data_stack.push(Item::Char(input_stack.pop_front().unwrap()));
          }
        }
        index += 2;
      }
      (&'木', Some(&'松')) => {
        // 出力
        let Some(item) = data_stack.pop() else {
          eprintln!("木松 accessed out of range.");
          index += 2;
          continue;
        };
        print!("{}", item);
        index += 2;
      }
      (&'野', Some(&'木')) => {
        // 比較(l<r)
        let Some(r) = data_stack.pop() else {
          eprintln!("野木 accessed out of range.");
          index += 2;
          continue;
        };
        let Some(l) = data_stack.pop() else {
          eprintln!("野木 accessed out of range.");
          data_stack.push(r);
          index += 2;
          continue;
        };
        if l < r {
          data_stack.push(Item::Int(1).into(mode));
        } else {
          data_stack.push(Item::Int(0).into(mode));
        }
        index += 2;
      }
      (&'野', Some(&'野')) => {
        // 複製
        let duplicated = data_stack[data_stack.len() - 1];
        data_stack.push(duplicated);
        index += 2;
      }
      (&'野', Some(&'松')) => {
        // ジャンプ
        let Some(will_jump) = data_stack.pop() else {
          eprintln!("野松 accessed out of range.");
          index += 2;
          continue;
        };
        let Some(address) = data_stack.pop() else {
          eprintln!("野松 accessed out of range.");
          data_stack.push(will_jump);
          index += 2;
          continue;
        };
        if will_jump.get_int() != 0 {
          index = address.get_int() as usize;
        } else {
          index += 2;
        }
      }
      (&'松', Some(&'木')) => {
        // 減算
        let Some(r) = data_stack.pop() else {
          eprintln!("松木 accessed out of range.");
          index += 2;
          continue;
        };
        let Some(l) = data_stack.pop() else {
          eprintln!("松木 accessed out of range.");
          data_stack.push(r);
          index += 2;
          continue;
        };
        data_stack.push((l - r).into(mode));
        index += 2;
      }
      (&'松', Some(&'野')) => {
        // 除算
        let Some(r) = data_stack.pop() else {
          eprintln!("松野 accessed out of range.");
          index += 2;
          continue;
        };
        let Some(l) = data_stack.pop() else {
          eprintln!("松野 accessed out of range.");
          data_stack.push(r);
          index += 2;
          continue;
        };
        data_stack.push((l / r).into(mode));
        index += 2;
      }
      (&'松', Some(&'松')) => {
        // 型変更
        mode = match mode {
          Mode::Int => Mode::Float,
          Mode::Float => Mode::Char,
          Mode::Char => Mode::Int,
        };
        index += 2;
      }
      (&'木', _) => {
        data_stack.reverse();
        index += 1;
      }
      (&'野', _) => {
        data_stack = Vec::new();
        index += 1;
      }
      (&'松', _) => {
        for item in &data_stack {
          print!("{}", item);
        }
        index += 1;
      }
      _ => unreachable!(),
    }
  }
}
