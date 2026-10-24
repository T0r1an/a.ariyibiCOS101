use std::fs::File;
use std::io::Write;

fn main() {
    let mut names: Vec<&str> = Vec::new();
    names.push("Oluchi Mordi");
    names.push("Adams Aliyu");
    names.push("Shania Bolade");
    names.push("Adekunle Gold");
    names.push("Blanca Edemoh");

    let mut matrics: Vec<&str> = Vec::new();
    matrics.push("26120111054");
    matrics.push("26120111045");
    matrics.push("26120111504");
    matrics.push("26120110154");
    matrics.push("26120115014");

    let mut depts: Vec<&str> = Vec::new();
    depts.push("Accounting");
    depts.push("Economics");
    depts.push("Computer");
    depts.push("Electrical");
    depts.push("Mechanical");

    let mut levels: Vec<i32> = Vec::new();
    levels.push(300);
    levels.push(100);
    levels.push(200);
    levels.push(200);
    levels.push(100);

    let mut file = File::create("pau_smis.txt").expect("create failed");
    
    println!("Student Name, Matric. Number, Department, Level");
    writeln!(file, "Student Name, Matric. Number, Department, Level").expect("write failed");

    for i in 0..names.len() {
        println!("{}, {}, {}, {}", names[i], matrics[i], depts[i], levels[i]);
        writeln!(file, "{}, {}, {}, {}", names[i], matrics[i], depts[i], levels[i]).expect("write failed");
    }
}