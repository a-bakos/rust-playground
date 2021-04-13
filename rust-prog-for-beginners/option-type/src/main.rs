#[derive(Debug)]
struct Locker {
    student: String,
    locker: Option<i32>,
}

fn main() {
    let student = Locker {
        student: String::from("Frank"),
        locker: Some(14),
    };

    match student.locker {
        Some(locker) => println!("{} is the owner of locker #{}", student.student, locker),
        None => println!("No locker has been assigned to this student."),
    }

    println!(
        "Student {:?} is the owner of locker #{:?}",
        student.student, student.locker
    );
}
