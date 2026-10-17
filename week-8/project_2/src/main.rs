fn main() {
    let mut candidates: Vec<(&str, i32)> = Vec::new();
    
    candidates.push(("Chidi", 4));
    candidates.push(("Amina", 9));
    candidates.push(("Tobi", 2));
    candidates.push(("Ngozi", 12));
    candidates.push(("Emeka", 7));
    candidates.push(("Folake", 12));
    candidates.push(("Ibrahim", 5));
    candidates.push(("Grace", 10));
    candidates.push(("Samuel", 11));
    
    println!("We have {} candidates applying for the EY position.", candidates.len());
    println!("Let's check their years of experience.\n");
    
    for person in &candidates {
        println!("Candidate: {}, Experience: {} years", person.0, person.1);
    }
    
    let mut highest_years = 0;
    
    for person in &candidates {
        if person.1 > highest_years {
            highest_years = person.1;
        }
    }
    
    println!("\nThe highest experience found is {} years.", highest_years);
    println!("These are the developers that made the cut:");
    
    for person in &candidates {
        if person.1 == highest_years {
            println!("- {} gets the job!", person.0);
        }
    }
}