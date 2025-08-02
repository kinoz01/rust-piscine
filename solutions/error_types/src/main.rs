use error_types::*;

fn main() {
    let mut form_output = Form {
        name: "Lee".to_owned(),
        password: "qwqwsa1dty_".to_owned(),
    };

    println!("{:?}", form_output);
    println!("{:?}", form_output.validate());

    form_output.name = "".to_owned();
    println!("{:?}", form_output.validate());

    form_output.name = "as".to_owned();
    form_output.password = "dty_1".to_owned();
    println!("{:?}", form_output.validate());

    form_output.password = "asdasASd(_".to_owned();
    println!("{:?}", form_output.validate());

    form_output.password = "asdasASd123SA".to_owned();
    println!("{:?}", form_output.validate());
}