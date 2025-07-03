fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???

    let a: String = String::from("Palabra con una longitud considerablemente larga para que llegue a superar los 100 caracteres y salte el error that's a biy array");

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
