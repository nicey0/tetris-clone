mod pieces;

use pieces::Piece;

fn main() {
    let mut p = Piece::new([0.0, 0.0, 0.0, 0.0],
                       [
                       vec![(0, 0), (1, 0), (2, 0)],
                       vec![(0, 1), (1, 1), (2, 1)],
                       vec![(0, 2), (1, 2), (2, 2)],
                       vec![(0, 3), (1, 3), (2, 3)],
                       ]);
    println!("0: {:?}", p.get_shape());
    p.rotate(true);
    println!("1: {:?}", p.get_shape());
    p.shift(-1, 0);
    println!("1 (x-1 shift): {:?}", p.get_shape());
}
