use std::fs::File;
use std::io::Write;

fn main() {
    let mut commissioners: Vec<&str> = Vec::new();
    commissioners.push("Aigbogun Alamba Daudu");
    commissioners.push("Murtala Afeez Bandu");
    commissioners.push("Okorocha Calistus Ogbona");
    commissioners.push("Adewale Jimoh Akanbi");
    commissioners.push("Osazuwa Faith Etieye");

    let mut ministries: Vec<&str> = Vec::new();
    ministries.push("Internal Affairs");
    ministries.push("Justice");
    ministries.push("Defense");
    ministries.push("Power & Steel");
    ministries.push("Petroleum");

    let mut zones: Vec<&str> = Vec::new();
    zones.push("South West");
    zones.push("North East");
    zones.push("South South");
    zones.push("South West");
    zones.push("South East");

    let mut file = File::create("efcc_merge.txt").expect("create failed");

    println!("Commissioner, Ministry, Zone");
    writeln!(file, "Commissioner, Ministry, Zone").expect("write failed");

    for i in 0..commissioners.len() {
        println!("{}, {}, {}", commissioners[i], ministries[i], zones[i]);
        writeln!(file, "{}, {}, {}", commissioners[i], ministries[i], zones[i]).expect("write failed");
    }
}