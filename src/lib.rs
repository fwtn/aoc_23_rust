use std::fs;

pub fn get_file_contents(path: &str) -> Vec<String> {
    let mut retval: Vec<String> = Vec::new();
    let contents = fs::read_to_string(path);
    let contents = match contents{
        Ok(x) => x,
        Err(e) => panic!("{0}", e)
    };
    
    contents.split('\n')
        .into_iter()
        .for_each(|line| {retval.push(String::from(line))});
    
    retval
}
