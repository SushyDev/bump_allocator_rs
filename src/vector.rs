use core::{
    mem,
    ops::Index,
    ptr::{
        NonNull,
        self
    }
};

use alloc::alloc::{
    alloc,
    dealloc,
    Layout
};

pub struct Vec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Self{
            ptr: NonNull::dangling(),
            cap: 0,
            len: 0
        }
    }

    #[inline(always)]
    fn max_size() -> usize {
        // isize::MAX - (isize::MAX as usize % align)
        // n & !(divisor.wrapping_sub(1))
        // isIze max
        // diVisor with mem::align_of::<T>()

        let divisor: usize = mem::align_of::<T>();
        let n: usize = isize::MAX as usize;

        n & !(divisor-1)
    }

    fn get_layout(size: usize) -> Layout {
        if size == 0 {
            panic!("fail");
        }

        let max_size = Self::max_size();
        if size > max_size {
            panic!("poop {} {}", size, max_size);
        }

        let alignment = mem::align_of::<T>();

        unsafe {
            Layout::from_size_align_unchecked(size, alignment)
        }
    }

    fn with_capacity(cap: usize) -> Self {
        let size: usize = mem::size_of::<T>().checked_mul(cap).unwrap();
        let layout = Self::get_layout(size);

        let maybe_null = unsafe { alloc(layout) };
        let ptr = NonNull::new(maybe_null).expect("allocation error").cast::<T>();

        Self {
            ptr,
            cap,
            len: 0
        }
    }

    unsafe fn resize_unchecked(&mut self, new_cap: usize) {
        debug_assert!(new_cap >= self.len);

        let new_size: usize = mem::size_of::<T>().checked_mul(new_cap).unwrap();
        let new_layout = Self::get_layout(new_size);

        let new_maybe_null = unsafe { alloc(new_layout) };

        let new_ptr = NonNull::new(new_maybe_null).expect("allocation error");
        let old_ptr = self.ptr.cast::<u8>();

        let cur_len_size = self.len * mem::size_of::<T>();

        unsafe {
            ptr::copy_nonoverlapping(old_ptr.as_ptr(), new_ptr.as_ptr(), cur_len_size);
        }

        let cur_size = self.cap * mem::size_of::<T>();
        let cur_layout = Self::get_layout(cur_size);

        unsafe {
            dealloc(old_ptr.as_ptr(), cur_layout);
        }

        self.cap = new_cap;
        self.ptr = new_ptr.cast::<T>();
    }

    fn amortized_cap_for(&self, min_size: usize) -> usize {
        core::cmp::max(self.cap * 2, min_size)
    }

    fn grow(&mut self, additional_size: usize) {
        let requested_cap = self.len
            .checked_add(additional_size)
            .unwrap();

        let new_cap = self.amortized_cap_for(requested_cap);

        unsafe {
            self.resize_unchecked(new_cap);
        }
    }

    #[inline(always)]
    pub unsafe fn push_unchecked(&mut self, data: T) {
        debug_assert!(self.len < self.cap);

        unsafe {
            self.ptr
                .add(self.len)
                .write(data);
        }

        self.len += 1
    }

    pub fn push(&mut self, data: T) {
        if self.len == self.cap {
            self.grow(1);
        }

        unsafe {
            self.push_unchecked(data);
        }
    }

    #[inline(always)]
    pub unsafe fn get_unchecked(&self, idx: usize) -> &T {
        debug_assert!(self.len > idx);

        unsafe {
            self.ptr.add(idx).as_ref()
        }
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        if self.len > idx {
            Some(unsafe {
                self.get_unchecked(idx)
            })
        } else {
            None
        }
    }
}

// pub trait Index<Idx>where
//     Idx: ?Sized,{
//     type Output: ?Sized;

//     // Required method
//     fn index(&self, index: Idx) -> &Self::Output;
// }

impl<T> Index<usize> for Vec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}
