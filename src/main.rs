
extern crate indextree;
use indextree::Arena;

fn main(){
// Create a new arena
let arena = &mut Arena::new();

// Add some new nodes to the arena
let a = arena.new_node(1);
let b = arena.new_node(2);

// Append b to a
a.append(b, arena);
assert_eq!(b.ancestors(arena).into_iter().count(), 2);
}