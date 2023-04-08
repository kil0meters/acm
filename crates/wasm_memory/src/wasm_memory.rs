use anyhow::Result;
use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use wasmtime::*;

pub type AllocatorFunc = TypedFunc<i32, i32>;

pub trait WasmMemory: Sized {
    const CPP_SIZE_OF: usize;

    /// returns the address of the item allocated
    fn into_memory<S>(
        self,
        store: &mut Store<S>,
        memory: &Memory,
        allocator: &AllocatorFunc,
        offset: Option<usize>,
    ) -> Result<usize>;

    fn from_memory<S>(store: &mut Store<S>, memory: &Memory, offset: usize) -> Result<Self>;
}

/// maps to `std::vector<T>`
///
/// c++ vector memory representation
/// - start pointer: u32
/// - end pointer: u32
/// - capacity pointer: u32 impl<T> WasmMemory for Vec<T> where
impl<T> WasmMemory for Vec<T>
where
    T: WasmMemory,
{
    const CPP_SIZE_OF: usize = 12;

    fn into_memory<S>(
        self,
        mut store: &mut Store<S>,
        memory: &Memory,
        allocator: &AllocatorFunc,
        offset: Option<usize>,
    ) -> Result<usize> {
        let mut data = [0; 12];

        let vec_address = offset.unwrap_or_else(|| {
            allocator
                .call(&mut store, Self::CPP_SIZE_OF as i32)
                .unwrap() as usize
        });

        let buffer_address =
            allocator.call(&mut store, (T::CPP_SIZE_OF * self.len()) as i32)? as usize;

        LittleEndian::write_i32_into(
            &[
                buffer_address as i32,                                 // start pointer
                (buffer_address + T::CPP_SIZE_OF * self.len()) as i32, // end pointer
                (buffer_address + T::CPP_SIZE_OF * self.len()) as i32, // capacity pointer
            ],
            &mut data,
        );

        memory.write(&mut store, vec_address as usize, &data)?;

        // this setup basically results in a tree being created
        for (i, element) in self.into_iter().enumerate() {
            element.into_memory(
                &mut store,
                memory,
                allocator,
                Some(buffer_address + i * T::CPP_SIZE_OF),
            )?;
        }

        Ok(vec_address)
    }

    fn from_memory<S>(mut store: &mut Store<S>, memory: &Memory, offset: usize) -> Result<Self> {
        let mut buf = [0; 8];

        memory.read(&mut store, offset, &mut buf)?;

        let start = LittleEndian::read_u32(&buf);
        let end = LittleEndian::read_u32(&buf[4..]);

        let mut result = vec![];

        for child_offset in (start..end).step_by(T::CPP_SIZE_OF) {
            result.push(T::from_memory(&mut store, &memory, child_offset as usize)?);
        }

        Ok(result)
    }
}

/// maps to `std::string`
///
/// c++ string memory representation (for long strings)
/// data (pointer) - u32
/// size (length) - u32
/// capacity (length) - u32
impl WasmMemory for String {
    const CPP_SIZE_OF: usize = 12;

    fn into_memory<S>(
        self,
        mut store: &mut Store<S>,
        memory: &Memory,
        allocator: &AllocatorFunc,
        offset: Option<usize>,
    ) -> Result<usize> {
        let bytes = self.into_bytes();
        let length = bytes.len();

        // the least significant bit of capacity determines whether we're dealing with a "short
        // string." This should be implemented for the sake of completeness but it is not necessary
        // to get a functional string.
        //
        // Capacity is guaranteed to be divisible by two so if the length is odd we add two,
        // otherwise we add one. We also need to count the null terminator for the capacity.
        let capacity = if (length + 1) % 2 == 0 {
            (length + 1) | (1 << 31)
        } else {
            (length + 2) | (1 << 31)
        };

        let address = offset.unwrap_or_else(|| {
            allocator
                .call(&mut store, Self::CPP_SIZE_OF as i32)
                .unwrap() as usize
        });

        let buffer_address = allocator.call(&mut store, bytes.len() as i32)? as usize;

        let mut data = [0; 12];
        LittleEndian::write_i32_into(
            &[
                buffer_address as i32, // start pointer
                length as i32,         // end pointer
                capacity as i32,       // capacity pointer
            ],
            &mut data,
        );

        memory.write(&mut store, address, &data)?;
        memory.write(&mut store, buffer_address, &bytes)?;

        Ok(address)
    }

