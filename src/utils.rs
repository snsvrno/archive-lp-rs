pub fn get_second_extension<'a>(file : &'a str) -> Option<&'a str> {
    //! gets the second extenison, so aa.b.c then it returns b
    
    let splits = file.split(".").collect::<Vec<&str>>();

    if splits.len() >= 3 {
        Some(splits[splits.len()-2])
    } else {
        None
    }
}