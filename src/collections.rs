#[allow(dead_code)]
pub mod collections {
  pub fn execute_vector() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1); // update a vector
    v.push(2);
    v.push(3);
    v.push(4);
    println!("Vector demo 1: {:?}", v);

    let v2 = vec![1, 2, 3]; // declaring vector with initial values
    println!("Vector demo 2: {:?}", v2);

    //Two ways to reference a vector element
    // &v[number] or v.get(number)
    
    let third: &i32 = &v[2];
    println!("Third element should be {}.", third);
    
    match v.get(2) {
      Some(third_element) => println!("The third element is {}", third_element),
      _ => println!("There is no third element")
    }

    // Iteration
    for i in &v2 {
      println!("{}", i);
    }

    //Iteration with mutable reference
    // For when making changes to all elements
    let mut v3 = vec![1, 3, 5];
    for i in &mut v3 {
      *i += 50; // *i is an immutable reference
    }

    for i in v3 {
      println!("{}", i);
    }


    // Using an Enum to Store Multiple Types

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);

    for i in &row {
      match i {
        SpreadsheetCell::Int(val) => println!("{}", val),
        SpreadsheetCell::Float(val) => println!("{}", val),
        SpreadsheetCell::Text(val) => println!("{}", val),
      };
    }
  }

  pub fn execute_string() {

  }

  pub fn execute_hash_maps() {

  }
}