    fn from_memory<S>(mut store: &mut Store<S>, memory: &Memory, offset: usize) -> Result<Self> {
        let mut buf = [0; 12];

        memory.read(&mut store, offset, &mut buf)?;

        // if small string (most significant bit of capacity)
        if buf[11] & 7 != 0 {
            // read bytes until we reach a null terminator in the small string case
            let length = buf.iter().position(|b| *b == '\0' as u8).unwrap();

            Ok(String::from_utf8_lossy(&buf[0..length]).to_string())
        } else {
            let start = LittleEndian::read_u32(&buf);
            let length = LittleEndian::read_u32(&buf[4..]);

            // long string mode
            // TODO: Support reading short strings
            let mut string_bytes = vec![0; length as usize];

            memory.read(&mut store, start as usize, &mut string_bytes)?;

            Ok(String::from_utf8_lossy(&string_bytes).to_string())
        }
    }
}

/// maps to `std::pair<A, B>`
impl<A, B> WasmMemory for (A, B)
where
    A: WasmMemory,
    B: WasmMemory,
{
    const CPP_SIZE_OF: usize = A::CPP_SIZE_OF + B::CPP_SIZE_OF;

    #[rustfmt::skip]
    fn into_memory<S>(
        self,
        mut store: &mut Store<S>,
        memory: &Memory,
        allocator: &AllocatorFunc,
        offset: Option<usize>,
    ) -> Result<usize> {
        let address = offset.unwrap_or_else(|| {
            allocator
                .call(&mut store, Self::CPP_SIZE_OF as i32)
                .unwrap() as usize
        });

        self.0.into_memory(&mut store, memory, allocator, Some(address))?;
        self.1.into_memory(&mut store, memory, allocator, Some(address + A::CPP_SIZE_OF))?;

        Ok(address)
    }

    fn from_memory<S>(mut store: &mut Store<S>, memory: &Memory, offset: usize) -> Result<Self> {
        let a = A::from_memory(&mut store, &memory, offset)?;
        let b = B::from_memory(&mut store, &memory, offset + A::CPP_SIZE_OF)?;

        Ok((a, b))
    }
}

impl WasmMemory for f32 {
    const CPP_SIZE_OF: usize = 4;

    fn into_memory<S>(
        self,
        mut store: &mut Store<S>,
        memory: &Memory,
        allocator: &AllocatorFunc,
        offset: Option<usize>,
    ) -> Result<usize> {
        let address = offset.unwrap_or_else(|| {
            allocator
                .call(&mut store, Self::CPP_SIZE_OF as i32)
                .unwrap() as usize
        });

        let mut buf = vec![];
        buf.write_f32::<LittleEndian>(self)?;
        memory.write(store, address, &buf)?;

        Ok(address)
    }

    fn from_memory<S>(mut store: &mut Store<S>, memory: &Memory, offset: usize) -> Result<Self> {
        let mut buf = [0; 4];

        memory.read(&mut store, offset, &mut buf)?;
        let result = LittleEndian::read_f32(&buf);

        Ok(result)
    }
}

/// maps to `int32_t` or `int`
impl WasmMemory for i32 {
    const CPP_SIZE_OF: usize = 4;

