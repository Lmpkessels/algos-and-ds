fn value(s: u8) -> i32 {
    match s as char {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0
    }
}

pub fn roman_to_int(s: String) -> i32 {
    let string_to_uppercase = s.to_ascii_uppercase();
    let string_as_bytes = string_to_uppercase.as_bytes();
    let mut integer = 0;
    
    let mut i = 0;
    while i < string_as_bytes.len() {
        
        let current = value(string_as_bytes[i]);

        let next = if i + 1 < string_as_bytes.len() {
            value(string_as_bytes[i + 1])
        } else {
            0
        };
        
        if current < next {
            integer -= current;
        } else {
            integer += current;
        }
        
        i += 1;
    }
    integer
}