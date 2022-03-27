//
// http parse
//
use httparse;


pub fn isRequest(buf:&[u8])->bool{
        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut req = httparse::Request::new(&mut headers);

        let ret = req.parse(buf);
        match ret {
            Ok(Complete)=>{println!("{:?},{:?}",ret,req);},
            _ =>{}
        }
        
        true 
}

