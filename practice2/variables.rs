// Practice 2 - Rust by Practice
// Sections 3, 4, 6

// ===============================
// Section 3 – Mutable
// ===============================
fn section3() {
    let mut x: i32 = 1;
    x = 7;
    x += 3;

    println!("Section 3 result: x = {}", x);
}

// ===============================
// Section 4 – Scope
// ===============================
fn section4() {
    let x: i32 = 10;

    {
        let y: i32 = 5;
        println!("Section 4 inside scope: x = {}, y = {}", x, y);
    }

    println!("Section 4 outside scope: x = {}", x);
}

// ===============================
// Section 6 – Constants
// ===============================
const MAX_POINTS: u32 = 100_000;

fn section6() {
    println!("Section 6 constant value: {}", MAX_POINTS);
}

// ===============================
// Main
// ===============================
fn main() {
    section3();
    section4();
    section6();

    println!("Practice 2 completed successfully!");
}