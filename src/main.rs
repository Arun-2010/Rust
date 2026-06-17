/* 
fn main() {
    let ans: u32 = sum(1, 2);
    println!("The sum is {}", ans);
}

fn sum(a: u32, b: u32) -> u32 {
    return a + b;
}
*/



// boolean

/* 
fn main(){
    print!("{}",is_even(5));
}

fn is_even(n:u32)->bool{
    return n%2 == 0;
}

*/

// string 

fn main(){
    let name:String = String::from("arun");
    print!("Hello, {}!", name);
}



//Ownership in Rust is one of the core ideas that makes Rust fast and memory‑safe without a garbage collector

1.
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;   // ownership moves to s2

    println!("{}", s2);
    // println!("{}", s1);  // ❌ ERROR: s1 no longer owns the string
}

/*What happened?
s1 owns the string "hello".

When we do let s2 = s1;, ownership moves to s2.

s1 becomes invalid.

Only one owner exists at a time.
*/


fn main() {
    let s = String::from("rust");
    take_ownership(s);   // ownership moves into the function

    // println!("{}", s);  // ❌ ERROR: s is no longer valid
}

fn take_ownership(x: String) {
    println!("{}", x);
} // x is dropped here

/*
Passing s into the function moves ownership.

After the function ends, the value is dropped.
 */


 //Returning Ownership
fn main() {
    let s1 = String::from("hello");
    let s2 = gives_back(s1);  // ownership moves back

    println!("{}", s2);
}

fn gives_back(s: String) -> String {
    s   // ownership returned
}

//Rust lets you pass ownership around like a baton.
