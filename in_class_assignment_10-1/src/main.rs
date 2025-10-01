struct Student {
    name: String,
    major: String,
}

impl Student {
    // constructor like method 
    fn new(name: &str, major: &str) -> Student {
        Student {
            name: name.to_string(),
            major: major.to_string(),
        }
    }

    // setter for major
    fn set_major(&mut self, major: &str) {
        self.major = major.to_string();
    }

    // getter for major
    fn get_major(&self) -> &str {
        &self.major
    }
}

fn main() {
    // create a new Student using constructor
    let mut s = Student::new("Xaric", "Physics");

    // print initial major
    println!("{}'s initial major: {}", s.name, s.get_major());

    // change major
    s.set_major("Computer Science");

    // print new major
    println!("{}'s updated major: {}", s.name, s.get_major());
}
