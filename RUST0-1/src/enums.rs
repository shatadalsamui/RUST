// Enums are used to define custom types with a fixed set of possible values.
// values are enforced and explicitly mentioned here we cannot use anyhting else.
enum Direction { 
    North,
    East,
    South,
    West,
}


fn main(){
    let my_direction = Direction::North ; 
    let new_direction  = my_direction ; 
    move_around(new_direction);
}


fn move_around(direction:Direction){
    // implement logic to move a character around 
}