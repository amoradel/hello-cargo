fn main() {
    println!("Hello, world!");

    //Declaracion de variables
    let name = "Anibal"; //Inmutable (como una variable static)
    let mut age = 30; //mMtable (que se puede cambiar el valor)
    print!("Hola {}, Tu edad es {}, feliciades\n\n", name, age);
    age = 32;
    print!("Hola {}, Tu edad es {}, feliciades\n\n", name, age);

    //Propiedad reemplazada de variables
    // Declarar el enlace de la primera variable con el nombre "shadow_num"
    let shadow_num = 5; //5

    // Declara el enlace de la segunda variable, oscurece la variable existente "shadow_num"
    let shadow_num = shadow_num + 5; //10

    // Declarar el enlace de la tercera variable, oscurece el segundo enlace de la variable "shadow_num"
    let shadow_num = shadow_num * 2; //20

    println!("The number is {}.", shadow_num);
}
