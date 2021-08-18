mod types;
mod parser;
mod impls;

fn main() {
    parser::parse("(1d6,1d8,1d10)mi2[fire]+2d20kh1+4/7*11-6+3d10ra10");
}
