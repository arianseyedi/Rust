// Author: Arian Seyedi
fn main() {
	// testing Q1
	println!("***-----------Testing Q1 begins:------------***\n");
	let mut s1 = String::from("string1 is attached to ");
	let mut s2 = String::from("string2.");
	let mut s1s2 = concat(&s1, &s2);
	println!("string 1 is: '{}' with length {}", s1, s1.len());
	println!("string 2 is: '{}' with length {}", s2, s2.len());
	println!("result of concatenation is: '{}' with length {}\n", s1s2, s1s2.len());
	
	s1 = String::from("<!");
	s2 = String::from("");
	s1s2 = concat(&s1, &s2);
	println!("string 1 is: '{}' with length {}", s1, s1.len());
	println!("string 2 is: '{}' with length {}", s2, s2.len());
	println!("result of concatenation is: '{}' with length {}\n", s1s2, s1s2.len());
	
	// testing Q2
	println!("\n***-----------Testing Q2 begins:------------***\n");
	let arr = [1.23, 2.98, -4.36, 9.1245, 15.0];
	println!("the input array is: [{}, {}, {}, {}, {}] \n", arr[0], arr[1], arr[2], arr[3], arr[4]);
	//
	let mut result = avg_of_slice(&arr, (0, arr.len()));
	println!("\nthe average of all elements: {} \n", result);

	result = avg_of_slice(&arr, (0, 2));
	println!("\nthe average of first 2 elements: {} \n", result);
	
	result = avg_of_slice(&arr, (arr.len()-2, arr.len()));
	println!("\nthe average of last 2 elements: {} \n", result);
	
	result = avg_of_slice(&arr, (arr.len()-1, arr.len()));
	println!("\nthe average of The last elements: {} \n", result);
	
	// testing Q3
	println!("\n***-----------Testing Q3 begins:------------***\n");
	let mut arr2 = [1, 2, -9, 0, 12_000_000_000_000_000];
	println!("the input array is: [{}, {}, {}, {}, {}] \n", arr2[0], arr2[1], arr2[2], arr2[3], arr2[4]);
	let refto = signum(&mut arr2);
	println!("the signed array is: [{}, {}, {}, {}, {}] \n", refto[0], refto[1], refto[2], refto[3], refto[4]);
}

fn concat (str1: &String, str2: &String) -> String
{
	let mut combo = String::from(""); // initialize
	combo.push_str(str1); // concat 1st argument
	combo.push_str(str2); // concat 2nd argument
	combo
}

fn avg_of_slice(arr: &[f32], bounds:(usize, usize)) -> f32
{
	let mut avg = 0.0;
	let slice = &arr[bounds.0..bounds.1];
	for i in slice.iter() {
		avg = avg + i;
	}
	avg / slice.len() as f32
}

fn signum(arr: &mut [i64]) -> &[i64] 
{
// mutate input array to values 0, 1 or -1 based on 
//the sign at the respective location
	for i in 0..arr.len(){
		if arr[i] > 0 {
			arr[i] = 1;
		}
		else if arr[i] < 0 {
			arr[i] = -1;
		}
		else {
			arr[i] = 0;
		}
	}
	arr // return
}
