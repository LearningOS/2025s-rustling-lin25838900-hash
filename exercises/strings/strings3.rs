fn trim_me(input: &str) -> String {
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    format!("{} world!", input)
    // 也可以写成 input.to_string() + " world!"
}

fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons")
}
        