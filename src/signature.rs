use std::fs;

pub fn get_signature_counter() -> String {
    let counter_name = "Medias/counter";
    let counter_raw = match fs::read_to_string(counter_name) {Ok(result)=>result, _=>panic!("could not read the counter file {}", counter_name)};
    let counter = match counter_raw.parse::<i32>() { Ok(result)=> result, _=>panic!("couldn't parse counter {}", counter_raw)};
    let mut formated_count = format!("{:#03}", counter);
    return formated_count;
}

pub fn get_signature() -> String{
    format!("AntonMakesGames {}", get_signature_counter())
}

pub fn increment_signature_counter() -> std::io::Result<()> {
    let counter_name = "Medias/counter";
    let counter_raw = match fs::read_to_string(counter_name) {Ok(result)=>result, _=>panic!("could not read the counter file {}", counter_name)};
    let counter = match counter_raw.parse::<i32>() { Ok(result)=> result, _=>panic!("couldn't parse counter {}", counter_raw)};
    fs::write(counter_name, format!("{}", counter + 1))?;
    Ok(())
}