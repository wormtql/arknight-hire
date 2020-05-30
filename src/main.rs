mod combo;

use combo::COMBO;
use std::io;
use std::fmt;

struct Item {
    pub tags: Vec<String>,
    pub outs: Vec<String>
}

impl Item {
    pub fn new() -> Item {
        Item {
            tags: Vec::new(),
            outs: Vec::new()
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in self.tags.iter() {
            write!(f, "{} ", i).unwrap();
        }
        write!(f, "->").unwrap();
        for i in self.outs.iter() {
            write!(f, " {}", i).unwrap();
        }
        Ok(())
    }
}

fn load_combo(combo: &str) -> Vec<Item> {
    let lines = combo.trim().split("\n");
    let mut ans: Vec<Item> = Vec::new();
    for line in lines {
        let temp: Vec<&str> = line.trim().split(",").collect();
        let temp2: Vec<&str> = temp[0].split_whitespace().collect();

        let tag_number: usize = temp2[0].parse().unwrap();
        let mut item = Item::new();
        for i in 1..1 + tag_number {
            item.tags.push(String::from(temp2[i]));
        }

        let temp3: Vec<&str> = temp[1].split_whitespace().collect();
        let out_number: usize = temp3[0].parse().unwrap();
        for i in 1..1 + out_number {
            item.outs.push(String::from(temp3[i]));
        }

        ans.push(item);
    }

    ans
}

fn main() {
    let c = load_combo(COMBO);

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let tags: Vec<&str> = input.split_whitespace().collect();

        let mut viable: Vec<usize> = Vec::new();

        for (index, item) in c.iter().enumerate() {
            let mut flag = true;
            for i in item.tags.iter() {
                if !tags.contains(&i.as_str()) {
                    flag = false;
                    break;
                }
            }

            if flag {
                viable.push(index);
            }
        }

        for (index, i) in viable.iter().enumerate() {
            println!("#{}: {}", index, c[*i]);
        }
    }

    
}
