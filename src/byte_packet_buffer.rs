use std::error::Error;

pub struct BytePacketBuffer {
    pub buf: [u8; 512],
    pub pos: usize,
}

impl BytePacketBuffer {

    pub fn new() -> BytePacketBuffer {
        BytePacketBuffer {
            buf: [0; 512],
            pos: 0,
        }
    }

    fn pos(&self) -> usize {
        self.pos
    }

    pub fn step(&mut self, steps: usize) -> Result<(), Box<dyn Error>> {
        self.pos += steps;

        Ok(())
    }

    fn seek(&mut self, pos: usize) -> Result<(), Box<dyn Error>> {
        self.pos = pos;

        Ok(())
    }

    pub fn read(&mut self) -> Result<u8, Box<dyn Error>> {
        if self.pos >= 512 {
            return Err("End of Buffer".into());
        }
        let res = self.buf[self.pos];
        self.pos += 1;

        Ok(res)
    }

    fn get(&mut self, pos: usize) -> Result<u8, Box<dyn Error>> {
        if pos >= 512 {
            return Err("End of Buffer".into());
        }
        Ok(self.buf[pos])
    }

    fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8], Box<dyn Error>> {
        if start + len >= 512 {
            return Err("End of Buffer".into())
        }
        Ok(&self.buf[start..start + len as usize])
    }

    pub fn read_u16(&mut self) -> Result<u16, Box<dyn Error>> {
        let res = ((self.read()? as u16) << 8) | (self.read()? as u16);

        Ok(res)
    }

    pub fn read_u32(&mut self) -> Result<u32, Box<dyn Error>> {
        let res = ((self.read()? as u32) << 24)
            | ((self.read()? as u32) << 16)
            | ((self.read()? as u32) << 8)
            | ((self.read()? as u32) << 0);
        
        Ok(res)
    }

    pub fn read_qname(&mut self, outstr: &mut String) -> Result<(), Box<dyn Error>> {
        let mut pos = self.pos();

        let mut jumped = false;
        let max_jumps = 5;
        let mut jumps_performed = 0;

        let mut delim = "";
        loop {
            if jumps_performed > max_jumps {
                return Err(format!("Limit of {} jumps exceeded", max_jumps).into());
            }

            let len = self.get(pos)?;

            if (len & 0xC0) == 0xC0 {
                if !jumped {
                    self.seek(pos + 2)?;
                }

                let b2 = self.get(pos + 1)? as u16;
                let offset = (((len as u16) ^ 0xC0) << 8) | b2;
                pos = offset as usize;

                jumped = true;
                jumps_performed += 1;

                continue;
            }
            else {
                pos += 1;

                if len == 0 {
                    break;
                }

                outstr.push_str(delim);

                let str_buffer = self.get_range(pos, len as usize)?;
                outstr.push_str(&String::from_utf8_lossy(str_buffer).to_lowercase());

                delim = ".";

                pos += len as usize;
            }
        }

        if !jumped {
            self.seek(pos)?;
        }

        Ok(())
    }
}