use std::io::{Error, Read, Write};
pub fn evaluate<F, R, W>(reader: &mut R, writer: &mut W, mut callback: F) -> Result<(), Error>
where
    F: FnMut(&[u8]) -> Result<Vec<u8>, Error>,
    R: Read,
    W: Write,
{
    let mut buf = Vec::new();
    reader.read(&mut buf)?;
    println!("{:?}", buf);
    let result = callback(&buf)?;
    writer.write(&result)?;
    println!("{:?}", buf);
    Ok(())
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_io_read_and_write() {
        use super::*;
        let mut stdin = std::io::stdin();
        let mut stdout = std::io::stdout();
        let _ = evaluate(&mut stdin, &mut stdout, |x| {
            let x = x.to_vec().iter().map(|z| z * 2).collect();
            Ok(x)
        })
        .unwrap();
    }
}
