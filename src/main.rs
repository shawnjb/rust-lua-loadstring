use mlua::{Error, Lua, Result};

fn lua_loadstring(lua: &Lua, code: &str) -> Result<()> {
    let chunk = lua.load(code);
    match chunk.exec() {
        Ok(()) => Ok(()),
        Err(error) => {
            match error {
                Error::SyntaxError { .. } => {
                    eprintln!("Syntax error: {:?}", error);
                }
                Error::RuntimeError(_) => {
                    eprintln!("Runtime error: {:?}", error);
                }
                Error::MemoryError(_) => {
                    eprintln!("Memory error: {:?}", error);
                }
                #[cfg(any(feature = "lua53", feature = "lua52", doc))]
                Error::GarbageCollectorError(_) => {
                    eprintln!("Garbage collector error: {:?}", error);
                }
                Error::SafetyError(_) => {
                    eprintln!("Safety error: {:?}", error);
                }
                Error::MemoryLimitNotAvailable => {
                    eprintln!("Memory limit not available: {:?}", error);
                }
                Error::MainThreadNotAvailable => {
                    eprintln!("Main thread not available: {:?}", error);
                }
                Error::RecursiveMutCallback => {
                    eprintln!("Recursive mut callback: {:?}", error);
                }
                Error::CallbackDestructed => {
                    eprintln!("Callback destructed: {:?}", error);
                }
                Error::StackError => {
                    eprintln!("Stack error: {:?}", error);
                }
                Error::BindError => {
                    eprintln!("Bind error: {:?}", error);
                }
                Error::ToLuaConversionError { .. } => {
                    eprintln!("To lua conversion error: {:?}", error);
                }
                Error::FromLuaConversionError { .. } => {
                    eprintln!("From lua conversion error: {:?}", error);
                }
                Error::CoroutineInactive => {
                    eprintln!("Coroutine inactive: {:?}", error);
                }
                Error::UserDataTypeMismatch => {
                    eprintln!("User data type mismatch: {:?}", error);
                }
                Error::UserDataDestructed => {
                    eprintln!("User data destructed: {:?}", error);
                }
                Error::UserDataBorrowError => {
                    eprintln!("User data borrow error: {:?}", error);
                }
                Error::UserDataBorrowMutError => {
                    eprintln!("User data borrow mut error: {:?}", error);
                }
                Error::MetaMethodRestricted(_) => {
                    eprintln!("Meta method restricted: {:?}", error);
                }
                Error::MetaMethodTypeError { .. } => {
                    eprintln!("Meta method type error: {:?}", error);
                }
                Error::MismatchedRegistryKey => {
                    eprintln!("Mismatched registry key: {:?}", error);
                }
                Error::CallbackError { .. } => {
                    eprintln!("Callback error: {:?}", error);
                }
                Error::PreviouslyResumedPanic => {
                    eprintln!("Previously resumed panic: {:?}", error);
                }
                #[cfg(feature = "serialize")]
                Error::SerializeError(_) => {
                    eprintln!("Serialize error: {:?}", error);
                }
                #[cfg(feature = "serialize")]
                Error::DeserializeError(_) => {
                    eprintln!("Deserialize error: {:?}", error);
                }
                Error::ExternalError(_) => {
                    eprintln!("External error: {:?}", error);
                }

                _ => {
                    eprintln!("Unknown error: {:?}", error);
                }
            }
            Err(error)
        }
    }
}

fn main() -> Result<()> {
    let lua = Lua::new();
    lua_loadstring(&lua, "print('Hello, world!')")?;
    Ok(())
}
