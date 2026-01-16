use std::fs::File;
use std::io::Read;
use std::error::Error;

use crate::{byte_packet_buffer::BytePacketBuffer, dns_packet::DnsPacket};

mod byte_packet_buffer;
mod result_code;
mod dns_header;
mod query_type;
mod dns_question;
mod dns_record;
mod dns_packet;

fn main() -> Result<(), Box<dyn Error>> {
    let mut f = File::open("response_packet.txt")?;
    let mut buffer = BytePacketBuffer::new();
    f.read(&mut buffer.buf)?;

    let packet = DnsPacket::from_buffer(&mut buffer)?;
    println!("{:#?}", packet.header);

    for q in packet.questions {
        println!("{:#?}", q);
    }
    for rec in packet.answers {
        println!("{:#?}", rec);
    }
    for rec in packet.authorities {
        println!("{:#?}", rec);
    }
    for rec in packet.resources {
        println!("{:#?}", rec);
    }

    Ok(())
}
