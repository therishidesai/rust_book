
fn first_word(s: &str) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}

	&s[..]
}

fn main() {
    let s = String::from("hello world");

    let word = first_word(&s);
	let word1 = first_word("hello world");
    //s.clear(); // error!

    println!("the first word is: {}", word);
	println!("the first word is: {}", word1);
}
