# Learning_Rust

## Here I Write My Notes for Now

### Stack and Heap in Rust
Rust primarily uses two memory regions: the Stack and the Heap. Understanding how these regions work helps in writing efficient and safe Rust programs.

---

### 1. Understanding the Stack
The Stack is a region of memory that operates in a **Last In, First Out (LIFO)** manner. It is fast, automatically managed, and does not require manual memory allocation.

#### Characteristics of the Stack:
- Memory is allocated and deallocated automatically
- Fast access due to predictable memory layout
- Limited in size, ideal for small/fixed-size data
- Stores function calls, local variables, and arguments

##### Example:
```rust
fn main() {
    let x: i32 = 10;  // Stack-allocated
    let y: i32 = 20;  // Stack-allocated
    println!("Sum: {}", x + y);
}
