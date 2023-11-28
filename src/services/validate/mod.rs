pub struct InputValidator {}

impl InputValidator {
    pub fn new() -> InputValidator {
        InputValidator {}
    }

    pub fn validate_service_type(service_type: u8) {
        match service_type {
            0 | 1  => {},
            _ => panic!("０か１を入力してください。"),
        }
    }
    
    pub fn validate_register_type(register_type: u8) {
        match register_type {
            0 | 1  => {},
            _ => panic!("０か１を入力してください。"),
        }
    }

    pub fn validate_category_type(register_type: u8, category_type: u8) {
        if register_type == 0 {
            match category_type {
                0 | 1 | 2  => {},
                _ => panic!("０〜2の数字を入力してください。"),
            }
        } else {
            match category_type {
                0 | 1 | 2  => {},
                _ => panic!("０-2の数字を入力してください。"),
            }
        }
    }
}