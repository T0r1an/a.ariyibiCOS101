fn main () {
	println!("TV Depreciation Calculator");
	println!("Ms. Akudo Ijezie acquired a brand new TV set");

	let original_price: f64 = 210000.00;
	println!("Original price of the TV is mathematically equivalent to, ₦{}", original_price);

	let depreciation_rate: f64 = 5.00;
	println!("Depreciation rate is mathematically equivalent to, {}%", depreciation_rate);

	let number_of_years: i32 = 3;
	println!("Number of years is mathematically equivalent to, {}", number_of_years);

	let depreciation_percentage: f64 = depreciation_rate / 100.00;
	println!("Depreciation percentage is mathematically equivalent to, {}", depreciation_percentage);

	let remaining_value: f64 = 1.00 - depreciation_percentage;
	println!("Remaining value after depreciation is mathematically equivalent to, {}", remaining_value);

	let value_after_3_years: f64 = original_price * remaining_value.powi(number_of_years);
	println!("Value of the TV after 3 years is mathematically equivalent to, ₦{:.2}", value_after_3_years);
}