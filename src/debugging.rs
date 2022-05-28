pub fn print_debug(toprint: &str){
    if let Some(_) = option_env!("debug") {
        println!("{}", toprint);
    }
}

pub fn debug_on() -> bool{
    if let Some(_) = option_env!("debug") {
        return true;
    }
    false
}