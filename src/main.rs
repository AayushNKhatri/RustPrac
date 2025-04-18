use std:: {any,io, str};
use regex::Regex;
fn typecheck<T>(_: &T) -> &str{
    return any::type_name::<T>();
}

#[warn(unused_variables)]
fn regex(input: &str)->&str {
    let mut which_type =  "";  
    let int_contain = Regex::new(r"^\d+$").unwrap();
    let float_contain = Regex::new(r"^[+-]?(\d*\.\d+|\d+\.\d*)$").unwrap();  
    
    if int_contain.is_match(input)
    {
        which_type = "int";
        return which_type;
    }
    else if float_contain.is_match(input)  {
        which_type = "float";
        return which_type;
    }
    else {
        which_type = "string";
        return which_type;
    }
}

fn main(){
    let input = io::stdin();
    let mut number = String::new();
    println!("Enter the what you are: ");
    input.read_line(&mut number).expect("Plz Enter");
    let _type_of_variable = typecheck(&number);
    println!("{_type_of_variable}");
    let trimed_input = number.trim();
    let type_of_var:&str = regex(&trimed_input);
    println!("The type is {type_of_var}");    

}


//This codes are still here for futrure references 
//need's import to work std::vec::Vec and fmt 
//
//
//
//#[warn(dead_code)]
//struct MyVec(Vec<char>); //this tupe struct which contain Vec<char> easy to call


// This implimentaion will let vec be printed normally by converting each index to sting
//impl fmt::Display for MyVec{
//    fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result{
  //       let string_representation:String = self.0.iter().collect();
    //     write!(f, "{}", string_representation)
    // }
//} 