    fn into_memory<S>(
        self,
        mut store: &mut Store<S>,
        memory: &Memory,
        allocator: &AllocatorFunc,
        offset: Option<usize>,
    ) -> Result<usize> {
        let address = offset.unwrap_or_else(|| {
            allocator
                .call(&mut store, Self::CPP_SIZE_OF as i32)
                .unwrap() as usize
        });

        let mut buf = vec![];
        buf.write_i32::<LittleEndian>(self)?;
        memory.write(store, address, &buf)?;

        Ok(address)
    }

    fn from_memory<S>(mut store: &mut Store<S>, memory: &Memory, offset: usize) -> Result<Self> {
        let mut buf = [0; 4];

        memory.read(&mut store, offset, &mut buf)?;
        let result = LittleEndian::read_i32(&buf);

        Ok(result)
    }
}

/// maps to `int64_t` or `long`
impl WasmMemory for i64 {
    const CPP_SIZE_OF: usize = 8;

    fn into_memory<S>(
        self,
        mut store: &mut Store<S>,
        memory: &Memory,
        allocator: &AllocatorFunc,
        offset: Option<usize>,
    ) -> Result<usize> {
        let address = offset.unwrap_or_else(|| {
            allocator
                .call(&mut store, Self::CPP_SIZE_OF as i32)
                .unwrap() as usize
        });

        let mut buf = vec![];
        buf.write_i64::<LittleEndian>(self)?;
        memory.write(store, address, &buf)?;

        Ok(address)
    }

    fn from_memory<S>(mut store: &mut Store<S>, memory: &Memory, offset: usize) -> Result<Self> {
        let mut buf = [0; 8];

        memory.read(&mut store, offset, &mut buf)?;
        let result = LittleEndian::read_i64(&buf);

        Ok(result)
    }
}

/// maps to `double`
impl WasmMemory for f64 {
    const CPP_SIZE_OF: usize = 8;

    fn into_memory<S>(
        self,
        mut store: &mut Store<S>,
        memory: &Memory,
        allocator: &AllocatorFunc,
        offset: Option<usize>,
    ) -> Result<usize> {
        let address = offset.unwrap_or_else(|| {
            allocator
                .call(&mut store, Self::CPP_SIZE_OF as i32)
                .unwrap() as usize
        });

        let mut buf = vec![];
        buf.write_f64::<LittleEndian>(self)?;
        memory.write(store, address, &buf)?;

        Ok(address)
    }

    fn from_memory<S>(mut store: &mut Store<S>, memory: &Memory, offset: usize) -> Result<Self> {
        let mut buf = [0; 8];

        memory.read(&mut store, offset, &mut buf)?;
        let result = LittleEndian::read_f64(&buf);

        Ok(result)
    }
}

/// maps to `char`
impl WasmMemory for char {
    const CPP_SIZE_OF: usize = 1;

    fn into_memory<S>(
        self,
        mut store: &mut Store<S>,
        memory: &Memory,
        allocator: &AllocatorFunc,
        offset: Option<usize>,
    ) -> Result<usize> {
        let address = offset.unwrap_or_else(|| {
            allocator
                .call(&mut store, Self::CPP_SIZE_OF as i32)
                .unwrap() as usize
        });

        let mut buf = vec![];
        buf.write_u8(self as u8)?;
        memory.write(store, address, &buf)?;

        Ok(address)
    }

    fn from_memory<S>(mut store: &mut Store<S>, memory: &Memory, offset: usize) -> Result<Self> {
        let mut buf = [0; 1];

        memory.read(&mut store, offset, &mut buf)?;

        Ok(buf[0] as char)
    }
}

impl WasmMemory for bool {
    const CPP_SIZE_OF: usize = 1;

    fn into_memory<S>(
        self,
        mut store: &mut Store<S>,
        memory: &Memory,
        allocator: &AllocatorFunc,
        offset: Option<usize>,
    ) -> Result<usize> {
        let address = offset.unwrap_or_else(|| {
            allocator
                .call(&mut store, Self::CPP_SIZE_OF as i32)
                .unwrap() as usize
        });

        let mut buf = vec![];
        buf.write_u8(self as u8)?;
        memory.write(store, address, &buf)?;

        Ok(address)
    }

