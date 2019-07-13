use std::io::{Read, Seek, SeekFrom};
use core::Token;

macro_rules! make_token {
    ($bytes:ident) => {
        if !$bytes.is_empty() {
            let s = String::from_utf8($bytes.to_vec()).unwrap();
            match s.as_ref() {
                "module" => Some(Token::Module),
                "import" => Some(Token::Import),
                "func" => Some(Token::Func),
                _ if $bytes[0] == b'$' => Some(Token::Name(s[1..].to_string())),
                _ => Some(Token::Symbol(s)),
            }                    
        } else {
            None
        }        
    };
}

pub fn lex(reader: &mut (impl Read + Seek)) -> Option<Token> {    
    let mut c: &mut [u8] = &mut [0;1];
    let mut token_bytes: Vec<u8> = vec![];

    loop {
        if let Ok(n) = reader.read(&mut c) {
            if n > 0 {
                // println!("c: {:?}", c);
                match c[0] {
                    b'(' => { return Some(Token::LeftParen); },
                    b')' => { return Some(Token::RightParen); },
                    b' ' => {},
                    _ => {
                        token_bytes.push(c[0]);
                        return lex_chars(reader, &mut token_bytes);
                    },
                }                            
            } else {
                return make_token!(token_bytes);
            }
        } else {
            // 本当はエラーを返したほうがいい            
            return None;
        }        
    }
}

fn lex_chars(reader: &mut (impl Read + Seek), token_bytes: &mut Vec<u8>) -> Option<Token> {
    let mut c: &mut [u8] = &mut [0;1];
    loop {
        if let Ok(_) = reader.read(&mut c) {
            match c[0] {
                b'(' | b')' | b' ' => {
                    reader.seek(SeekFrom::Current(-1)).unwrap();
                    return make_token!(token_bytes);                                    
                },
                _ => {
                    token_bytes.push(c[0]);
                }
            }
        } else {
            // 本当はエラーを返したほうがいい            
            return None;
        }
    }
}