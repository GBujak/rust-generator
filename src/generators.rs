use crate::lines;
use rand::Rng;
use std::fmt::Display;
pub use std::iter::FromIterator;

pub trait Generator {
    fn next(&mut self) -> &dyn Display;
}

pub struct RangeGenerator {
    range: (i32, i32),
    val: i32,
}

impl RangeGenerator {
    pub fn new(l: i32, h: i32) -> Self {
        RangeGenerator {
            range: (l, h),
            val: 0,
        }
    }
    pub fn new_from_format(format: &str) -> Self {
        let split = format
            .split("-")
            .map(|x| x.parse::<i32>().expect("can't parse range format"))
            .collect::<Vec<i32>>();
        assert_eq!(split.len(), 2);
        Self::new(split[0], split[1])
    }
}

impl Generator for RangeGenerator {
    fn next(&mut self) -> &dyn Display {
        let (l, h) = self.range;
        self.val = rand::thread_rng().gen_range(l, h);
        &self.val
    }
}

pub struct FileGenerator {
    lines: Vec<String>,
}

impl FileGenerator {
    pub fn new(filename: &str) -> Self {
        let file = std::fs::File::open(filename).expect("can't open file");
        let bufreader = std::io::BufReader::new(file);
        FileGenerator {
            lines: lines(bufreader),
        }
    }
}

impl Generator for FileGenerator {
    fn next(&mut self) -> &dyn Display {
        let index = rand::thread_rng().gen_range(0, self.lines.len());
        &self.lines[index]
    }
}

pub struct GeneratorVector {
    gens: Vec<Box<dyn Generator>>,
}

impl GeneratorVector {
    pub fn next_line(&mut self) -> String {
        let mut ret = self
            .gens
            .iter_mut()
            .map(|x| format!("{}", x.next()))
            .collect::<Vec<_>>()
            .join(",");
        ret.push(',');
        ret
    }
}

impl<'a> FromIterator<&'a str> for GeneratorVector {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let mut ret = Self { gens: Vec::new() };
        for i in iter {
            if i.contains("-") {
                ret.gens.push(Box::new(RangeGenerator::new_from_format(i)));
            } else {
                ret.gens.push(Box::new(FileGenerator::new(i)));
            };
        }
        ret
    }
}
