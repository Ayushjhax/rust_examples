fn main() {
    let mut setting_value = Some(5);
    let  new_setting_value = Some(6);

match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
        println!("Cant't overwrite an existign customized value!");
    }
    _ => {
        setting_value = new_setting_value;
    }
}


println!("setting is {:?}", setting_value);
}