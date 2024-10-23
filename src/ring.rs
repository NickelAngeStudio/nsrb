/* 
Copyright (c) 2024  NickelAnge.Studio 
Email               mathieu.grenier@nickelange.studio
Git                 https://github.com/NickelAngeStudio/nsrb

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

/// Create a ring buffer (aka circular buffer) data structure.
/// 
/// 
/// ## Checked
/// 
/// Checked ring buffer need a size specified and execute extra instructions to prevent [integer overflow](https://doc.rust-lang.org/beta/book/ch03-02-data-types.html#integer-overflow).

/// ##### `$(#[$attr:meta])*`
/// Extra [attributes](https://doc.rust-lang.org/reference/attributes.html) for the ring buffer. *`Optional`*
/// 
/// ##### `$visibility`
/// Specify the [visibility](https://doc.rust-lang.org/reference/visibility-and-privacy.html) of the ring buffer struct. Private if not specified. *`Optional`*
/// 
/// ##### `$name`
/// Name of the circular buffer struct without `"`.
/// 
/// ##### `$type`
/// Type contained in the buffer. Must implement [Clone], [Copy], [Default] traits and must be [Sized] since it's created on the stack.
/// 
/// ##### `$size`
/// Count of element in the buffer. Limit is between [NSRB_LOWER_LIMIT](super::NSRB_LOWER_LIMIT) and [NSRB_UPPER_LIMIT](super::NSRB_UPPER_LIMIT) unless the `no_limit` feature is specified.
/// 
/// #### Example
/// ```
/// // Important to import crate with #[macro_use] 
/// #[macro_use] extern crate nsrb;
/// 
/// #[derive(Clone, Copy, Debug)]
/// pub struct LogEntry {
///     pub time_date : usize,
///     pub entry : [char;256]
/// }
/// 
/// impl Default for LogEntry {
///     fn default() -> Self { LogEntry { time_date : 0, entry : [' ';256] } }
///  }
/// 
/// nsrb::ring!(#[derive(Debug)] pub(crate) LogChecked[LogEntry; 10]);
/// 
/// fn main() {
///     let log = LogChecked::new();
/// }
/// 
/// ```
/// ## Unchecked
/// Unchecked ring buffer use [integer overflow](https://doc.rust-lang.org/beta/book/ch03-02-data-types.html#integer-overflow) to wrap head and tail thus need less intructions.
/// 
/// 
/// ##### `$int`
/// Int type. Either [u8] or [u16]. Buffer will have a size of [u8::MAX] or [u16::MAX]. `no_limit` feature must be used for [u32]+.
/// ##### `$(#[$attr:meta])*`
/// Extra [attributes](https://doc.rust-lang.org/reference/attributes.html) for the ring buffer. *`Optional`*
/// 
/// ##### `$visibility`
///  Specify the [visibility](https://doc.rust-lang.org/reference/visibility-and-privacy.html) of the ring buffer struct. Private if not specified. *`Optional`* 
/// ##### `$name`
/// Name of the circular buffer struct without `"`.
/// ##### `$type`
/// Type contained in the buffer. Must implement [Copy] and [Default] traits.
/// 
/// #### Example
/// ```
/// // Important to import crate with #[macro_use] 
/// #[macro_use] extern crate nsrb;
/// 
/// #[derive(Clone, Copy, Debug)]
/// pub struct LogEntry {
///     pub time_date : usize,
///     pub entry : [char;256]
/// }
/// 
/// impl Default for LogEntry {
///     fn default() -> Self { LogEntry { time_date : 0, entry : [' ';256] } }
///  }
/// 
/// nsrb::ring!(@unchecked(u8) #[derive(Debug)] LogUnchecked[LogEntry]);
/// 
/// fn main() {
///     let log = LogUnchecked::new();
/// }
/// 
/// ```
/// 
/// ## Implementation
/// Each ring buffer provides those method by default.
/// 
/// #### `$name::new()`
/// Create a new instance of `$name` fixed circular buffer.
/// 
/// #### `$name::push(item : $type)`
/// Push an item into `$name` circular buffer.
/// 
/// #### `$name::pop() -> Option<&$type>`
/// Returns Some(&`$type`) if buffer contains an element.
/// 
/// ## Extra
/// Extra implementation that can be added if needed.
/// 
/// ```
/// #[macro_use] extern crate nsrb;
/// nsrb::ring!(RingBuffer[u8; 10]);
/// 
/// impl RingBuffer {
///     /// Clear all element from buffer
///     pub fn clear(&mut self) {
///         self.tail = self.head;
///     }
/// 
///     /// Returns the size of element in ring buffer
///     pub fn len(&self) -> usize {
///         if self.tail > self.head {
///             self.buffer.len() + self.head - self.tail
///         } else {
///             self.head - self.tail
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! ring {
    ($(#[$attr:meta])* $visibility : vis $name : ident[$type : ty; $size : expr]) => {
        $(
            #[$attr]
        )*
        #[allow(dead_code)]
        $visibility struct $name { tail : usize, head : usize, buffer : [$type; $size], }

        #[allow(dead_code)]
        impl $name {
            pub fn new() -> $name {             
            
                #[cfg(not(feature = "no_limit"))]
                assert!($size as usize >= $crate::NSRB_LOWER_LIMIT);

                #[cfg(not(feature = "no_limit"))]
                assert!($size as usize <= $crate::NSRB_UPPER_LIMIT);

                $name {
                    tail: 0,
                    head: 0,
                    buffer: [<$type>::default(); $size],
                }
            }

            #[inline(always)]
            pub fn push(&mut self, item : $type) {
                self.buffer[self.head] = item;
                self.push_head();
            }

            #[inline(always)]
            pub fn pop(&mut self) -> Option<&$type> {
                
                if self.tail != self.head {
                    let tail = self.tail;
                    self.push_tail();
                    Some(&self.buffer[tail])
                } else {
                    None
                }
            }

            #[inline(always)]
            fn push_head(&mut self) {

                if self.head >= $size - 1 {
                    self.head = 0;
                } else {
                    self.head += 1;
                }

                if self.head == self.tail {
                    self.push_tail();
                }

            }

            #[inline(always)]
            fn push_tail(&mut self) {
                if self.tail >= $size - 1 {
                    self.tail = 0;
                } else {
                    self.tail += 1;
                }
            }
        }   
    };
    (@unchecked($int:ty) $(#[$attr:meta])* $visibility : vis $name : ident[$type : ty]) => {
        $(
            #[$attr]
        )*
        #[allow(dead_code)]
        $visibility struct $name {
            tail : $int,
            head : $int,
            buffer : [$type; <$int>::MAX as usize + 1],
        }

        #[allow(dead_code)]
        impl $name {
            pub fn new() -> $name {

                #[cfg(not(feature = "no_limit"))]
                assert!(<$int>::MAX as usize <= $crate::NSRB_UPPER_LIMIT);

                $name {
                    tail: 0,
                    head: 0,
                    buffer: [<$type>::default(); <$int>::MAX as usize + 1],
                }
            }

            #[inline(always)]
            pub fn push(&mut self, item : $type) {
                self.buffer[self.head as usize] = item;
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
        }   
    };

}

#[cfg(test)]
#[cfg(not(feature = "no_limit"))]   // Only limit features are tested
pub(crate) mod tests_checked {

    // Test the lower limit of ring buffer
    ring!(TooSmall[usize;super::super::NSRB_LOWER_LIMIT - 1]);
    #[test]
    #[should_panic]
    fn ring_lower_limit() {
        let _ = TooSmall::new();
    }

    // Test the upper limit of ring buffer
    ring!(TooBig[usize;super::super::NSRB_UPPER_LIMIT + 1]);
    #[test]
    #[should_panic]
    fn ring_upper_limit() {
        let _ = TooBig::new();
    }

    // Test push and pop of ring buffer
    ring!(RbPP[usize;10]);
    #[test]
    fn ring_push_pop() {
        let mut rb = RbPP::new();

        for i in 0..15 {
            rb.push(i);
        }

        for i in 6..15 {
            assert_eq!(*rb.pop().unwrap(), i);
        }

        assert!(rb.pop().is_none());
    }

    // Test extra clear and len implementation
    ring!(RbExtra[usize;50]);

    impl RbExtra {
        /// Clear all element from buffer
        pub fn clear(&mut self) {
            self.tail = self.head;
        }
     
        /// Returns the size of element in ring buffer
        pub fn len(&self) -> usize {
            if self.tail > self.head {
                self.buffer.len() + self.head - self.tail
            } else {
                self.head - self.tail
            }
        }
    }

    #[test]
    fn ring_extra_impl() {
        let mut rb = RbExtra::new();

        assert!(rb.len() == 0);

        for i in 0..15 {
            rb.push(i);
        }

        assert_eq!(rb.len(),  15);

        rb.clear();

        assert!(rb.len() == 0);

        while rb.tail <= rb.head {
            rb.push(0);
        }

        assert_eq!(rb.len(),  35);

        rb.clear();

        assert!(rb.len() == 0);

        // Testing len() more intensively
        let mut rb = RbExtra::new();

        for i in 0..255 {
            if i < rb.buffer.len() {
                assert_eq!(rb.len(),  i);
            } else {
                assert_eq!(rb.len(),  49);
            }

            rb.push(i);
        }

        rb.pop();

        assert_eq!(rb.len(),  48);

    }
   

}


#[cfg(test)]
#[cfg(not(feature = "no_limit"))]   // Only limit features are tested
pub(crate) mod tests_unchecked {

    // Test the upper limit of ring buffer
    ring!(@unchecked(u32) TooBig[usize]);
    #[test]
    #[should_panic]
    fn ring_upper_limit() {
        let _ = TooBig::new();
    }

    // Test push and pop of ring buffer
    ring!(@unchecked(u8) RbPP[usize]);
    #[test]
    fn ring_push_pop() {
        let mut rb = RbPP::new();

        for i in 0..u8::MAX as usize {
            rb.push(i);
        }

        for i in 0..u8::MAX as usize {
            assert_eq!(*rb.pop().unwrap(), i);
        }

        assert!(rb.pop().is_none());
    }

    // Test extra clear and len implementation
    ring!(@unchecked(u8) RbExtra[usize]);

    impl RbExtra {
        /// Clear all element from buffer
        pub fn clear(&mut self) {
            self.tail = self.head;
        }
     
        /// Returns the size of element in ring buffer
        pub fn len(&self) -> usize {
            if self.tail > self.head {
                self.buffer.len() as usize + self.head as usize - self.tail as usize
            } else {
                self.head as usize - self.tail as usize
            }
        }
    }

    #[test]
    fn ring_extra_impl() {
        let mut rb = RbExtra::new();

        assert!(rb.len() == 0);

        for i in 0..15 {
            rb.push(i);
        }

        assert_eq!(rb.len(),  15);

        rb.clear();

        assert!(rb.len() == 0);

        while rb.tail <= rb.head {
            rb.push(0);
        }

        assert_eq!(rb.len(),  241);

        rb.clear();

        assert!(rb.len() == 0);

        // Testing len() more intensively
        let mut rb = RbExtra::new();

        for i in 0..255 {
            if i < rb.buffer.len() {
                assert_eq!(rb.len(),  i);
            } else {
                assert_eq!(rb.len(),  49);
            }

            rb.push(i);
        }

        rb.pop();

        assert_eq!(rb.len(),  254);

    }
   

}
