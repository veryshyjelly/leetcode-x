mod pkg;

use pkg::implement_trie::*;

fn main() {
    println!("{}", pkg::coin_change::coin_change(vec![1, 2, 5], 11));
}