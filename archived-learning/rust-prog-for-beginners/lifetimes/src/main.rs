// Lifetimes allow you to store references within
// structures and sometimes needed when working
// with complicated functions.
// Lifetimes are basically the ownership model
// except you'll be explicitly annotating the code
// with lifetimes which is telling the compiler where
// the ownership lies.

#[derive(Debug)]
enum Answer {
    Yes,
    No,
}

// When we want to borrow and store information
// in the structure (for example), we need lifetimes
// specifier <'a>
// With this you're telling the compiler that
// &'a Answer borrowed data lives at least as long as
// the structure exists. Without it, compiler can't tell
// how long Answer will live. It could be deleted, but
// the struct still exists and the borrowed data is gone
// - which would be a memory error.
//
// Another way to look at it:
// when you use the lifetime annotations you're telling
// the compiler that you're creating an Answer outside
// of the Form, you're gonna let the Form borrow it,
// and then you're gonna delete the Form before you
// delete this Answer. So this Answer always has to
// exist even if this form gets deleted, otherwise the
// structure won't be able to borrow it
#[derive(Debug)]
// this defines the lifetime
struct Form<'a> {
    question: &'a Answer, // this uses the lifetime
}

// Lifetimes within functions
#[derive(Debug)]
struct Quiz {
    question: Answer,
}
// We're borrowing the returned Answer, and it's okay,
// because we're already borrowing Quiz and if we
// borrow more data out of Quiz, Quiz will still going
// to exist after the function finishes.
//
// Without lifetime specifiers we're gonna get an error
// because we have to pieces of borrowed data (Quiz) and
// the compiler can't know which one to return Answer from
// aka which quiz we're borrowing data from.
//
// So:
// - we're telling the compiler that our get_first_question
// function has borrowed data <'a> as specified by the 'a
// lifetime
// - then that 'a lifetime is part of quiz_1: &'a Quiz
// - and the answer we're returning is coming from quiz_1
fn get_first_question<'a>(quiz_1: &'a Quiz, quiz_2: &Quiz) -> &'a Answer {
    &quiz_1.question
}

// This exercise is from the Microsoft course:
// returns a reference to the value inserted in the vector.
fn copy_and_return<'a>(vector: &'a mut Vec<String>, value: &'a str) -> &'a String {
    vector.push(String::from(value));
    vector.get(vector.len() - 1).unwrap()
}

fn main() {
    let answer = Answer::Yes;
    let form = Form { question: &answer };
    println!("{:?}", form);

    // Microsoft exercise below:
    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();

    assert_eq!("Joe", copy_and_return(&mut names, &name1));
    assert_eq!("Chris", copy_and_return(&mut names, &name2));
    assert_eq!("Anne", copy_and_return(&mut names, &name3));

    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
    )
}
