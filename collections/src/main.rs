use std::collections::HashMap;

fn main() {
    // Format string using 'push_str'
    let mut s = String::from("Hello, ");
    s.push_str("World!");
    println!("{}", s);

    // Format string using '+'
    let s1 = String::from("Goodbye, ");
    let s2 = String::from("Mars!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used, but s2 hasn't
    println!("{}", s3);

    // Format string using 'format!'
    let t1 = String::from("tic");
    let t2 = String::from("tac");
    let t3 = String::from("toe");
    let t = format!("{}-{}-{}", t1, t2, t3);
    println!("{}", t);

    // Print all characters in "Mars!"
    for c in s2.chars() {
        println!("{}", c);
    }

    // Collect teams and scores into a hash map
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    // Iterating over a hash map
    for (key, value) in &scores {
        println!("{}: {}", key, value)
    }

    // Overwriting a value
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // Check entry exists before inserting
    scores.entry(String::from("Purple")).or_insert(30);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // Add value to hash map using 'insert'
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); // field_name and field_value are invalid after this point

    // Update values based on previous values
    let text = "hello world goodbye world hello Mars";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", word_count);
}
