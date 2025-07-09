
/// Provides a wrapper around a block of bytes in memory to
/// help with tracking the size of the currently written data.
/// 
/// The primary purpose of this is to help with allocating and
/// using a block of memory on the stack on embedded devices
/// without needing to cause new allocations.
pub struct PacketBuffer<'a> {
    pub size: usize,
    pub data: &'a mut [u8]
}

impl<'a> From<&'a mut [u8]> for PacketBuffer<'a> {
    fn from(value: &'a mut [u8]) -> Self {
        Self {
            size: 0,
            data: value
        }
    }
}

impl<'a> PacketBuffer<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        Self{
            size: 0,
            data
        }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.data[0..self.size]
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn reset(&mut self) {
        self.size = 0;
    }

    pub fn put_u8(&mut self, n: u8) {
        self.data[self.size] = n;
        self.size += 1;
    }

    pub fn put_u16(&mut self, n: u16) {
        for byte in &n.to_be_bytes() {
            self.data[self.size] = *byte;
            self.size += 1;
        }       
    }

    pub fn put_u32(&mut self, n: u32) {
        for byte in &n.to_be_bytes() {
            self.data[self.size] = *byte;
            self.size += 1;
        }       
    }

    pub fn put_u64(&mut self, n: u64) {
        for byte in &n.to_be_bytes() {
            self.data[self.size] = *byte;
            self.size += 1;
        }       
    }

    pub fn put(&mut self, src: &[u8]) {
        for byte in src {
            self.data[self.size] = *byte;
            self.size += 1;
        }
    }
}

impl AsRef<[u8]> for PacketBuffer<'_> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.bytes()
    }
}