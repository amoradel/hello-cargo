fn main() {
    println!("Hello, world!");
    let name: &str = "Anibal"; //Inmutable (como una variable static)
    let mut age: i32 = 30; //mMtable (que se puede cambiar el valor)
    print!("Hola {}, Tu edad es {}, feliciades\n\n", name, age);
    age = 32;
    print!("Hola {}, Tu edad es {}, feliciades\n\n", name, age);

    // todo!("Este es un todo por hacer");
}
