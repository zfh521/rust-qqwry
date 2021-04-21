use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub struct QQWry {
    pub file:String
}
fn tou32(buf:&mut [u8])->u32{
    let b1:u32 = buf[0] as u32 & 0x0000FF;
    let b2:u32 = buf[1] as u32 & 0x0000FF;
    let b3:u32 = buf[2] as u32 & 0x0000FF;
    if buf.len()==4 {
        let b4:u32 = buf[3] as u32 & 0x0000FF;
        b4<<24|b3<<16|b2<<8|b1
    }else{
        b3<<16|b2<<8|b1
    }
}
fn ip_to_u32(ip:&str) -> u32{
    let k = ip.split(".").enumerate();
    let mut buf = vec![];
    for (_,item) in k {
        let no = item.parse::<u8>().unwrap();
        buf.push(no);
    }
    buf.reverse();
    return tou32(&mut buf);
}
fn u32_to_ip(ip:u32)->String{
    let b1 = (ip>>0 & 0xFF) as u8;
    let b2 = (ip>>8 & 0xFF) as u8;
    let b3 = (ip>>16 & 0xFF) as u8;
    let b4 = (ip>>24 & 0xFF) as u8;
    let b1 = b1.to_string();
    let b2 = b2.to_string();
    let b3 = b3.to_string();
    let b4 = b4.to_string();
    let dot = ".";
    let mut r = String::new();
    r+=&b4;r+=&dot;r+=&b3;r+=&dot;r+=&b2;r+=&dot;r+=&b1;
    r
}
fn read_as_string(file: &mut File) -> String{
    let  mut  _b1:[u8;1]=[0];
    let mut v: Vec<u8> = Vec::new();
    let mut len:u32=0;
    loop {
        file.read(&mut _b1).expect("read file error");
        len=len+1;
        if _b1[0]==('\0' as u8) {
            break;
        }
        v.push(_b1[0]);
    }
    let str = textcode::gb2312::decode_to_string(&v);
    return str;
}
fn read_as_area(file: &mut File) -> String{
    String::from("");
    let  mut  _b1:[u8;1]=[0];
    let  mut  b3:[u8;3]=[0,0,0];
    file.read(&mut _b1).expect("read file error");
    if _b1[0] == 0x01 || _b1[0] == 0x02 {
        file.read(&mut b3).expect("error");
        let data_offset = tou32(&mut b3);
        if data_offset == 0 {
            return String::from("unknown.area");
        }
        file.seek(SeekFrom::Start(data_offset as u64)).expect("read file error");
        return read_as_string(file);
    }else{
        file.seek(SeekFrom::Current(-1)).expect("read file error");
        return read_as_string(file);
    }
}
fn read_location(file:&mut File, record_offset: &mut u32) ->IPLocation {
    let mut location:IPLocation = IPLocation {
        index_offset: 0,
        start_ip:0,
        end_ip: 0,
        record_offset: *record_offset,
        country: String::from(""),
        area: String::from("")
    };
    let  mut  b4:[u8;4]=[0,0,0,0];
    let  mut  b3:[u8;3]=[0,0,0];
    let  mut  _b1:[u8;1]=[0];

    file.seek(SeekFrom::Start(*record_offset as u64)).expect("read file error");
    file.read(&mut b4).expect("read file error");
    location.end_ip = tou32(&mut b4);

    file.read(&mut _b1).expect("read file error");
    if _b1[0] == 0x01 || _b1[0] == 0x02{
        file.read(&mut b3).expect("read file error");
        let data_offset = tou32(&mut b3);
        file.seek(SeekFrom::Start(data_offset as u64)).expect("read file error");
        if _b1[0] == 0x01 {
            file.read(&mut _b1).expect("read file error");
            let area_offset:u64;
            if _b1[0] == 0x02 {
                area_offset = (data_offset+4) as u64;
                file.read(&mut b3).expect("read file error");
                let data_offset = tou32(&mut b3);
                file.seek(SeekFrom::Start(data_offset as u64)).expect("read file error");
                location.country = read_as_string(file);
            }else{
                file.seek(SeekFrom::Current(-1)).expect("read file error");
                location.country = read_as_string(file);
                area_offset = file.stream_position().unwrap();
            }
            file.seek(SeekFrom::Start(area_offset)).expect("read file error");
            location.area = read_as_area(file);
        } else {
            location.country = read_as_string(file);
            location.area = read_as_area(file);
        }

    }else{
        file.seek(SeekFrom::Current(-1)).expect("read file error");
        location.country = read_as_string(file);
        location.area = read_as_area(file);
    }
    location
}
#[derive(Debug)]
pub struct IPLocation {
    pub index_offset: u32,
    pub record_offset: u32,
    pub start_ip: u32,
    pub end_ip:u32,
    pub country: String,
    pub area:String
}
impl IPLocation {
    pub fn get_start_ip_str(&self)->String{
        u32_to_ip(self.start_ip)
    }
    pub fn get_end_ip_str(&self)->String{
        u32_to_ip(self.end_ip)
    }
}
impl QQWry {
    pub fn from(file:  String) -> QQWry {
        QQWry {
            file
        }
    }
    /**
    * 读取IP信息
    */
    pub fn read_ip_location(&mut self,ip: &str) -> Option<IPLocation> {
        let ip = ip_to_u32(ip);
        let mut file = File::open(&mut self.file).unwrap();
        let  mut  b4:[u8;4]=[0,0,0,0];
        let  mut  b3:[u8;3]=[0,0,0];
        let  mut  _b1:[u8;1]=[0];

        file.read(&mut b4).unwrap();
        let  first_index_offset:u32 = tou32(&mut b4);
        file.read(&mut b4).unwrap();
        let  last_index_offset:u32 = tou32(&mut b4);
        let total_index_records = (last_index_offset-first_index_offset)/7+1;
        let mut i:u32=0;
        let mut right_edge = false;
        let mut result: Option<IPLocation> = None;
        loop {
            let index_offset = first_index_offset+i*7;
            file.seek(SeekFrom::Start(index_offset as u64)).expect("read file error");
            file.read(&mut b4).expect("read file error");
            file.read(&mut b3).expect("read file error");

            let  start_ip = tou32(&mut b4);
            if ip == start_ip {
                let mut record_idx = tou32(&mut b3);
                let mut location = read_location(&mut file, &mut record_idx);
                location.index_offset = index_offset;
                location.start_ip = start_ip;
                result = Some(location);
                break;
            }
            if ip < start_ip {
                i-=1;
                right_edge = true;
                continue;
            }
            if right_edge {
                let mut record_idx = tou32(&mut b3);
                let mut location = read_location(&mut file, &mut record_idx);
                if ip<=location.end_ip {
                    location.index_offset = index_offset;
                    location.start_ip = start_ip;
                    result = Some(location);
                }
                break;
            }
            i = i+1;
            if i==total_index_records {
                let mut record_idx = tou32(&mut b3);
                let mut location = read_location(&mut file, &mut record_idx);
                if ip<=location.end_ip {
                    location.index_offset = index_offset;
                    location.start_ip = start_ip;
                    result = Some(location);
                }
                break;
            }
        }
        return result
    }
}