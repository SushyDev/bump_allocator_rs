fn main() {
    println!("Hello, world!");
}

fn construct() -> idk {
    buffer = create_buffer();
    pointer = 0;

    idk {
        buffer,
        pointer,
    }
}

fn create_buffer() -> Vec<u8> {
    let size = 1024; // 1 KB buffer
    let buffer = vec![0u8; size];
    buffer
}
