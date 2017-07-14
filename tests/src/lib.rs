/*
MIT License

Copyright (c) 2017 David DeSimone

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
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

#![feature(proc_macro)]
#[macro_use]
extern crate mock_derive;

use mock_derive::mock;

struct Foo {
    x: i32,
    y: i32,
}

impl Foo {
    pub fn new() -> Foo {
        Foo { x: 32, y: 32 }
    }
}

trait HelloWorld {
    fn hello_world(&self);
    fn foo(&self) -> u32;
}

#[mock]
impl HelloWorld for Foo {
    fn hello_world(&self) {
        println!("Hello World!");
    }

    fn foo(&self) -> u32 {
        1
    }
}

/* Example of API
   let mut mock = MockHelloWorld::new();
   let method = mock.method_bar()
       .first_call()
       .set_result((Ok(13)))
       .second_call()
       .set_result((None));
   mock.set_bar(method);
   mock.bar(); // Returns Ok(13)
   mock.bar(); // Returns None

   // Will fall back to Foo's implementation
   // if method is not mocked
   let foo = Foo::new(...);
   let mut mock = MockHelloWorld::new();
   mock.set_fallback(foo); 

   let method = mock.method_hello_world()
       .when(|| true) 
       .set_result((20));
   mock.set_hello_world(method); 
   mock.hello_world(); // Returns 20
   mock.other_method(); // Calls foo's version of other_method
 
*/
#[test]
fn it_works() {
    let foo = Foo::new();
    let mut mock = MockHelloWorld::new();
    mock.set_fallback(foo);
    let method = mock.method_hello_world()
        .first_call()
        .when(|| true)
        .set_result(());

    mock.set_hello_world(method);
    mock.hello_world();

    let foo_method = mock.method_foo()
        .first_call()
        .set_result((3));

    mock.set_foo(foo_method);
    mock.foo();
}
