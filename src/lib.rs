#![no_std]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/67743099?v=4")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/67743099?v=4")]


// Enable experimental features for documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]

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

//! Nifty Simple Ring Buffer (aka circular buffer) is a [`no_std`](https://docs.rust-embedded.org/book/intro/no-std.html) crate that provides the [`ring!`] and [`manx!`] macros to easily create
//! circular buffer data structure on the stack.
//! 
//! ### Example 
//! ```
//! // Import crate with #[macro_use] 
//! ##[macro_use] extern crate nsrb;
//! 
//! // Use ring! macro to create circular buffer structure.
//! nsrb::ring!(pub(crate) ExampleRB[usize; 10]); 
//! 
//! // You can implement and access buffer inner variables if needed.
//! impl ExampleRB {
//!     pub fn head(&self) -> usize {
//!         self.head
//!     }
//! }
//! 
//! fn main() {
//!     // Use struct in code.
//!     let mut rb = ExampleRB::new();
//!     rb.push(5);
//!     assert_eq!(*rb.pop().unwrap(), 5);
//!     assert_eq!(rb.head(), 1);   // Using newly implemented method.
//! }
//! ``````

/// Smallest size a ring buffer can be. Default : 2.
/// 
/// Can be removed via the `no_limit` feature.
pub const NSRB_LOWER_LIMIT : usize = 2;

/// Largest size a ring buffer can be. Default : [u16::MAX].
/// 
/// Can be removed via the `no_limit` feature.
pub const NSRB_UPPER_LIMIT : usize = u16::MAX as usize;

#[doc(hidden)]
pub mod ring;

#[doc(hidden)]
mod manx;

/*
//! You can also create [optimized](https://en.wikipedia.org/wiki/Circular_buffer#Optimization) 
//! [unchecked](https://doc.rust-lang.org/beta/book/ch03-02-data-types.html#integer-overflow) [u8] / [u16] [`ring!`] 
//! and [`manx!`] buffer if performance is needed by using the `@unchecked(u8)` and `@unchecked(u16)` 
//! modifiers. Refer to each macro documentation for more information.
*/


