use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



struct Mesh3D{
    xs : Vec<f64>,
    ys : Vec<f64>,
    zs : Vec<f64>,
    etov : Vec<i64>
}

impl Mesh3D{


    fn new(xs : Vec<f64>,ys : Vec<f64>, zs : Vec<f64>, etov : Vec<i64>) -> Mesh3D{
        Mesh3D { xs : xs, ys : ys, zs : zs, etov : etov }
    }

    fn from_file(path : String) -> Mesh3D{
        if let Ok(lines) = read_lines(path){
            for line in lines{
                if let Ok(ip) = line{
                    println!("{}",ip);
                }
            }
        }
        Mesh3D{ xs : vec![0.0], ys : vec![0.0], zs : vec![0.0], etov : vec![0]}
    }
}



#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_from_file(){
        let mesh_ = Mesh3D::from_file("./mesh_data/cube.msh".to_string());
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
