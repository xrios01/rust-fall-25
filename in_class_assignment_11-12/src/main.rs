trait ShowInfo {
    fn show_info(&self);
}

struct Undergrad {
    name: String,
    major: String,
    gpa: f32,
}


struct GradStudent {
    name: String,
    major: String,
    gpa: f32,
    thesis: String,
}


impl ShowInfo for Undergrad {
    fn show_info(&self) {
        println!("Undergrad: {}, Major: {}, GPA: {}", self.name, self.major, self.gpa);
    }
}


impl ShowInfo for GradStudent {
    fn show_info(&self) {
        println!(
            "Grad Student: {}, Major: {}, GPA: {}, Thesis: {}",
            self.name, self.major, self.gpa, self.thesis
        );
    }
}


struct Enrollment {
    students: Vec<Box<dyn ShowInfo>>,
}


impl Enrollment {
    fn new() -> Self {
        Enrollment { students: Vec::new() }
    }

    fn add<T: ShowInfo + 'static>(&mut self, student: T) {
        self.students.push(Box::new(student));
    }

    fn show_all(&self) {
        for s in &self.students {
            s.show_info();
        }
    }
}

fn main() {
    let u = Undergrad {
        name: "Alex".to_string(),
        major: "Computer Science".to_string(),
        gpa: 3.5,
    };

    let g = GradStudent {
        name: "Sam".to_string(),
        major: "Math".to_string(),
        gpa: 3.9,
        thesis: "AI Research".to_string(),
    };

    let mut enrollment = Enrollment::new();
    enrollment.add(u);
    enrollment.add(g);
    enrollment.show_all();
}
