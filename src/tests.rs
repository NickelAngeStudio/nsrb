/* 
Copyright (c) 2024  NickelAnge.Studio 
Email               mathieu.grenier@nickelange.studio
Git                 https://github.com/NickelAngeStudio/nsfcb

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


super::fcb_generic!(FixedCRB, 15);
super::fcb_generic!(pub(crate), FixedCRB2, 15);
    super::ring!(FixedDouble, f64, 10);
    super::ring!(@unchecked(u8) FixedDouble2, f64);
    super::manx_generic!(GenManx, 15);
    super::manx!(Manx, u8, 50);
 
    pub struct Blabla {
        
    }

    #[test]
    fn test_add() {
        let mut a : FixedCRB<usize> = FixedCRB::new();
        let aa : FixedCRB2<usize> = FixedCRB2::new();
        let b  = FixedDouble::new();
        let c : GenManx<u32> = GenManx::new();
        let d = Manx::new();

        let bb = 55;
        a.push(bb);

        println!("bb={}", bb);

        
    }

/*
#[cfg(test)]
mod tests {

    



}
*/