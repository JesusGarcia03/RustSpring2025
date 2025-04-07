//don't use closures

// In class Assignment

// Create a struct Student (major)
struct Student {
    major:String,
}
// Higher order functions update majors

fn update_majors(mut collection: Vec<Student>, behavior: fn(&mut Student, String)) -> Vec<Student> {
    for student in &mut collection {
        behavior(student, "Computer Science".to_string());
    }
    collection
}
// First Order functions, assign_major(student,major_declared)

fn assign_major(s: &mut Student, major: String) {
    s.major = major;
}
// create a vector of students1,2,3 and update all students major
fn main() {
    let students = vec![
        Student { major: String::new() },
        Student { major: String::new() },
        Student { major: String::new() },
    ];

    let updated_students = update_majors(students, assign_major);

    for (i, student) in updated_students.iter().enumerate() {
        println!("Student {}: Major - {}", i + 1, student.major);
    }
}
