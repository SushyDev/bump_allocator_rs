// How it Works
// Pointer-Based: Maintains a single pointer to the next available memory location within a big block.
// Linear Allocation: When memory is requested, the pointer "bumps" forward by the requested size (plus alignment).
// No Individual Free: It can't free single items because that would break the linear sequence; memory is only reclaimed when the entire arena is reset.
// Chunking (Optional): To handle larger needs, some bump allocators allocate new, often larger, chunks when the current one fills up.

fn main() {
    println!("Hello, world!");
}

fn construct() -> idk {
    buffer = create_buffer();
    pointer = 0;

    struct {
        buffer,
        pointer,
    }
}

fn create_buffer() -> Vec<u8> {
    let size = 1024; // 1 KB buffer
    let buffer = vec![0u8; size];
    buffer
}

fn allocate(buffer, pointer, data) {
    size = len(data);

    buffer[] = data;
    pointer += size;

    idk {
        buffer,
        pointer,
    }
}

fn free(buffer, pointer) {
    pointer = 0;
    buffer = create_buffer();

    idk {
        buffer,
        pointer,
    }
}
