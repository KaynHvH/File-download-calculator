use std::io;

fn main() {
    let mut file_size = String::new();
    let mut internet_speed = String::new();

    println!("What's your internet speed?");
    io::stdin().read_line(&mut internet_speed).unwrap();

    println!("What's your file size?");
    io::stdin().read_line(&mut file_size).unwrap();

    let internet_speed: f32 = internet_speed.trim().parse().expect("Internet speed cannot be parsed as a number");
    let converted_speed = internet_speed / 8 as f32;

    let file_size: f32 = file_size.trim().parse().expect("File size cannot be parsed as a number");
    let file_size_to_mb = (file_size * 1024.0) * 1.0;

    let convert_to_minutes = (file_size_to_mb / converted_speed) / 600.0;
    println!("Your file will be installed in {} minutes!", convert_to_minutes as i32);
}