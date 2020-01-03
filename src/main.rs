use std::cmp::Ordering;

#[derive(Debug)]
enum BTree<T: Ord> {
  Node{
    v: T,
    l: Box<BTree<T>>,
    r: Box<BTree<T>>,
  },
  Empty,
}

impl<T: Ord> BTree<T> {

  fn new() -> BTree<T> {

    BTree::Empty

  }

  fn insert(&mut self, nv: T) {

    match self{

      &mut BTree::Node{ ref v, ref mut l, ref mut r }  => {
        let c = nv.cmp(v);
          match c{
            Ordering::Less => { l.insert(nv); },
            Ordering::Greater => { r.insert(nv); },
            _ => return //Already in the tree
          };

      },
      &mut BTree::Empty => {
        /*Found an empty leaf -- insert*/
        *self = BTree::Node{ v: nv, l: Box::new(BTree::Empty), r: Box::new(BTree::Empty)};

      }

    };
  }

  fn is_empty(&self) -> bool {

    match self{
      &BTree::Empty => {
        true  
      },
      _ => false
    }
  }

  fn find(&self, fv: T) -> bool {
    match self{
      &BTree::Node{ ref v, ref l, ref r } => {
        let c = fv.cmp(v);
        match c{
          Ordering::Less => l.find(fv),
          Ordering::Greater => r.find(fv),
          _ => true
        }
      },
      &BTree::Empty => { false }
    }
  }
}

fn main(){
  let mut t: BTree<u64> = BTree::new();
  println!("Is Empty? {}",t.is_empty());
  t.insert(128);
  t.insert (256);
  t.insert(4500000001);
  println!("256: {}",t.find(256));
  println!("128: {}",t.find(128));
  println!("12: {}",t.find(12));
}
