#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        if s.is_empty() {
            return Person::default();
        }
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Person::default();
        }
        let name = parts[0].trim();
        let age_str = parts[1].trim();
        if name.is_empty() {
            return Person::default();
        }
        if let Ok(age) = age_str.parse::<usize>() {
            Person {
                name: name.to_string(),
                age,
            }
        } else {
            Person::default()
        }
    }
}

fn main() {
    // Use the `from` function
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}
