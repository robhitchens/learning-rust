fn main() {
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!(
        "{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over"
    );

    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    println!("{number:>width$}", number=1, width=6);

    println!("{number:>0width$}", number=1, width=6);

    //This will cause rust to check the arguments
    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);

    println!("This struct `{:?}` won't print..., unless you add Debug to the struct", Structure(3));

    #[allow(dead_code)]
    struct AltStructure(i32);

    impl std::fmt::Display for AltStructure {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
            write!(f, "(value for AltStructure: {})", self.0)
        }
    }

    println!("Alternative way of handling printing of structures {}", AltStructure(64));
}