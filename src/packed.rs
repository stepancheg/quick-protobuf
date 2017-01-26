//! A module to handle `Packed` iterator

use reader::Reader;

/// An iterator over packed repeated fields
///
/// Contrary to unpacked repeated fields, packed are contiguous in memory
pub struct Packed<'a, F> {
    reader: Reader<'a>,
    read: F,
}

impl<'a, M, F: FnMut(&mut Reader<'a>) -> M> Iterator for Packed<'a, F> {
    type Item = M;
    fn next(&mut self) -> Option<M> {
        if self.reader.is_eof() {
            None
        } else {
            Some((self.read)(&mut self.reader))
        }
    }
}

impl<'a, F> Packed<'a, F> {

    /// Creates a new `Packed` iterator based on the chunk of data of the packed fields
    /// and a read closure to parse each item
    pub fn new(bytes: &'a [u8], read: F) -> Packed<'a, F> {
        let reader = Reader::from_bytes(bytes);
        Packed {
            reader: reader,
            read: read,
        }
    }

}
