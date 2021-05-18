// Circular references:
// Two objects referencing each other
//
// Example:
// Online learning platform.
// Students - courses
//
//

#[derive(Debug)]
struct Student {
    name: String,
}

impl Student {
    fn courses(&self, platform: Platform) -> Vec<String> {
        platform
            .enrollments
            .iter()
            .filter(|&e| e.student.name == self.name)
            .map(|e| e.course.name.clone())
            .collect()
    }
}

#[derive(Debug)]
struct Course {
    name: String,
}

#[derive(Debug)]
struct Enrollment<'a> {
    student: &'a Student,
    course: &'a Course,
}

impl<'a> Enrollment<'a> {
    fn new(student: &'a Student, course: &'a Course) -> Enrollment<'a> {
        Enrollment { student, course }
    }
}

#[derive(Debug)]
struct Platform<'a> {
    enrollments: Vec<Enrollment<'a>>,
}

impl<'a> Platform<'a> {
    fn new() -> Platform<'a> {
        Platform {
            enrollments: Vec::new(),
        }
    }

    fn enroll(&mut self, student: &'a Student, course: &'a Course) {
        self.enrollments.push(Enrollment::new(student, course));
    }
}

fn main() {
    let mut platform = Platform::new();

    let student = Student {
        name: "Frank".into(),
    };
    let course = Course {
        name: "Rust Course".into(),
    };

    platform.enroll(&student, &course);

    println!("{:?}", platform);

    let student2 = Student {
        name: "Grace".into(),
    };
    let course2 = Course {
        name: "Rust Advanced Course".into(),
    };

    platform.enroll(&student2, &course2);

    println!("{:?}", platform);

    for course in student2.courses(platform) {
        println!("{} is taking {}", student2.name, course);
    }
}
