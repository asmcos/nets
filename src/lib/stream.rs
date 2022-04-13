/*
*  stream is a network stream read and write operation library
*
*/
use std::ptr;

#[derive(Default)]
pub struct Stream{
     pub buf:Vec<u8>,
     pub length:usize,
     pub pos:usize,
}

impl Stream{
    pub fn new()->Self{
        Stream::default()
    }

    //copy raw data
    pub fn from_raw(data:* const u8 ,length:usize)->Self{
        let v = unsafe {
            let mut dst = Vec::with_capacity(length);
            ptr::copy(data,dst.as_mut_ptr(),length);
            dst.set_len(length);
            dst
        };
        
        Stream{
            buf:v,
            length:length,
            pos:0,
        }
    }

}

