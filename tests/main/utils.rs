pub fn read_encoded(text: &str) -> Vec<u8> {
    let mut bytes = vec![];
    let lines: Vec<_> = text
        .lines()
        .map(|line| {
            if let Some(pos) = line.find("//") {
                &line[..pos]
            } else {
                line
            }
        })
        .map(|line| line.trim())
        .collect();

    for mut line in lines {
        while !line.is_empty() {
            let val = if line.starts_with("\\x") {
                let val = u8::from_str_radix(&line[2..4], 16).unwrap();
                line = &line[4..];
                val
            } else {
                let val = line.as_bytes()[0];
                line = &line[1..];
                val
            };
            bytes.push(val);
        }
    }

    bytes
}

pub fn read_encoded_file(name: &str) -> Vec<u8> {
    let text = std::fs::read_to_string(format!("tests/main/data/{}.binn", name)).unwrap();
    read_encoded(&text)
}
