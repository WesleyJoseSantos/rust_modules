use core::fmt;
use std::io;
use std::fmt::Result as StdResult;

fn function1() -> fmt::Result {
    Ok(())
}

fn function2() -> io::Result<()> {
    Ok(())    
}

fn function3() -> StdResult {
    Ok(())
}