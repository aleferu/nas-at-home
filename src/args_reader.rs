use std::env;


pub fn look_for_flag(flag_name: &str) -> bool {
    let flag_name = format!("--{flag_name}");
    for arg in env::args() {
        if arg == flag_name {
            return true;
        }
    }
    false
}


pub fn look_for_option(setting_name: &str) -> Option<String> {
    let setting_name = format!("--{setting_name}");
    let mut args = env::args().into_iter();
    while let Some(arg) = args.next() {
        if arg == setting_name {
            if let Some(setting) = args.next(){
                if setting.starts_with("--") { return None; }
                return Some(setting);
            } else {
                return None;
            }
        }
    }
    None
}
