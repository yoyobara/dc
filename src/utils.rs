/*
 * bunch of utilities
 */

use std::io::Read;

/* reads a single byte from a stream */
pub fn read_byte<T: Read>(stream: &mut T) -> Option<u8> {
    let mut buff = [0u8; 1];
    stream.read_exact(&mut buff).ok().map(|_| buff[0])
}
