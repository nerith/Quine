// A Quine in Rust
fn main() {
    let s = ["// A Quine in Rust",
             "fn main() {",
             "    let s = [",
             "",
             "           ];",
             "",
             "    for item in 0..2 {",
             "        println!(\"{}\", s[item]);",
             "    }",
             "",
             "    print!(\"{}\", s[2]);",
             "    println!(\"{:?}\", s[0]);",
             "",
             "    for item in 1..s.len() {",
             "        println!(\"             {:?},\", s[item]);",
             "    }",
             "",
             "    for item in 4..s.len() {",
             "        println!(\"{}\", s[item]);",
             "    }",
             "}",
            ];

    for item in 0..2 {
        println!("{}", s[item]);
    }

    print!("{}", s[2]);
    println!("{:?}", s[0]);

    for item in 1..s.len() {
        println!("             {:?},", s[item]);
    }

    for item in 4..s.len() {
        println!("{}", s[item]);
    }
}
