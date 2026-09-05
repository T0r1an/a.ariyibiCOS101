fn main () {
	println!("APQ reprsents amount per quantity");
	let amount_1 = 450000.00;
	println!("APQ-1 is mathematically equivalent to, ₦{}", amount_1);
	let amount_2 = 1500000.00;
	println!("APQ-2 is mathematically equivalent to, ₦{}", amount_2);
	let amount_3 = 750000.00;
	println!("APQ-3 is mathematically equivalent to, ₦{}", amount_3);
	let amount_4 = 2850000.00;
	println!("APQ-4 is mathematically equivalent to, ₦{}", amount_4);
	let amount_5 = 250000.00;
	println!("APQ_5 is mathematically equivalent to, ₦{}", amount_5);
	
	let t1ty_1 = 2;
	println!("Quantity_1 is mathematically equivalent to, {}", t1ty_1);
	let t2ty_2 = 1;
    println!("Quantity_2 is mathematically equivalent to, {}",t2ty_2 );
	let t3ty_3 = 3;
    println!("Quantity_3 is mathematically equivalent to, {}",t3ty_3 );
	let t4ty_4 = 3;
    println!("Quantity_4 is mathematically equivalent to, {}",t4ty_4 );
	let t5ty_5 = 1;
    println!("Quantity_5 is mathematically equivalent to, {}",t5ty_5 );

    let total = amount_1 + amount_2 + amount_3 + amount_4 + amount_5;

    println!("Total APQ = ₦{}", total);	

    let sales_1 = amount_1 * t1ty_1 as f64;
    println!("sales_1 =₦{}", sales_1);

    let sales_2 = amount_2 * t2ty_2 as f64;
    println!("sales_2 =₦{}", sales_2);

    let sales_3 = amount_3 * t3ty_3 as f64;
    println!("sales_3 =₦{}", sales_3);

    let sales_4 = amount_4 * t4ty_4 as f64;
    println!("sales_4 =₦{}", sales_4);

    let sales_5 = amount_5 * t5ty_5 as f64;
    println!("sales_5 =₦{}", sales_5); 

    let sum = sales_1 + sales_2 + sales_3 + sales_4 + sales_5;

    let sumuan = t1ty_1 + t2ty_2 + t3ty_3 + t4ty_4 + t5ty_5;
    
    let average = sum/sumuan as f64;

    println!("Average is therefore equal to ={}",average );
}
