// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

#[derive(Debug)]
pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard {

    const GRADE_QUANT: f32 = 0.1875;

    pub fn print_grade_num(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }

    pub fn print_grade_alpha(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, Self::convert_grade_to_alpha(&self.grade))
    }

    fn convert_grade_to_alpha(grade: &f32) -> String {
        let base_grades: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F'];

        let mut pos: f32 = 1.0;

        for base_grade in base_grades.iter() {
            // stage '+'
            pos += Self::GRADE_QUANT;
            if *grade < pos {
                return format!("{}+", base_grade);
            }
            // stage 'bare'
            pos += Self::GRADE_QUANT * 2.0;
            if *grade < pos {
                return format!("{}", base_grade);
            }
            // stage '-'
            pos += Self::GRADE_QUANT;
            if *grade < pos {
                return format!("{}-", base_grade);
            }
        }

        return String::from("Som-Tink-Wonk!");
    }
}

fn main() {

    let report_card = ReportCard {
        grade: 2.1,
        student_name: "Tom Wriggle".to_string(),
        student_age: 12,
    };
    assert_eq!(
        report_card.print_grade_num(),
        "Tom Wriggle (12) - achieved a grade of 2.1"
    );

    // TODO: Make sure to change the grade here after you finish the exercise.
    let report_card = ReportCard {
        grade: 1.1,
        student_name: "Gary Plotter".to_string(),
        student_age: 11,
    };
    assert_eq!(
        report_card.print_grade_alpha(),
        "Gary Plotter (11) - achieved a grade of A+"
    );

    let report_card = ReportCard {
        grade: 5.4,
        student_name: "Twoj Stary".to_string(),
        student_age: 55,
    };

    println!("{}", report_card.print_grade_alpha());
}
