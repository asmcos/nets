/*
*  stream is a network stream read and write operation library
*
*/
#[derive(Default)]
pub struct Stream{
     buf:Vec<u8>,
     length:usize,
     pos:usize,
}

impl Stream{
    pub fn new()->Self{
        Stream::default()
    }
    pub fn from_raw(data:* mut u8 ,length:usize)->Self{
        let v = unsafe {Vec::from_raw_parts(data,length,length)};
        Stream{
            buf:v,
            length:length,
            pos:0,
        }
    }

}

