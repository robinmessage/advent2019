use std::fs::File;
use std::fs;
use std::io::{self, prelude::*, BufReader};
use std::cmp;

struct Image {
    data: Vec<u8>,
    width: usize,
    height: usize,
    layers: usize
}

impl Image {
    pub fn new(input: &str, width: usize, height: usize) -> Image {
        let layer_size = width * height;
        let total_size = input.len();
        if total_size % layer_size != 0 {
            panic!("Expected multiple of {}, got {} (remainder {})", layer_size, total_size, total_size % layer_size);
        }

        return Image { data: input.as_bytes().iter().map(|b| b - ('0' as u8)).collect(), width: width, height: height, layers: total_size / layer_size };
    }

    pub fn get_pixel(self:&Image, x: usize, y: usize, l: usize) -> u8 {
        return self.data[x + y * self.width + l * self.width * self.height];
    }

    pub fn get(self:&Image, x: usize, y: usize) -> u8 {
        for l in 0..self.layers {
            let p = self.get_pixel(x, y, l);
            if p == 2 {
                continue;
            }
            return p;
        }
        panic!("Pixel at {}, {} is maformed", x, y);
    }
}

fn main() {
    let image = Image::new(&fs::read_to_string("input").expect("Couldn't read input").trim(), 25, 6);

    let mut minimumLayer = 0;
    let mut minimumZeros = image.height * image.width;
    for l in 0..image.layers {
        let mut zeros = 0;
        for y in 0..image.height {
            for x in 0..image.width {
                zeros += if image.get_pixel(x, y, l) == 0 {1} else {0};
            }
        }
        if zeros < minimumZeros {
            minimumZeros = zeros;
            minimumLayer = l;
        }
    }
    let mut ones = 0;
    let mut twos = 0;
    for y in 0..image.height {
        for x in 0..image.width {
            ones += if image.get_pixel(x, y, minimumLayer) == 1 {1} else {0};
            twos += if image.get_pixel(x, y, minimumLayer) == 2 {1} else {0};
        }
    }
    println!("{}", ones * twos);

    for y in 0..image.height {
        println!("{}", (0..image.width).map(|x| 
            if image.get(x, y) == 1 {"*"} else {" "}
        ).collect::<Vec<&str>>().join(""));
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_load() {
    }
}
