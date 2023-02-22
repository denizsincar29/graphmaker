use std::{io, io::Write};
use urlencoding::encode;

fn main() {
	let stdin = io::stdin();
	let mut data=Vec::<String>::with_capacity(10);
	let mut values=Vec::<String>::with_capacity(10);
	let mut descryption=String::new();
	println!("введите название вашего графика/диаграммы");
	stdin.read_line(&mut descryption).unwrap();
	descryption=descryption.trim().to_string();
	let mut min_value=String::new();
	let mut max_value=String::new();
	println!("введите минимальное значение Y");
	stdin.read_line(&mut min_value).unwrap();
	min_value.trim().parse::<f32>().expect("введите число!");
	println!("введите максимальное значение Y");
	stdin.read_line(&mut max_value).unwrap();
	max_value.trim().parse::<f32>().expect("введите число!");

	println!("поштучно введите столбики диаграммы");
	println!("Если всё ввели, нажмите enter, не вводя ничего");
	let mut i=0;

	loop {
		i+=1;
		let mut datum=String::new();
		println!("как называется {}-й столбец диаграммы?", i);
		stdin.read_line(&mut datum).unwrap();
		datum=datum.trim().to_string();
		if datum.is_empty() {break;}
		data.push(datum);
		let mut datum=String::new();
		println!("введите значение y");
		stdin.read_line(&mut datum).unwrap();
		datum=datum.trim().to_string();
		if datum.is_empty(){data.pop(); break;}
		datum.trim().parse::<f32>().expect("введите число");
		values.push(datum);
	} // loop ended!
	let result_data=vec![data.join("\t"), values.join("\t")];
	let mut file=std::fs::File::create("url.txt").unwrap();
	writeln!(file, "https://accessibleGraphs.org/view/index.html?data={}&descryption={}&minValue={}&maxValue={}&instrumentType=synthesizer", encode(&result_data.join("\r\n")[..]), encode(&descryption), min_value, max_value)
		.expect("could not write file!");
} // main ended!	