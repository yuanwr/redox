use core::iter::Iterator;
use core::mem::size_of;
use core::ops::Add;
use core::ops::Drop;
use core::slice;
use core::slice::SliceExt;
use core::str::StrExt;

use common::debug::*;
use common::memory::*;

pub struct String {
    data: *const char,
    length: usize
}

impl String {
    pub fn new() -> String {
        String {
            data: 0 as *const char,
            length: 0
        }
    }

    // TODO FromStr trait
    pub fn from_str(s: &str) -> String {
        let length = s.chars().count();
        
        if length == 0 {
            return String::new();
        }
        
        let data = alloc(length * size_of::<char>());
    
        let mut i = 0;
        for c in s.chars() {
            unsafe {
                *((data + i * size_of::<char>()) as *mut char) = c;
            }
            i += 1;
        }
        
        String {
            data: data as *const char,
            length: length
        }
    }
    
    pub fn from_slice(s: &[char]) -> String {
        let length = s.len();
        
        if length == 0 {
            return String::new();
        }
        
        let data = alloc(length * size_of::<char>());
    
        let mut i = 0;
        for c in s {
            unsafe {
                *((data + i * size_of::<char>()) as *mut char) = *c;
            }
            i += 1;
        }
        
        String {
            data: data as *const char,
            length: length
        }
    }
    
    pub fn from_c_slice(s: &[u8]) -> String {
        let mut length = 0;
        for c in s {
            if *c == 0 {
                break;
            }
            length += 1;
        }
        
        if length == 0 {
            return String::new();
        }
        
        let data = alloc(length * size_of::<char>());
    
        let mut i = 0;
        for c in s {
            if i >= length {
                break;
            }
            unsafe {
                *((data + i * size_of::<char>()) as *mut char) = *c as char;
            }
            i += 1;
        }
        
        String {
            data: data as *const char,
            length: length
        }
    }
    
    pub unsafe fn from_c_str(s: *const u8) -> String {
        let mut length = 0;
        loop {
            if *(((s as usize) + length) as *const u8) == 0 {
                break;
            }
            length += 1;
        }
        
        if length == 0 {
            return String::new();
        }
        
        let data = alloc(length * size_of::<char>());
    
        for i in 0..length {
            *((data + i * size_of::<char>()) as *mut char) = *(((s as usize) + i) as *const u8) as char;
        }
        
        String {
            data: data as *const char,
            length: length
        }
    }
    
    pub fn from_num_radix(num: usize, radix: usize) -> String {
        if radix == 0 {
            return String::new();
        }
    
        let mut length = 1;
        let mut length_num = num;
        while length_num >= radix {
            length_num /= radix;
            length += 1;
        }
        
        let data = alloc(length * 4);
    
        let mut digit_num = num;
        for i in 0..length {
            let mut digit = (digit_num % radix) as u8;
            if digit > 9 {
                digit += 'A' as u8 - 10;
            }else{
                digit += '0' as u8;
            }
            
            unsafe {
                *((data + (length - 1 - i) * size_of::<char>()) as *mut char) = digit as char;
            }
            digit_num /= radix;
        }
        
        String {
            data: data as *const char,
            length: length
        }
    }
    
    pub fn from_char(c: char) -> String {
        if c == '\0' {
            return String::new();
        }
        
        let data = alloc(size_of::<char>());
        unsafe {
            *(data as *mut char) = c;
        }
        
        String {
            data: data as *const char,
            length: 1
        }
    }
    
    pub fn from_num(num: usize) -> String {
        String::from_num_radix(num, 10)
    }
    
    pub fn substr(&self, start: usize, len: usize) -> String {
        let mut i = start;
        if i > self.len() {
            i = self.len();
        }
    
        let mut j = i + len;
        if j > self.len() {
            j = self.len();
        }
        
        let length = j - i;
        if length == 0 {
            return String::new();
        }
        
        let data = alloc(length * 4);
    
        for k in i..j {
            unsafe {
                *((data + (k - i)*4) as *mut char) = *(((self.data as usize) + k*4) as *const char);
            }
        }
        
        String {
            data: data as *const char,
            length: length
        }
    }
    
    pub fn len(&self) -> usize {
        self.length
    }
    
    // TODO: Str trait
    pub fn as_slice(&self) -> &[char] {
        if self.data as usize == 0 || self.length == 0 {
            &[]
        }else{
            unsafe {
                slice::from_raw_parts(self.data, self.length)
            }
        }
    }
    
    pub fn d(&self){
        for c_ptr in self.as_slice() {
            dc(*c_ptr);
        }
    }
}

impl Drop for String {
    fn drop(&mut self){
        unalloc(self.data as usize);
        self.data = 0 as *const char;
        self.length = 0;
    }
}

impl Add for String {
    type Output = String;
    fn add(self, other: String) -> String {
        let length = self.length + other.length;
        
        if length == 0 {
            return String::new();
        }
        
        let data = alloc(length * 4);
    
        let mut i = 0;
        for c in self.as_slice() {
            unsafe {
                *((data + i * size_of::<char>()) as *mut char) = *c;
            }
            i += 1;
        }
        for c in other.as_slice() {
            unsafe {
                *((data + i * size_of::<char>()) as *mut char) = *c;
            }
            i += 1;
        }
    
        String {
            data: data as *const char,
            length: length
        }
    }
}

impl Add<&'static str> for String {
    type Output = String;
    fn add(self, other: &'static str) -> String {
        self + String::from_str(other)
    }
}

impl Add<char> for String {
    type Output = String;
    fn add(self, other: char) -> String {
        self + String::from_char(other)
    }
}

impl Add<usize> for String {
    type Output = String;
    fn add(self, other: usize) -> String {
        self + String::from_num(other)
    }
}