fn sum(val: &serde_json::Value) -> i64 {
    match val {
        serde_json::Value::Number(n) => n.as_i64().unwrap(),
        serde_json::Value::Array(vs) => vs.iter().map(sum).sum(),
        serde_json::Value::Object(o) => o.values().map(sum).sum(),
        _ => 0,
    }
}

fn sum2(val: &serde_json::Value) -> i64 {
    match val {
        serde_json::Value::Number(n) => n.as_i64().unwrap(),
        serde_json::Value::Array(vs) => vs.iter().map(sum2).sum(),
        serde_json::Value::Object(o) => if o.values().any(|v| v.as_str() == Some("red")) { 0 } else { o.values().map(sum2).sum() },
        _ => 0,
    }
}

fn run(title: &str, input: &str) {
    let json: serde_json::Value = serde_json::from_str(input).unwrap();

    println!("{} part 1: {}", title, sum(&json));

    println!("{} part 2: {}", title, sum2(&json));
}

const INPUT_DEMO: &str = r#"{"a":{"b":4},"c":-1}"#;
const INPUT_DEMO2: &str = r#"{"a":{"b":"red"},"c":-1}"#;

fn main() {
    run("demo", INPUT_DEMO);
    run("demo2 ", INPUT_DEMO2);
    run("input", &std::fs::read_to_string("12/input.txt").unwrap());
}
