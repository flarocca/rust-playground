/*
You are a developer for a university. Your current project is to develop a system for students to find courses they share with friends. The university has a system for querying courses students are enrolled in, returned as a list of (ID, course) pairs.

Write a function that takes in a collection of (student ID number, course name) pairs and returns, for every pair of students, a collection of all courses they share.


Sample Input:

enrollments1 = [
  ["58", "Linear Algebra"],
  ["94", "Art History"],
  ["94", "Operating Systems"],
  ["17", "Software Design"],
  ["58", "Mechanics"],
  ["58", "Economics"],
  ["17", "Linear Algebra"],
  ["17", "Political Science"],
  ["94", "Economics"],
  ["25", "Economics"],
  ["58", "Software Design"],
]

Sample Output (pseudocode, in any order):

find_pairs(enrollments1) =>
{
  "58,17": ["Software Design", "Linear Algebra"]
  "58,94": ["Economics"]
  "58,25": ["Economics"]
  "94,25": ["Economics"]
  "17,94": []
  "17,25": []
}



Additional test cases:

Sample Input:

enrollments2 = [
  ["0", "Advanced Mechanics"],
  ["0", "Art History"],
  ["1", "Course 1"],
  ["1", "Course 2"],
  ["2", "Computer Architecture"],
  ["3", "Course 1"],
  ["3", "Course 2"],
  ["4", "Algorithms"]
]



Sample output:

find_pairs(enrollments2) =>
{
  "1,0":[]
  "2,0":[]
  "2,1":[]
  "3,0":[]
  "3,1":["Course 1", "Course 2"]
  "3,2":[]
  "4,0":[]
  "4,1":[]
  "4,2":[]
  "4,3":[]
}
("4","0"): [],
("4","3"): [],
("4","1"): [],
("0","1"): [],
("4","2"): [],
("0","2",): [],
("0","3",): [],
("1","2",): [],
("1","3",): ["Course 1","Course 2",],
("2","3",): [],
}

Sample Input:
enrollments3 = [
  ["23", "Software Design"],
  ["3", "Advanced Mechanics"],
  ["2", "Art History"],
  ["33", "Another"],
]


Sample output:

find_pairs(enrollments3) =>
{
  "23,3": []
  "23,2": []
  "23,33":[]
  "3,2":  []
  "3,33": []
  "2,33": []
}

All Test Cases:
find_pairs(enrollments1)
find_pairs(enrollments2)
find_pairs(enrollments3)

Complexity analysis variables:

n: number of student,course pairs in the input
s: number of students
c: total number of courses being offered (note: The number of courses any student can take is bounded by a small constant)
*/
use std::collections::HashMap;

fn main() {
    let enrollments1 = vec![
        vec!["58", "Linear Algebra"],
        vec!["94", "Art History"],
        vec!["94", "Operating Systems"],
        vec!["17", "Software Design"],
        vec!["58", "Mechanics"],
        vec!["58", "Economics"],
        vec!["17", "Linear Algebra"],
        vec!["17", "Political Science"],
        vec!["94", "Economics"],
        vec!["25", "Economics"],
        vec!["58", "Software Design"],
    ];

    let enrollments2 = vec![
        vec!["0", "Advanced Mechanics"],
        vec!["0", "Art History"],
        vec!["1", "Course 1"],
        vec!["1", "Course 2"],
        vec!["2", "Computer Architecture"],
        vec!["3", "Course 1"],
        vec!["3", "Course 2"],
        vec!["4", "Algorithms"],
    ];

    let enrollments3 = vec![
        vec!["23", "Software Design"],
        vec!["3", "Advanced Mechanics"],
        vec!["2", "Art History"],
        vec!["33", "Another"],
    ];

    let pairs = find_pairs(enrollments1);
    println!("1- Pairs: {:#?}", pairs);
    let pairs = find_pairs(enrollments2);
    println!("2- Pairs: {:#?}", pairs);
    let pairs = find_pairs(enrollments3);
    println!("3- Pairs: {:#?}", pairs);
}

fn find_pairs(enrollments: Vec<Vec<&str>>) -> HashMap<(String, String), Vec<String>> {
    let reduced = reduce_to_course_per_students(enrollments);
    get_pairs(reduced)
}

fn find_pairs_v2(enrollments: Vec<Vec<&str>>) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::new();

    for outer in enrollments.iter() {
        for inner in enrollments.iter() {
            if outer[0] != inner[0] {
                let composed_key = (outer[0].to_owned(), inner[0].to_owned());
                let inverted_composed_key = (inner[0].to_owned(), outer[0].to_owned());

                if !result.contains_key(&composed_key)
                    && !result.contains_key(&inverted_composed_key)
                {
                    let coincidences: Vec<String> =
                        extract_coincidenses(inner[1].to_vec(), inner_value.to_vec());
                    result.insert(composed_key, coincidences);
                }
            }
        }   
    }

    result
}

fn reduce_to_course_per_students(enrollments: Vec<Vec<&str>>) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::<String, Vec<String>>::new();

    for item in enrollments.into_iter() {
        let key = item[0].to_owned();
        let value = item[1].to_owned();
        if !result.contains_key(&key) {
            result.insert(key, vec![value]);
        } else {
            let existing_item = result.remove(&key);

            let mut new_students = existing_item.unwrap();
            new_students.push(value);
            result.insert(key, new_students);
        }
    }

    result
}

fn get_pairs(students: HashMap<String, Vec<String>>) -> HashMap<(String, String), Vec<String>> {
    let mut result = HashMap::new();

    for (key, value) in students.iter() {
        for (inner_key, inner_value) in students.iter() {
            if key != inner_key {
                let composed_key = (key.to_owned(), inner_key.to_owned());
                let inverted_composed_key = (inner_key.to_owned(), key.to_owned());

                if !result.contains_key(&composed_key)
                    && !result.contains_key(&inverted_composed_key)
                {
                    let coincidences: Vec<String> =
                        extract_coincidenses(value.to_vec(), inner_value.to_vec());
                    result.insert(composed_key, coincidences);
                }
            }
        }
    }

    result
}

fn extract_coincidenses(left: Vec<String>, right: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    let (target, dest) = if right.len() >= left.len() {
        (left, right)
    } else {
        (right, left)
    };

    for item in target {
        if dest.contains(&item) {
            result.push(item);
        }
    }

    result
}
