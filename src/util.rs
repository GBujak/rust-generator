use std::io;

pub fn lines<T: io::BufRead>(buf: T) -> Vec<String> {
    buf.lines()
        .map(|x| x.unwrap_or_else(|err| panic!("{:?}", err)))
        .collect()
}
