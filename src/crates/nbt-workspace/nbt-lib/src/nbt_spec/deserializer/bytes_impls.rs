use super::*;

impl NBTDeserializeBytes for i8 {
    #[inline]
    fn read_from_bytes(cursor: &mut Cursor<Vec<u8>>) -> NBTResult<Self> {
        cursor.read_i8()
    }
}

impl NBTDeserializeBytes for i16 {
    #[inline]
    fn read_from_bytes(cursor: &mut Cursor<Vec<u8>>) -> NBTResult<Self> {
        cursor.read_i16()
    }
}

impl NBTDeserializeBytes for i32 {
    #[inline]
    fn read_from_bytes(cursor: &mut Cursor<Vec<u8>>) -> NBTResult<Self> {
        cursor.read_i32()
    }
}

impl NBTDeserializeBytes for i64 {
    #[inline]
    fn read_from_bytes(cursor: &mut Cursor<Vec<u8>>) -> NBTResult<Self> {
        cursor.read_i64()
    }
}

impl NBTDeserializeBytes for f32 {
    #[inline]
    fn read_from_bytes(cursor: &mut Cursor<Vec<u8>>) -> NBTResult<Self> {
        cursor.read_f32()
    }
}

impl NBTDeserializeBytes for f64 {
    #[inline]
    fn read_from_bytes(cursor: &mut Cursor<Vec<u8>>) -> NBTResult<Self> {
        cursor.read_f64()
    }
}

impl NBTDeserializeBytes for String {
    #[inline]
    fn read_from_bytes(cursor: &mut Cursor<Vec<u8>>) -> NBTResult<Self> {
        cursor.read_nbt_string()
    }
}

impl<T: NBTDeserializeBytes> NBTDeserializeBytes for Vec<T> {
    #[inline]
    fn read_from_bytes(cursor: &mut Cursor<Vec<u8>>) -> NBTResult<Self> {
        let len = cursor.read_i32()?;
        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(T::read_from_bytes(cursor)?);
        }
        Ok(vec)
    }
}