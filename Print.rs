#[derive(Debug)]
struct Lang {
    language: String,
    version: String,
}
fn main() {
    let lang = "Rust";
    println!("Hello, {lang}");

    let x = 2;
    println!("{0} x {1}={2}", x, x * x, x * x * x);

    let lang = Lang {
        language: "rust".to_string(),
        version: "1.61.0".to_string(),
    };
    println!("Language:{},Version:{}", lang.language, lang.version);
    println!("{:?}", lang);
    println!("{:#?}", lang);
}
