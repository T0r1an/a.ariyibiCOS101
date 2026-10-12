use std::io;

fn main() {
    // 1. The Welcome Banner
    println!("=============================================");
    println!("=============================================");
    println!("=============================================");
    println!("WELCOME THE AMAZING DIGITAL APS LEVEL CHECKER");
    println!("=============================================");
    println!("=============================================");
    println!("=============================================");
    
    // 2. The Database
    let aps_database = vec![
        ("Intern", "APS 1-2"),
        ("Paralegal", "APS 1-2"),
        ("Placement", "APS 1-2"),
        ("Administrator", "APS 3-5"),
        ("Research Assistant", "APS 3-5"),
        ("Junior Associate", "APS 3-5"),
        ("Classroom Teacher", "APS 3-5"),
        ("Senior Administrator", "APS 5-8"),
        ("PhD Candidate", "APS 5-8"),
        ("Associate", "APS 5-8"),
        ("Senior Teacher", "APS 5-8"),
        ("Office Manager", "EL1 8-10"),
        ("Post-Doc Researcher", "EL1 8-10"),
        ("Senior Associate 1-2", "EL1 8-10"),
        ("Leading Teacher", "EL1 8-10"),
        ("Director", "EL2 10-13"),
        ("Senior Lecturer", "EL2 10-13"),
        ("Senior Associate 3-4", "EL2 10-13"),
        ("Deputy Principal", "EL2 10-13"),
        ("CEO", "SES"),
        ("Dean", "SES"),
        ("Partner", "SES"),
        ("Principal", "SES")
    ];

    println!("Enter the staff role to validate their APS level:");

    // 3. The Input Logic
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let role = input.trim();
    let mut is_found = false;

    // 4. The Search Loop
    for i in 0..aps_database.len() {
        if aps_database[i].0 == role {
            println!("Validation successful: A(n) {} holds the position of {}.", role, aps_database[i].1);
            is_found = true;
            break; 
        }
    }

    if !is_found {
        println!("Role not found. Please check the spelling and try again.");
    }
}
