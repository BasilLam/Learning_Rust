use std::io;

fn user_input_str() -> String {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let user_input = user_input.trim().to_lowercase();
    return user_input
}

fn duplicate_control(course: &String, courses: &Vec<String>) -> bool {
    for existing_course in courses.iter() {
        if existing_course.to_lowercase() == course.to_lowercase() {
            return true;
        }
    }
    return false;
}

fn add_course(courses: &mut Vec<String>) {
    loop {
        println!("Please enter course title");
        let mut course_title = String::new();
        io::stdin().read_line(&mut course_title).expect("Failed to read line");
        let course_title = course_title.trim();

        if duplicate_control(&course_title.to_string(), courses) {
            println!("This course already exists. Do you want to enter another course? [Yy/Nn]");
            //TODO circular logic to confirm whether user wants to add another course
        } else {
            if course_title.is_empty() || course_title.parse::<f64>().is_ok() {
                println!("Invalid course name. Please enter a valid course name.");
            } else {
                courses.push(course_title.to_string());
                println!("You have added the course: {}", course_title);
                break;
            }
        }
    }
}

fn course_list(courses: &mut Vec<String>) {
    loop {
        println!("Do you want to add another course? [Yy/Nn]");

        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let user_input = user_input.trim();

        if user_input == "Y" || user_input == "y" || user_input == "Yy" {
            add_course(courses); 
        } else {
            println!("No courses will be added.\n\n\n");
            break;
        }
    }
    let course_list = courses.iter()
                             .map(|course| format!("    - {}", course))
                             .collect::<Vec<String>>()
                             .join("\n");
    println!("Courses offered are: \n{}", course_list);
}

fn course_finalize(courses: &mut Vec<String>) {
    loop {
        let c = courses.iter()
                       .map(|course| format!("    - {}", course))
                       .collect::<Vec<String>>()
                       .join("\n");
        println!("Is this list correct? [Yy/Nn]\n{}", c);
        
        let mut user_input = user_input_str();

        if user_input == "y" {
            break;
        } else if user_input == "n" {
            println!("Do you want to [ADD] or [REMOVE] a course?");
            user_input = user_input_str();

            if user_input == "add" {
                add_course(courses);
            } else if user_input == "remove" {
                println!("Which course do you want to remove?");
                let course_to_remove = user_input_str();
                if let Some(pos) = courses.iter().position(|x| x.to_lowercase() == course_to_remove) { // if course_to_remove in list of courses 
                    courses.remove(pos);
                    println!("Remove course: {}", course_to_remove);
                } else {
                    println!("Course not found");
                }

            }
        }
    }
}

fn main() {
    let mut courses: Vec<String> = vec!["Rust".to_string(), "Python".to_string(), "Go".to_string()];

    // Welcome message start
    println!("Welcome to Student Management System 0.0.0.1 - 
    The base courses are \n    - {}, \n    - {}, \n    - {}.\n\n",
    courses[0], courses[1], courses[2]);
    // Welcome message end

    course_list(&mut courses); //Prompt user to set courses
    course_finalize(&mut courses) //Finalize course list
    

}
