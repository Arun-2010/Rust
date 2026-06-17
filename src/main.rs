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

// Example 1: Ownership Transfer
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


// ✅ Borrowing Rules (super simple)

Rule 1: You can have any number of immutable references (&T)
They are read‑only, so no conflict.

Rule 2: You can have only ONE mutable reference (&mut T) at a time
This prevents data races.

Rule 3: You cannot mix mutable and immutable references at the same time
Either:

many readers
OR

one writer
but not both.

✅ Example 1: Immutable Borrowing (allowed)
rust
fn main() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2); // both can read
}
✔ Many immutable borrows
✔ No one is modifying the data
✔ Safe



❌ Example 2: Mutable + Immutable (NOT allowed)
rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;      // immutable borrow
    let r2 = &mut s;  // mutable borrow

    println!("{}", r1); // ❌ ERROR
}
Rust stops you because:

r1 is reading

r2 wants to write

This could cause unpredictable behavior



✅ Example 3: Only One Mutable Borrow (allowed)
rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;  // one mutable borrow
    r1.push_str(" world");

    println!("{}", r1);
}
✔ Only one writer
✔ Safe to modify



⭐ Example 4: Borrowing in Functions
rust
fn main() {
    let s = String::from("rust");

    print_length(&s); // borrow, not move

    println!("{}", s); // still valid
}

fn print_length(x: &String) {
    println!("Length = {}", x.len());
}
Passing &s means:

Ownership stays with main

Function only borrows the value