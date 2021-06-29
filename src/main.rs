fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => {
            eprintln!("No arguments");
            std::process::exit(1);
        }
        2 => {
            println!("{}", parse(args.get(1).unwrap()));
            std::process::exit(0);
        }
        _ => {
            eprintln!("Multiple arguments");
            std::process::exit(1);
        }
    }
}

fn parse(input: &String) -> String {
    let root: Vec<bool> = input.chars().scan(true, |acc, x| {
        if x == '\'' {
            *acc = !*acc;
        }
        Some(*acc)
    }).collect();

    let mut chars: Vec<char> = input.chars().collect();
    input.match_indices("True").for_each(|(idx, _)| {
        match root.get(idx).unwrap() {
            true => chars[idx] = 't',
            false => (),
        }
    });
    input.match_indices("False").for_each(|(idx, _)| {
        match root.get(idx).unwrap() {
            true => chars[idx] = 'f',
            false => (),
        }
    });
    let output = chars.iter()
        .collect::<String>()
        .replace("\"", "\\\"")
        .replace("'", "\"");

    let v: serde_json::Value = serde_json::from_str(&*output).unwrap();
    serde_json::to_string_pretty(&v).unwrap()
}

#[test]
fn test_parse() {
    let input = "{'a': 'abc', '3': True, 'b': 'asd \"quoted\"', '4': [False, 1, 'text']}";
    let output = parse(&String::from(input));
    let expected_output = r#"{
  "3": true,
  "4": [
    false,
    1,
    "text"
  ],
  "a": "abc",
  "b": "asd \"quoted\""
}"#;
    assert_eq!(output, expected_output);
}