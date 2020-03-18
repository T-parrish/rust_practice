pub mod geo_structs {
    pub fn rectangles() {
        let rect1 = Rectangle {
            width: 20,
            height: 40,
        };
    
        println!(
            "The area of a rectangle with width: {} and height: {} is {}",
            rect1.width,
            rect1.height,
            rect1.get_area()
        );
    
        // Since we added the derive(Debug) trait on the rectangle struct,
        // we can now print the struct data like so
        println!("dat rectangle: {:?}", rect1);
    
        // Alternate formatting
        println!("dat rectangle: {:#?}", rect1);
    
        let rect2 = Rectangle {
            width: 50,
            height: 20,
        };
    
        println!(
            "Rectangle 2 can hold rectangle 1? {}",
            rect2.can_hold(&rect1)
        );
    
        // making a square with an associated function
        let square = Rectangle::square(50);
        println!("Made a square {:#?}", square)
    }

    // Calculates area of a rectangle using Structs
    // adding the derive debug trait allows for debug formatting on print
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // Impl blocks allow you to implement instance methods on a struct
    impl Rectangle {
        fn get_area(&self) -> u32 {
            self.height * self.width
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.get_area() > other.get_area()
        }
    }

    // Can have more than one impl block on each struct
    impl Rectangle {
        // Associated functions are called with '::' and are similar to static methods
        fn square(size: u32) -> Rectangle {
            Rectangle {
                height: size,
                width: size,
            }
        }
    }
}