/*
#[macro_export]
macro_rules! ring {
    ($name : ident, $type : ty, $size : expr) => {
        $crate::ring_core!(,$name, $type,$size,);
    };
    ($name : ident, $type : ty, $size : expr, $header : meta) => {
        $crate::ring_core!(,$name, $type,$size,$header);
    };
    ($visibility : vis, $name : ident, $type : ty, $size : expr) => {
        $crate::ring_core!($visibility,$name, $type,$size,);
    };
    ($visibility : vis, $name : ident, $type : ty, $size : expr, $header : meta) => {
        $crate::ring_core!($visibility,$name, $type,$size,$header);
    };
    
    (@unchecked(u8) $name : ident, $type : ty) =>{
        $crate::ring_core!(@unchecked(u8) ,$name, $type);
    };
    (@unchecked(u8) $name : ident, $type : ty, $header : meta) =>{
        $crate::ring_core!(@unchecked(u8) ,$name, $type);
    };
    (@unchecked(u8) $visibility : vis, $name : ident, $type : ty) =>{
        $crate::ring_core!(@unchecked(u8) ,$name, $type);
    };
    (@unchecked(u8) $visibility : vis, $name : ident, $type : ty, $header : meta) =>{
        $crate::ring_core!(@unchecked(u8) ,$name, $type);
    };
    
    (@unchecked(u16) $name : ident, $type : ty) =>{
        $crate::ring_core!(@unchecked(u16) ,$name, $type);
    };
    (@unchecked(u16) $name : ident, $type : ty, $header : meta) =>{
        $crate::ring_core!(@unchecked(u16) ,$name, $type);
    };
    (@unchecked(u16) $visibility : vis, $name : ident, $type : ty) =>{
        $crate::ring_core!(@unchecked(u16) ,$name, $type);
    };
    (@unchecked(u16) $visibility : vis, $name : ident, $type : ty, $header : meta) =>{
        $crate::ring_core!(@unchecked(u16) ,$name, $type);
    };
}

//ring!(pub, Toto, usize, 10, derive(Debug));

/// Create a [manx](https://www.approxion.com/circular-adventures-ix-the-poor-ring-buffer-that-had-no-tail/) 
/// buffer. See [`manx syntax`](macro.manx.html#syntax) for usage.
#[macro_export]
macro_rules! manx {
    ($name : ident, $type : ty, $size : expr) => {
        $crate::manx_core!(,$name, $type,$size);
    };
    ($visibility : vis, $name : ident, $type : ty, $size : expr) => {
        $crate::manx_core!($visibility,$name, $type,$size);
    };
    (@unchecked(u8) $name : ident, $type : ty) =>{
        $crate::manx_core!(@unchecked(u8) ,$name, $type);
    };
    (@unchecked(u16) $name : ident, $type : ty) =>{
        $crate::manx_core!(@unchecked(u16) ,$name, $type);
    };
}

ring2!(Toto[usize; 10]);
ring2!(pub(crate) Toto2[usize; 10]);
ring2!(#[derive(Debug)] Toto3[usize; 10]);
ring2!(pub(crate) Toto4[usize; 10]);
ring2!{ #[doc="Foo"] 
        #[derive(Debug,Copy,Clone)] 
        pub Toto5[usize; 10] }

ring2!(@unchecked(u8) Toto7[usize]);

ring2!{ @unchecked(u8) #[doc="Foo"] 
        #[derive(Debug,Copy,Clone)] 
        pub Toto8[usize] }

#[macro_export]
macro_rules! ring2 {
    /*
    ($name : ident, $type : ty, $size : expr) => {
        $crate::ring_core!(,$name, $type,$size,);
    };
    ($name : ident, $type : ty, $size : expr, $header : meta) => {
        $crate::ring_core!(,$name, $type,$size,$header);
    };
    ($visibility : vis, $name : ident, $type : ty, $size : expr) => {
        $crate::ring_core!($visibility,$name, $type,$size,);
    };
    */
    ($(#[$attr:meta])* $visibility : vis $name : ident[$type : ty; $size : expr]) => {
        //$crate::ring_core!($visibility,$name, $type,$size,$header);
        $(
            #[$attr]
        )*
        #[allow(dead_code)]
        $visibility struct $name { tail : usize, head : usize, buffer : [$type; $size], }
    };

    (@unchecked(u8) $(#[$attr:meta])* $visibility : vis $name : ident[$type : ty]) => {
        //$crate::ring_core!($visibility,$name, $type,$size,$header);
        $(
            #[$attr]
        )*
        #[allow(dead_code)]
        $visibility struct $name { tail : usize, head : usize, buffer : [$type; u8::MAX as usize], }
    };
    
    
    (@unchecked(u8) $name : ident, $type : ty) =>{
        $crate::ring_core!(@unchecked(u8) ,$name, $type);
    };
    (@unchecked(u8) $name : ident, $type : ty, $header : meta) =>{
        $crate::ring_core!(@unchecked(u8) ,$name, $type);
    };
    (@unchecked(u8) $visibility : vis, $name : ident, $type : ty) =>{
        $crate::ring_core!(@unchecked(u8) ,$name, $type);
    };
    (@unchecked(u8) $visibility : vis, $name : ident, $type : ty, $header : meta) =>{
        $crate::ring_core!(@unchecked(u8) ,$name, $type);
    };
    
    (@unchecked(u16) $name : ident, $type : ty) =>{
        $crate::ring_core!(@unchecked(u16) ,$name, $type);
    };
    (@unchecked(u16) $name : ident, $type : ty, $header : meta) =>{
        $crate::ring_core!(@unchecked(u16) ,$name, $type);
    };
    (@unchecked(u16) $visibility : vis, $name : ident, $type : ty) =>{
        $crate::ring_core!(@unchecked(u16) ,$name, $type);
    };
    (@unchecked(u16) $visibility : vis, $name : ident, $type : ty, $header : meta) =>{
        $crate::ring_core!(@unchecked(u16) ,$name, $type);
    };
    
}
*/