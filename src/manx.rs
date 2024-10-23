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


/// Create a no tail [Manx buffer](https://www.approxion.com/circular-adventures-ix-the-poor-ring-buffer-that-had-no-tail/) data structure.
/// 
/// 
/// ## Checked
/// 
/// Checked manx buffer need a size specified and execute extra instructions to prevent [integer overflow](https://doc.rust-lang.org/beta/book/ch03-02-data-types.html#integer-overflow).

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
/// nsrb::manx!(#[derive(Debug)] pub(crate) LogManxChecked[LogEntry; 10]);
/// 
/// fn main() {
///     let log = LogManxChecked::new();
/// }
/// 
/// ```
/// ## Unchecked
/// Unchecked manx buffer use [integer overflow](https://doc.rust-lang.org/beta/book/ch03-02-data-types.html#integer-overflow) to wrap head thus need less intructions.
/// 
/// 
/// ##### `$int`
/// Int type. Either [u8] or [u16]. Buffer will have a size of [u8::MAX] or [u16::MAX]. `no_limit` feature must be used for [u32]+.
/// ##### `$(#[$attr:meta])*`
/// Extra [attributes](https://doc.rust-lang.org/reference/attributes.html) for the manx buffer. *`Optional`*
/// 
/// ##### `$visibility`
///  Specify the [visibility](https://doc.rust-lang.org/reference/visibility-and-privacy.html) of the manx buffer struct. Private if not specified. *`Optional`* 
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
/// nsrb::manx!(@unchecked(u8) #[derive(Debug)] LogManxUnchecked[LogEntry]);
/// 
/// fn main() {
///     let log = LogManxUnchecked::new();
/// }
/// 
/// ```
/// 
/// ## Implementation
/// Each manx buffer provides those method by default.
/// 
/// #### `$name::new()`
/// Create a new instance of `$name` fixed manx buffer.
/// 
/// #### `$name::push(item : $type)`
/// Push an item into `$name` manx buffer.
/// 
/// #### `$name::items() -> &$type[]`
/// Returns a read only reference to the buffer.
#[macro_export]
macro_rules! manx {
    ($(#[$attr:meta])* $visibility : vis $name : ident[$type : ty; $size : expr]) => {
        $(
            #[$attr]
        )*
        #[allow(dead_code)]
        $visibility struct $name { head : usize, buffer : [$type; $size], }

        #[allow(dead_code)]
        impl $name {
            pub fn new() -> $name {             
            
                #[cfg(not(feature = "no_limit"))]
                assert!($size as usize >= $crate::NSRB_LOWER_LIMIT);

                #[cfg(not(feature = "no_limit"))]
                assert!($size as usize <= $crate::NSRB_UPPER_LIMIT);

                $name {
                    head: 0,
                    buffer: [<$type>::default(); $size],
                }
            }

            #[inline(always)]
            pub fn push(&mut self, item : $type) {
                self.buffer[self.head] = item;
                if self.head >= $size - 1 {
                    self.head = 0;
                } else {
                    self.head += 1;
                }
            }

            #[inline(always)]
            pub fn items(&self) -> &[$type; $size] {
                &self.buffer
            }

           
        }   
    };
    (@unchecked($int:ty) $(#[$attr:meta])* $visibility : vis $name : ident[$type : ty]) => {
        $(
            #[$attr]
        )*
        #[allow(dead_code)]
        $visibility struct $name {
            head : $int,
            buffer : [$type; <$int>::MAX as usize + 1],
        }

        #[allow(dead_code)]
        impl $name {
            pub fn new() -> $name {

                #[cfg(not(feature = "no_limit"))]
                assert!(<$int>::MAX as usize <= $crate::NSRB_UPPER_LIMIT);

                $name {
                    head: 0,
                    buffer: [<$type>::default(); <$int>::MAX as usize + 1],
                }
            }

            #[inline(always)]
            pub fn push(&mut self, item : $type) {
                self.buffer[self.head as usize] = item;
                self.head += 1;
            }

            #[inline(always)]
            pub fn items(&self) -> &[$type; <$int>::MAX as usize + 1] {
                &self.buffer
            }
        }   
    };

}

#[cfg(test)]
#[cfg(not(feature = "no_limit"))]   // Only limit features are tested
pub(crate) mod tests_checked {

    // Test the lower limit of ring buffer
    manx!(TooSmall[usize;super::super::NSRB_LOWER_LIMIT - 1]);
    #[test]
    #[should_panic]
    fn manx_lower_limit() {
        let _ = TooSmall::new();
    }

    // Test the upper limit of ring buffer
    manx!(TooBig[usize;super::super::NSRB_UPPER_LIMIT + 1]);
    #[test]
    #[should_panic]
    fn manx_upper_limit() {
        let _ = TooBig::new();
    }

    // Test push and items
    manx!(ManxPush[usize;10]);
    #[test]
    fn manx_push_items() {
        let mut rb = ManxPush::new();

        assert_eq!(rb.head, 0);

        for i in 1..15 {
            rb.push(i);
        }

        assert_eq!(rb.head, 4);

        for i in 0..rb.items().len() {
            assert_ne!(rb.items()[i],0);
        }

        assert_eq!(rb.head, 4);

    }

}


#[cfg(test)]
#[cfg(not(feature = "no_limit"))]   // Only limit features are tested
pub(crate) mod tests_unchecked {

    // Test the upper limit of ring buffer
    manx!(@unchecked(u32) TooBig[usize]);
    #[test]
    #[should_panic]
    fn manx_upper_limit() {
        let _ = TooBig::new();
    }

    // Test push and items
    manx!(@unchecked(u8) ManxPush[usize]);
    #[test]
    fn manx_push_items() {
        let mut rb = ManxPush::new();

        assert_eq!(rb.head, 0);

        for i in 1..333 {
            rb.push(i);
        }

        assert_eq!(rb.head, 76);

        for i in 0..rb.items().len() {
            assert_ne!(rb.items()[i],0);
        }

        assert_eq!(rb.head, 76);

    }

    // Test push and items
    manx!(@unchecked(u16) ManxPushU16[usize]);
    #[test]
    fn manx_push_items_u16() {
        let mut rb = ManxPushU16::new();

        assert_eq!(rb.head, 0);

        for i in 1..333 {
            rb.push(i);
        }

        assert_eq!(rb.head, 332);

    }

}