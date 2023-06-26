fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    {
        let s = String::from("hello"); // s comes into scope

        takes_ownership(s); // s's value moves into the function...
                            // ... and so is no longer valid here

        let x = 5; // x comes into scope

        makes_copy(x); // x would move into the function,
                       // but i32 is Copy, so it's okay to still
                       // use x afterward
    } // Here, x goes out of scope, then s. But because s's value was moved, nothing
      // special happens.

    {
        // gives_ownership moves its return
        // value into s1
        let s1 = gives_ownership();
        println!("{s1}");

        // s2 comes into scope
        let s2 = String::from("hello");
        println!("{s2}");

        // s2 is moved into
        // takes_and_gives_back, which also
        // moves its return value into s3
        let s3 = takes_and_gives_back(s2);
        println!("{s3}");
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
      // happens. s1 goes out of scope and is dropped.

    // References & Borrows
    {
        let mut x: Box<i32> = Box::new(1);
        let _a: i32 = *x; // *x reads the heap value, so a = 1
        *x += 1; // *x on the left-side modifies the heap value,
                 // so x points to the value 2

        let r1: &Box<i32> = &x; // r1 points to x on the stack
        let _b: i32 = **r1; // two dereferences get us to the heap value

        let r2: &i32 = &*x; // r2 points to the heap value directly
        let _c: i32 = *r2; // so only one dereference is needed to read it
    }
    {
        let x: Box<i32> = Box::new(-1);
        let x_abs1 = i32::abs(*x); // explicit dereference
        let x_abs2 = x.abs(); // implicit dereference
        assert_eq!(x_abs1, x_abs2);

        let r: &Box<i32> = &x;
        let r_abs1 = i32::abs(**r); // explicit dereference (twice)
        let r_abs2 = r.abs(); // implicit dereference (twice)
        assert_eq!(r_abs1, r_abs2);

        let s = String::from("Hello");
        let s_len1 = str::len(&s); // explicit reference
        let s_len2 = s.len(); // implicit reference
        assert_eq!(s_len1, s_len2);
    }
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}