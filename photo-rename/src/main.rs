mod photo_rename;


fn main() {
    let input = "./in/";
    let output = "./out/";
    photo_rename::run_for(input, output).expect("TODO: panic message");
}
