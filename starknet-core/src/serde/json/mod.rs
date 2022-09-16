use serde::Serialize;
use serde_json::{ser::Formatter, Serializer};
use std::io;

/// A `serde_json` formatter that mimicks the output of `json.dumps()` in Python. This is primarily
/// used in `hinted_class_hash` calculation to obtain the exact same hash as in `cairo-lang`.
#[derive(Debug)]
pub struct PythonicJsonFormatter;

impl Formatter for PythonicJsonFormatter {
    #[inline]
    fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if first {
            Ok(())
        } else {
            writer.write_all(b", ")
        }
    }

    #[inline]
    fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if first {
            Ok(())
        } else {
            writer.write_all(b", ")
        }
    }

    #[inline]
    fn begin_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b": ")
    }
}

#[inline]
pub fn to_string_pythonic<T>(value: &T) -> Result<String, serde_json::Error>
where
    T: ?Sized + Serialize,
{
    let vec = to_vec_pythonic(value)?;
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}

#[inline]
fn to_vec_pythonic<T>(value: &T) -> Result<Vec<u8>, serde_json::Error>
where
    T: ?Sized + Serialize,
{
    let mut writer = Vec::with_capacity(128);
    to_writer_pythonic(&mut writer, value)?;
    Ok(writer)
}

#[inline]
fn to_writer_pythonic<W, T>(writer: W, value: &T) -> Result<(), serde_json::Error>
where
    W: io::Write,
    T: ?Sized + Serialize,
{
    let mut ser = Serializer::with_formatter(writer, PythonicJsonFormatter);
    value.serialize(&mut ser)?;
    Ok(())
}
