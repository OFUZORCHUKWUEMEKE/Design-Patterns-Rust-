// Generics Examples


fn return_thing<T>(thing:T)->T{
    println!("Here is a Thing");
}

fn caller(){
    let my_string = return_thing(String::from("I am a string"));
    return my_string
}