    fn from_memory<S>(mut store: &mut Store<S>, memory: &Memory, offset: usize) -> Result<Self> {
        let mut buf = [0; 1];
        memory.read(&mut store, offset, &mut buf)?;

        Ok(buf[0] != 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_setup() -> Result<(Store<usize>, Memory, AllocatorFunc)> {
        let engine = Engine::default();
        let mut store = Store::new(&engine, 0);
        let memory = Memory::new(&mut store, MemoryType::new(1, None))?;
        memory.grow(&mut store, 2)?;
        let linker = Linker::new(&engine);
        let module = Module::new(&engine, include_bytes!("../test.wasm"))?;
        let instance = linker.instantiate(&mut store, &module)?;
        let allocator: AllocatorFunc = instance.get_typed_func(&mut store, "alloc")?;

        Ok((store, memory, allocator))
    }

    #[test]
    fn vec_i32() -> Result<()> {
        let (mut store, memory, allocator) = test_setup()?;

        let initial = (0..10).collect::<Vec<_>>();
        let address = initial
            .clone()
            .into_memory(&mut store, &memory, &allocator, None)?;

        let result = Vec::<i32>::from_memory(&mut store, &memory, address)?;

        assert_eq!(initial, result);

        Ok(())
    }

    #[test]
    fn vec_nested() -> Result<()> {
        let (mut store, memory, allocator) = test_setup()?;

        let initial = vec![vec![vec![vec![1, 2, 3, 4]]]];
        let address = initial
            .clone()
            .into_memory(&mut store, &memory, &allocator, None)?;

        let result = Vec::<Vec<Vec<Vec<i32>>>>::from_memory(&mut store, &memory, address)?;

        assert_eq!(initial, result);

        Ok(())
    }

    #[test]
    fn string() -> Result<()> {
        let (mut store, memory, allocator) = test_setup()?;

        let initial = "hello world".to_string();

        let address = initial
            .clone()
            .into_memory(&mut store, &memory, &allocator, None)?;

        let result = String::from_memory(&mut store, &memory, address)?;

        assert_eq!(initial, result);

        Ok(())
    }

    #[test]
    fn vec_string() -> Result<()> {
        let (mut store, memory, allocator) = test_setup()?;

        let initial = vec![
            "Longing heart aches deep,".to_string(),
            "Echoes of distant love call,".to_string(),
            "Silent tears fall asleep.".to_string(),
        ];

        let address = initial
            .clone()
            .into_memory(&mut store, &memory, &allocator, None)?;

        let result = Vec::<String>::from_memory(&mut store, &memory, address)?;

        assert_eq!(initial, result);

        Ok(())
    }

    #[test]
    fn pair() -> Result<()> {
        let (mut store, memory, allocator) = test_setup()?;

        let initial = (69, 3.14159);

        let address = initial.into_memory(&mut store, &memory, &allocator, None)?;

        let result = <(i32, f32)>::from_memory(&mut store, &memory, address)?;

        assert_eq!(initial, result);

        Ok(())
    }

    #[test]
    fn vec_pair_vec() -> Result<()> {
        let (mut store, memory, allocator) = test_setup()?;

        let initial = (
            vec![(1, 1.0), (2, 2.0), (3, 3.0), (4, 4.0), (5, 5.0)],
            vec![(1.0, 1), (2.0, 2), (3.0, 3), (4.0, 4), (5.0, 5)],
        );

        let address = initial
            .clone()
            .into_memory(&mut store, &memory, &allocator, None)?;

        let result =
            <(Vec<(i32, f32)>, Vec<(f32, i32)>)>::from_memory(&mut store, &memory, address)?;

        assert_eq!(initial, result);

        Ok(())
    }
}
