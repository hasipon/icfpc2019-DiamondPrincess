
use pos::value_at;
use std::mem::swap;
use std::collections::HashSet;

pub fn union_find(map:&Vec<bool>, size:i32) -> i32 {
    let tree_size = ((size + 2) * (size + 2)) as usize;
    let mut tree = vec![-1; tree_size];
    for x in 0..size + 2 {
        for y in 0..size + 2 {
            if y > 0 {
                let value0 = value_at(&map, x - 1, y - 1, size);
                let value1 = value_at(&map, x - 1, y - 2, size);
                if value0 == value1 {
                    let tree0 = y * (size + 2) + x;
                    let tree1 = (y - 1) * (size + 2) + x;
                    unite(&mut tree, tree0 as usize, tree1 as usize);
                }
            }
            if x > 0 {
                let value0 = value_at(&map, x - 1, y - 1, size);
                let value1 = value_at(&map, x - 2, y - 1, size);
                if value0 == value1 {
                    let tree0 = y * (size + 2) + x;
                    let tree1 = y * (size + 2) + (x - 1);
                    unite(&mut tree, tree0 as usize, tree1 as usize);
                }
            }
        }
    }
    
    let mut root_set = HashSet::new();
    for i in 0..tree_size {
        root_set.insert(root(&mut tree, i));
    }
    root_set.len() as i32
}


pub fn unite(tree:&mut Vec<i32>, a:usize, b:usize) {
    let mut rb = root(tree, b);
    let mut ra = root(tree, a);
    
    if ra != rb {
        let rank_a = tree[ra];
        let rank_b = tree[rb];
        if rank_a > rank_b {
            swap(&mut ra, &mut rb);
        } else if rank_a == rank_b {
            tree[ra] -= 1;
        }
        if tree[rb] != ra as i32 {
            tree[rb] = ra as i32;
        }
    }
}
pub fn root(tree:&mut Vec<i32>, a:usize) -> usize {
    let b = tree[a];
    if b < 0 { 
        a 
    } else { 
        let value = root(tree, b as usize) as i32;
        if tree[a] != value {
            tree[a] = value;
        }
        value as usize
    }
}
