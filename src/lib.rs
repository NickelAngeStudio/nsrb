// Enable experimental features for documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]

/* 
Copyright (c) 2024  NickelAnge.Studio 
Email               mathieu.grenier@nickelange.studio
Git                 https://github.com/NickelAngeStudio/nsring

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFcircularEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

//! Nifty Simple Ring Buffer (aka circular buffer) provides macros to rapidly create
//! circular buffer data structure on the stack.
//! 
//! It also provides ([Manx buffer](https://www.approxion.com/circular-adventures-ix-the-poor-ring-buffer-that-had-no-tail/)),  
//! a no tail buffer for rapid data accumulation.
//! 
//! 
//! # Reference(s)
//! - [Manx buffer](https://www.approxion.com/circular-adventures-ix-the-poor-ring-buffer-that-had-no-tail/)
//! 
//!
//! 
//! 
//! ### Usage 
//! ```
//! ##[macro_use] extern crate nsring;
//! 
//! nsring::ring!(UsizeRB, usize, 10); // Use macro to circular buffer structure.
//! 
//! fn main() {
//!     let mut rb = UsizeRB::new();
//!     rb.push(5);
//!     assert_eq!(*rb.pop().unwrap(), 5);
//! }
//! ``````
//! 
//! 


/// nsring Unit tests
#[cfg(test)]
pub(crate) mod tests {
    include!("tests.rs");
}

/// Create a fixed ring buffer for specific type.
/// 
/// ## Usage
/// 
/// ## Implementations
/// #### `$name::new()`
/// Create a new instance of `$name` fixed circular buffer.
/// 
/// #### `$name::push(element : $type)`
/// Push an element into `$name` circular buffer.
/// 
/// #### `$name::pop() -> Option<&$type>`
/// Returns Some(&$type) if buffer contains a new element.
/// 
/// #### `$name::clear()`
/// Clear the buffer (doesn't clear elements values)
#[macro_export]
macro_rules! ring {
    ($name : ident, $type : ty, $size : expr) => {
        $crate::ring_core!(,$name, $type,$size);
    };
    ($visibility : vis, $name : ident, $type : ty, $size : expr) => {
        $crate::ring_core!($visibility,$name, $type,$size);
    };
    (@unchecked(u8) $name : ident, $type : ty) =>{
        $crate::ring_core!(@unchecked(u8) ,$name, $type);
    };
    (@unchecked(u16) $name : ident, $type : ty) =>{
        $crate::ring_core!(@unchecked(u16) ,$name, $type);
    };
}

/// Core code of ring! macro.
/// Not meant to be called directly hence hidden from documentation.
#[doc(hidden)]
#[macro_export]
macro_rules! ring_core {
    ($visibility : vis, $name : ident, $type : ty, $size : expr) => {
        #[allow(dead_code)]
        $visibility struct $name { tail : usize, head : usize, buffer : [$type; $size], }

        #[allow(dead_code)]
        impl $name {
            pub fn new() -> $name {
                $name {
                    tail: 0,
                    head: 0,
                    buffer: [<$type>::default(); $size],
                }
            }

            /// Push an element into the fixed circular buffer.
            #[inline(always)]
            pub fn push(&mut self, element : $type) {
                self.buffer[self.head] = element;
                self.push_head();
            }

            /// Get refence to oldest unpop [Log] entry.
            /// 
            /// Returns Some([LogEntry]) is there are new [LogEntry], None if no entry.
            #[inline(always)]
            pub fn pop(&mut self) -> Option<&$type> {
                
                if self.tail != self.head {
                    let tail = self.tail;   // Keep tail in memory before pushing
                    self.push_tail();
                    Some(&self.buffer[tail])
                } else {    // No entries in the circular buffer.
                    None
                }
            }

            /// Push the head of the circular buffer. Head will push the tail if it ends
            /// up being equal.
            #[inline(always)]
            fn push_head(&mut self) {

                if self.head >= $size - 1 {
                    self.head = 0;  // Put head to the back of the buffer.
                } else {
                    self.head += 1; // Increment head.
                }

                // Push tail if equal
                if self.head == self.tail {
                    self.push_tail();
                }

            }

            /// Push the tail forward in the circular buffer.
            /// Tail will NEVER push the head.
            #[inline(always)]
            fn push_tail(&mut self) {
                if self.tail >= $size - 1 {
                    self.tail = 0;  // Put tail to the back of the buffer.
                } else {
                    self.tail += 1; // Increment tail.
                }
            }

            

            /// Clear the fixed circular buffer.
            pub fn clear(&mut self) {
                self.tail = self.head;
            }

        }   
    };
    (@unchecked($size_type : ty) $visibility : vis, $name : ident, $type : ty) =>{
        #[allow(dead_code)]
        $visibility struct $name {
            tail : u8,
            head : u8,
            buffer : [$type; <$size_type>::MAX as usize],
        }

        #[allow(dead_code)]
        impl $name {
            pub fn new() -> $name {
                $name {
                    tail: 0,
                    head: 0,
                    buffer: [<$type>::default(); <$size_type>::MAX as usize],
                }
            }

            #[inline(always)]
            pub fn push(&mut self, element : $type) {
                self.buffer[self.head as usize] = element;
                self.head += 1;
                if self.head == self.tail {
                    self.tail += 1;
                }
            }

            #[inline(always)]
            pub fn pop(&mut self) -> Option<&$type> {
                if self.tail != self.head {
                    let tail = self.tail;
                    self.tail += 1;
                    Some(&self.buffer[tail as usize])
                } else {
                    None
                }
            }
            
            pub fn clear(&mut self) {
                self.tail = self.head;
            }
        }   
    };
}


/// Create a [generic](https://doc.rust-lang.org/book/ch10-01-syntax.html) circular buffer reusable with multiple types.
#[macro_export]
macro_rules! fcb_generic {
    ($name : ident, $size : expr) => {
        crate::fcb_generic!(,$name, $size);
    };
    ($visibility : vis, $name : ident, $size : expr) => {
        #[allow(dead_code)]
        $visibility struct $name<T> {
            tail : usize,
            head : usize,
            buffer : [T; $size],
        }

        #[allow(dead_code)]
        impl<T : Copy + Default> $name<T> {
            pub fn new() -> $name<T> {
                $name {
                    tail: 0,
                    head: 0,
                    buffer: [T::default(); $size],
                }
            }

            /// Push an element into the fixed circular buffer.
            #[inline(always)]
            pub fn push(&mut self, element : T) {
                self.buffer[self.head] = element;
                self.push_head();
            }

            /// Get refence to oldest unpop [Log] entry.
            /// 
            /// Returns Some([LogEntry]) is there are new [LogEntry], None if no entry.
            #[inline(always)]
            pub fn pop(&mut self) -> Option<&T> {
                
                if self.tail != self.head {
                    let tail = self.tail;   // Keep tail in memory before pushing
                    self.push_tail();
                    Some(&self.buffer[tail])
                } else {    // No entries in the circular buffer.
                    None
                }
            }

            /// Push the head of the circular buffer. Head will push the tail if it ends
            /// up being equal.
            #[inline(always)]
            fn push_head(&mut self) {

                if self.head >= $size - 1 {
                    self.head = 0;  // Put head to the back of the buffer.
                } else {
                    self.head += 1; // Increment head.
                }

                // Push tail if equal
                if self.head == self.tail {
                    self.push_tail();
                }

            }

            /// Push the tail forward in the circular buffer.
            /// Tail will NEVER push the head.
            #[inline(always)]
            fn push_tail(&mut self) {
                if self.tail >= $size - 1 {
                    self.tail = 0;  // Put tail to the back of the buffer.
                } else {
                    self.tail += 1; // Increment tail.
                }
            }

            

            /// Clear the fixed circular buffer.
            pub fn clear(&mut self) {
                self.tail = self.head;
            }

        }   
    }

}

/// Create a [manx](https://www.approxion.com/circular-adventures-ix-the-poor-ring-buffer-that-had-no-tail/) 
/// circular buffer for a single type.
#[macro_export]
macro_rules! manx {
    ($name : ident, $type : ty, $size : expr) => {
        #[allow(dead_code)]
        pub struct $name {
            pub index : usize,
            buffer : [$type; $size],
        }

        #[allow(dead_code)]
        impl $name {
            pub fn new() -> $name {
                $name {
                    index: 0,
                    buffer: [<$type>::default(); $size],
                }
            }

            /// Push an element into the fixed circular buffer.
            #[inline(always)]
            pub fn push(&mut self, element : $type) {
                self.buffer[self.index] = element;
                if self.index >= $size - 1 {
                    self.index = 0;
                } else {
                    self.index += 1;
                }
            }

            #[inline(always)]
            fn buffer(&self) -> &[$type; $size] {
                &self.buffer
            }

            #[inline(always)]
            fn buffer_mut(&self) -> &[$type; $size] {
                &self.buffer
            }



            /// Push the head of the circular buffer. Head will push the tail if it ends
            /// up being equal.
            #[inline(always)]
            fn push_index(&mut self) {
                
            }

            

        }   
    }

}


/// Create a [generic](https://doc.rust-lang.org/book/ch10-01-syntax.html) [manx](https://www.approxion.com/circular-adventures-ix-the-poor-ring-buffer-that-had-no-tail/) 
/// circular buffer reusable with multiple types.
#[macro_export]
macro_rules! manx_generic {
    ($name : ident, $size : expr) => {
        #[allow(dead_code)]
        pub struct $name<T> {
            pub index : usize,
            buffer : [T; $size],
        }

        #[allow(dead_code)]
        impl<T : Copy + Default> $name<T> {
            pub fn new() -> $name<T> {
                $name {
                    index: 0,
                    buffer: [T::default(); $size],
                }
            }

            #[inline(always)]
            pub fn push(&mut self, element : T) {
                self.buffer[self.index] = element;
                if self.index >= $size - 1 { self.index = 0; } else { self.index += 1; }
            }

            #[inline(always)]
            fn buffer(&self) -> &[T; $size] {
                &self.buffer
            }

            #[inline(always)]
            fn buffer_mut(&self) -> &[T; $size] {
                &self.buffer
            }
        }   
    }

}