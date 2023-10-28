use std::fmt;

pub trait AnyWrite {
    type Wstr: ?Sized;
    type Error;

    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Self::Error>;

    fn write_str(&mut self, s: &Self::Wstr) -> Result<(), Self::Error>;
}

impl<W: fmt::Write> AnyWrite for W {
    type Wstr = str;
    type Error = fmt::Error;

    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Self::Error> {
        fmt::Write::write_fmt(self, fmt)
    }

    fn write_str(&mut self, s: &Self::Wstr) -> Result<(), Self::Error> {
        fmt::Write::write_str(self, s)
    }
}
