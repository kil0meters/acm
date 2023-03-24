use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use wasmtime::*;

use crate::{AllocatorFunc, WasmMemory};

#[derive(Error, Debug)]
enum FunctionError {
    #[error("Expected a function with name \"{0}\", but it was not found.")]
    NameNotFound(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ContainerVariantType {
    Graph,
    Grid,
    List,
    Single,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ContainerVariant<T: WasmMemory> {
    Graph(Vec<Vec<T>>),
    Grid(Vec<Vec<T>>),
    List(Vec<T>),
    Single(T),
}

impl<T> ContainerVariant<T>
where
    T: WasmMemory,
{
    fn into_memory<S>(
        self,
        store: &mut Store<S>,
        memory: &Memory,
        allocator: &AllocatorFunc,
    ) -> Result<usize> {
        match self {
            ContainerVariant::Graph(graph) => graph.into_memory(store, memory, allocator, None),
            ContainerVariant::Grid(grid) => grid.into_memory(store, memory, allocator, None),
            ContainerVariant::List(list) => list.into_memory(store, memory, allocator, None),
            ContainerVariant::Single(single) => single.into_memory(store, memory, allocator, None),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FunctionValue {
    String(ContainerVariant<String>),
    Int(ContainerVariant<i32>),
    Long(ContainerVariant<i64>),
    Float(ContainerVariant<f32>),
    Double(ContainerVariant<f64>),
    Char(ContainerVariant<char>),
    Bool(ContainerVariant<bool>),
}

impl FunctionValue {
    fn into_memory<S>(
        self,
        store: &mut Store<S>,
        memory: &Memory,
        allocator: &AllocatorFunc,
    ) -> Result<usize> {
        let next_offset = match self {
            FunctionValue::String(s) => s.into_memory(store, memory, allocator)?,
            FunctionValue::Int(i) => i.into_memory(store, memory, allocator)?,
            FunctionValue::Float(f) => f.into_memory(store, memory, allocator)?,
            FunctionValue::Char(c) => c.into_memory(store, memory, allocator)?,
            FunctionValue::Bool(b) => b.into_memory(store, memory, allocator)?,
            FunctionValue::Long(l) => l.into_memory(store, memory, allocator)?,
            FunctionValue::Double(d) => d.into_memory(store, memory, allocator)?,
        };

        Ok(next_offset)
    }

    pub fn scaling_factor(&self) -> f32 {
        match self {
            FunctionValue::String(ContainerVariant::Single(s)) => s.len() as f32,
            FunctionValue::String(ContainerVariant::List(l)) => l.len() as f32,
            FunctionValue::String(ContainerVariant::Grid(g)) => (g[0].len() * g.len()) as f32,
            FunctionValue::String(ContainerVariant::Graph(g)) => g.len() as f32, // for now we just do number of nodes, should factor edges though

            FunctionValue::Int(ContainerVariant::Single(s)) => s.abs() as f32,
            FunctionValue::Int(ContainerVariant::List(l)) => l.len() as f32,
            FunctionValue::Int(ContainerVariant::Grid(g)) => (g[0].len() * g.len()) as f32,
            FunctionValue::Int(ContainerVariant::Graph(g)) => g.len() as f32,

            FunctionValue::Long(ContainerVariant::Single(s)) => s.abs() as f32,
            FunctionValue::Long(ContainerVariant::List(l)) => l.len() as f32,
            FunctionValue::Long(ContainerVariant::Grid(g)) => (g[0].len() * g.len()) as f32,
            FunctionValue::Long(ContainerVariant::Graph(g)) => g.len() as f32,

            FunctionValue::Float(ContainerVariant::Single(s)) => s.abs() as f32,
            FunctionValue::Float(ContainerVariant::List(l)) => l.len() as f32,
            FunctionValue::Float(ContainerVariant::Grid(g)) => (g[0].len() * g.len()) as f32,
            FunctionValue::Float(ContainerVariant::Graph(g)) => g.len() as f32,

            FunctionValue::Double(ContainerVariant::Single(s)) => s.abs() as f32,
            FunctionValue::Double(ContainerVariant::List(l)) => l.len() as f32,
            FunctionValue::Double(ContainerVariant::Grid(g)) => (g[0].len() * g.len()) as f32,
            FunctionValue::Double(ContainerVariant::Graph(g)) => g.len() as f32,

            FunctionValue::Char(ContainerVariant::Single(_)) => 1.0,
            FunctionValue::Char(ContainerVariant::List(l)) => l.len() as f32,
            FunctionValue::Char(ContainerVariant::Grid(g)) => (g[0].len() * g.len()) as f32,
            FunctionValue::Char(ContainerVariant::Graph(g)) => g.len() as f32,

            FunctionValue::Bool(ContainerVariant::Single(_)) => 1.0,
            FunctionValue::Bool(ContainerVariant::List(l)) => l.len() as f32,
            FunctionValue::Bool(ContainerVariant::Grid(g)) => (g[0].len() * g.len()) as f32,
            FunctionValue::Bool(ContainerVariant::Graph(g)) => g.len() as f32,
        }
    }
}

impl PartialEq for FunctionValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Compare singleton doubles with less precision because there can be annoying
            // precision issues when solving problems otherwise.
            (
                FunctionValue::Double(ContainerVariant::Single(left)),
                FunctionValue::Double(ContainerVariant::Single(right)),
            ) => (*left - *right).abs() < 1e-9,

            (FunctionValue::String(left), FunctionValue::String(right)) => left == right,
            (FunctionValue::Int(left), FunctionValue::Int(right)) => left == right,
            (FunctionValue::Long(left), FunctionValue::Long(right)) => left == right,
            (FunctionValue::Char(left), FunctionValue::Char(right)) => left == right,
            (FunctionValue::Bool(left), FunctionValue::Bool(right)) => left == right,

            _ => false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FunctionType {
    String(ContainerVariantType),
    Int(ContainerVariantType),
    Long(ContainerVariantType),
    Float(ContainerVariantType),
    Double(ContainerVariantType),
    Char(ContainerVariantType),
    Bool(ContainerVariantType),
}

impl FunctionType {
    fn from_memory<S>(
        &self,
        store: &mut Store<S>,
        memory: &Memory,
        offset: usize,
    ) -> Result<FunctionValue> {
        let res = match self {
            FunctionType::String(ContainerVariantType::Graph) => FunctionValue::String(
                ContainerVariant::Graph(Vec::<Vec<String>>::from_memory(store, memory, offset)?),
            ),
            FunctionType::String(ContainerVariantType::Grid) => FunctionValue::String(
                ContainerVariant::Grid(Vec::<Vec<String>>::from_memory(store, memory, offset)?),
            ),
            FunctionType::String(ContainerVariantType::List) => FunctionValue::String(
                ContainerVariant::List(Vec::<String>::from_memory(store, memory, offset)?),
            ),
            FunctionType::String(ContainerVariantType::Single) => FunctionValue::String(
                ContainerVariant::Single(String::from_memory(store, memory, offset)?),
            ),

            FunctionType::Int(ContainerVariantType::Graph) => FunctionValue::Int(
                ContainerVariant::Graph(Vec::<Vec<i32>>::from_memory(store, memory, offset)?),
            ),
            FunctionType::Int(ContainerVariantType::Grid) => FunctionValue::Int(
                ContainerVariant::Grid(Vec::<Vec<i32>>::from_memory(store, memory, offset)?),
            ),
            FunctionType::Int(ContainerVariantType::List) => FunctionValue::Int(
                ContainerVariant::List(Vec::<i32>::from_memory(store, memory, offset)?),
            ),
            FunctionType::Int(ContainerVariantType::Single) => FunctionValue::Int(
                ContainerVariant::Single(i32::from_memory(store, memory, offset)?),
            ),

            FunctionType::Long(ContainerVariantType::Graph) => FunctionValue::Long(
                ContainerVariant::Graph(Vec::<Vec<i64>>::from_memory(store, memory, offset)?),
            ),
            FunctionType::Long(ContainerVariantType::Grid) => FunctionValue::Long(
                ContainerVariant::Grid(Vec::<Vec<i64>>::from_memory(store, memory, offset)?),
            ),
            FunctionType::Long(ContainerVariantType::List) => FunctionValue::Long(
                ContainerVariant::List(Vec::<i64>::from_memory(store, memory, offset)?),
            ),
            FunctionType::Long(ContainerVariantType::Single) => FunctionValue::Long(
                ContainerVariant::Single(i64::from_memory(store, memory, offset)?),
            ),

            FunctionType::Float(ContainerVariantType::Graph) => FunctionValue::Float(
                ContainerVariant::Graph(Vec::<Vec<f32>>::from_memory(store, memory, offset)?),
            ),
            FunctionType::Float(ContainerVariantType::Grid) => FunctionValue::Float(
                ContainerVariant::Grid(Vec::<Vec<f32>>::from_memory(store, memory, offset)?),
            ),
            FunctionType::Float(ContainerVariantType::List) => FunctionValue::Float(
                ContainerVariant::List(Vec::<f32>::from_memory(store, memory, offset)?),
            ),
            FunctionType::Float(ContainerVariantType::Single) => FunctionValue::Float(
                ContainerVariant::Single(f32::from_memory(store, memory, offset)?),
            ),

            FunctionType::Double(ContainerVariantType::Graph) => FunctionValue::Double(
                ContainerVariant::Graph(Vec::<Vec<f64>>::from_memory(store, memory, offset)?),
            ),
            FunctionType::Double(ContainerVariantType::Grid) => FunctionValue::Double(
                ContainerVariant::Grid(Vec::<Vec<f64>>::from_memory(store, memory, offset)?),
            ),
            FunctionType::Double(ContainerVariantType::List) => FunctionValue::Double(
                ContainerVariant::List(Vec::<f64>::from_memory(store, memory, offset)?),
            ),
            FunctionType::Double(ContainerVariantType::Single) => FunctionValue::Double(
                ContainerVariant::Single(f64::from_memory(store, memory, offset)?),
            ),

            FunctionType::Char(ContainerVariantType::Graph) => FunctionValue::Char(
                ContainerVariant::Graph(Vec::<Vec<char>>::from_memory(store, memory, offset)?),
            ),
            FunctionType::Char(ContainerVariantType::Grid) => FunctionValue::Char(
                ContainerVariant::Grid(Vec::<Vec<char>>::from_memory(store, memory, offset)?),
            ),
            FunctionType::Char(ContainerVariantType::List) => FunctionValue::Char(
                ContainerVariant::List(Vec::<char>::from_memory(store, memory, offset)?),
            ),
            FunctionType::Char(ContainerVariantType::Single) => FunctionValue::Char(
                ContainerVariant::Single(char::from_memory(store, memory, offset)?),
            ),

            FunctionType::Bool(ContainerVariantType::Graph) => FunctionValue::Bool(
                ContainerVariant::Graph(Vec::<Vec<bool>>::from_memory(store, memory, offset)?),
            ),
            FunctionType::Bool(ContainerVariantType::Grid) => FunctionValue::Bool(
                ContainerVariant::Grid(Vec::<Vec<bool>>::from_memory(store, memory, offset)?),
            ),
            FunctionType::Bool(ContainerVariantType::List) => FunctionValue::Bool(
                ContainerVariant::List(Vec::<bool>::from_memory(store, memory, offset)?),
            ),
            FunctionType::Bool(ContainerVariantType::Single) => FunctionValue::Bool(
                ContainerVariant::Single(bool::from_memory(store, memory, offset)?),
            ),
        };

        Ok(res)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WasmFunctionCall {
    pub name: String,
    pub arguments: Vec<FunctionValue>,
    pub return_type: FunctionType,
}

impl WasmFunctionCall {
    const PAGE_OFFSET: usize = 4;

    pub fn new(name: &str, arguments: Vec<FunctionValue>, return_type: FunctionType) -> Self {
        WasmFunctionCall {
            name: name.into(),
            arguments,
            return_type,
        }
    }

    // Returns the return value of the function, along with the fuel consumed *purely* by the
    // invocation of that funcion, not the memory allocation of passing the arguments.
    pub fn call<S>(
        self,
        mut store: &mut Store<S>,
        instance: &Instance,
    ) -> Result<(FunctionValue, u64)> {
        let allocator: AllocatorFunc = instance.get_typed_func(&mut store, "alloc")?;
        let memory = instance
            .get_memory(&mut store, "memory")
            .expect("Failed to get memory");

        memory.grow(&mut store, Self::PAGE_OFFSET as u64)?;

        let mut params = vec![];
        let mut results = vec![];

        // If the return value is not a simple singleton, we instead put the address for the
        // return value to be written to as the first argument
        match self.return_type {
            FunctionType::Int(ContainerVariantType::Single)
            | FunctionType::Char(ContainerVariantType::Single)
            | FunctionType::Bool(ContainerVariantType::Single) => {
                results.push(Val::I32(0));
            }

            FunctionType::Long(ContainerVariantType::Single) => {
                results.push(Val::I64(0));
            }

            FunctionType::Float(ContainerVariantType::Single) => {
                results.push(Val::F32(0));
            }

            FunctionType::Double(ContainerVariantType::Single) => {
                results.push(Val::F64(0));
            }

            _ => params.push(Val::I32(0 as i32)),
        }

        const ARG_ALLOC_FUEL_DEFAULT: u64 = 100_000_000_000;
        // because we need to allocate for some args, it's possible to improperly run out of fuel
        store.add_fuel(ARG_ALLOC_FUEL_DEFAULT)?;

        for arg in self.arguments {
            match arg {
                FunctionValue::Int(ContainerVariant::Single(i)) => params.push(Val::I32(i)),
                FunctionValue::Long(ContainerVariant::Single(l)) => params.push(Val::I64(l)),
                FunctionValue::Float(ContainerVariant::Single(f)) => {
                    params.push(Val::F32(f.to_bits()))
                }
                FunctionValue::Double(ContainerVariant::Single(d)) => {
                    params.push(Val::F64(d.to_bits()))
                }
                FunctionValue::Char(ContainerVariant::Single(c)) => params.push(Val::I32(c as i32)),
                FunctionValue::Bool(ContainerVariant::Single(b)) => params.push(Val::I32(b as i32)),

                _ => {
                    let address = arg.into_memory(store, &memory, &allocator)?;
                    params.push(Val::I32(address as i32));
                }
            }
        }

        let initial_fuel = store.fuel_consumed().unwrap();
        // store.consume_fuel(ARG_ALLOC_FUEL_DEFAULT - initial_fuel)?;
        // let initial_fuel = store.fuel_consumed().unwrap();

        instance
            .get_func(&mut store, &self.name)
            .ok_or_else(|| FunctionError::NameNotFound(self.name))?
            .call(&mut store, &params, &mut results)?;

        let after_fuel = store.fuel_consumed().unwrap();

        // If the return type is a simple singleton, we can simply take the value directly from the
        // return value. Otherwise, we must read it from memory, with the address given by the
        // first parameter.
        let return_value = match self.return_type {
            FunctionType::Int(ContainerVariantType::Single) => {
                FunctionValue::Int(ContainerVariant::Single(results[0].unwrap_i32()))
            }

            FunctionType::Long(ContainerVariantType::Single) => {
                FunctionValue::Long(ContainerVariant::Single(results[0].unwrap_i64()))
            }

            FunctionType::Char(ContainerVariantType::Single) => FunctionValue::Char(
                ContainerVariant::Single(results[0].unwrap_i32() as u8 as char),
            ),

            FunctionType::Bool(ContainerVariantType::Single) => {
                FunctionValue::Bool(ContainerVariant::Single(results[0].unwrap_i32() != 0))
            }

            FunctionType::Float(ContainerVariantType::Single) => {
                FunctionValue::Float(ContainerVariant::Single(results[0].unwrap_f32()))
            }

            FunctionType::Double(ContainerVariantType::Single) => {
                FunctionValue::Double(ContainerVariant::Single(results[0].unwrap_f64()))
            }

            _ => self
                .return_type
                .from_memory(store, &memory, params[0].unwrap_i32() as usize)?,
        };

        Ok((return_value, after_fuel - initial_fuel))
    }
}
