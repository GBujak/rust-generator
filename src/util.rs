use std::io;

pub fn lines<T: io::BufRead + std::fmt::Debug>(buf: T) -> Vec<String> {
    let desc = format!("{:?}", buf);
    buf.lines()
        .map(|x| x.expect(&format!("{:?}", desc)))
        .collect()
}
