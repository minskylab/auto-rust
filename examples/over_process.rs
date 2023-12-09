use auto_rust::auto_generate;

fn main() {
    let a = auto_generate!("a random number between 1 and 10");

    print_type_of(&a);
    println!("{}", a);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
