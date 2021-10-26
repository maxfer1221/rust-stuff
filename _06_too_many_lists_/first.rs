use std::mem; // Replace borrows

/*=============================
__Why doesn't this work?__
The size of the List depends on the number of elements in
the list. So we won't know how much memory to allocate for "Elem"
If we introduce a Box, like in the following example, it works cont.

pub enum List {
    Empty,
    Elem(i32, List),
}
=============================*/
/*=============================
__Why does this work?__
Boxes are of definite size. "Elem" is now of definite size as well.
As such, we can allocate a specific amount of memory for a List.

__How would we use it?__
fn main() {
    let list: List<i32> = List::Elem(1, Box::new(List::Elem(2,Nil));
    println!("{:?}", list);
}

__Why is it bad?__
1. The first element is not heap-allocated, but rather allocated in
the stack. Instead, we'd simply like a pointer on the stack which
points at the location in the heap with our initial node.
2. We will be allocating a node which is saying it is not a node (?)

pub enum List {
    Empty,
    Elem(i32, Box<List>),
}
=============================*/
/*=============================
__Why does this work?__
Read above.

__Why is it bad?__
1. Non-uniform definition of nodes. Some nodes have the variant
ElemThenNotEmpty, whereas some have the variant ElemThenEmpty.
2. There are states which are simply invalid:
ElemThenNotEmpty(0, Box(Empty))

pub enum List {
    Empty,
    ElemThenEmpty(i32),
    ElemThenNotEmpty(i32, Box<List>),
}
=============================*/
/*=============================
__Why doesn't this work?__
The List is marked as public, but Nodes themselves aren't. The
internals of the enum are public, but we can't publicly talk about
private types, such as the Nodes. An easy solution would be making
Node public, but Rust prefers keeping private implementations.

struct Node {
    elem: i32,
    next: List,
}

pub enum List {
    Empty,
    More(Box<Node>),
}
=============================*/
/*=============================
__Why does this work?__
List is public, however Links are not. This does not cause immedeate
issues, but it means that no one is able to efficiently use Lists
as heads will be private/hidden.

__Why is this bad?__
Read above
*/

pub struct List {
    head: Link,
} // Size is the same as Link!
  // struct with single field has the same size as that field!

enum Link {
  Empty,
  More(Box<Node>),
}

struct Node {
  elem: i32,
  next: Link,
}

impl List {
  pub fn new() -> Self {
      List { head: Link::Empty }
  }
}
/*===========================*/
/*=============================
__Why doesn't this work?__
Rust notices that we are attempting to move the value out of self.head. This is cause for concern
as if a worker thread was to read the value of self.head while the value was gone, we'd run into
an exception.

impl List {
    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem: elem,
            next: self.head,
        };
    }
}
=============================*/
/*=============================
__Why doesn't this work?__
See above. Rust can still see the value will be gone.

impl List {
    pub fn push(&mut self, elem: i32) {
        let new_node: Box<Node> = Box::new(Node {
            elem: elem,
            next: self.head
        });
        self.head = Link::More(new_node);
    }
}
=============================*/
/*=============================
__Why does this work?__
mem::replace will "steal" a value from a borrow by replacing it with another value we pass in. In
this case, we are replacing &mut self's head with an Empty field from the Link enum. After this
replacement, we can call 'self.head = Link::More(new_node);' in order to replace the "Empty" link
with the node we just created.

__Why is this bad?__
Not sure.
*/

impl List {
    pub fn push(&mut self, elem: i32){
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }
}
/*===========================*/
/*=============================
__Why doesn't this work?__
Pattern matching with match will try to move contents into the branch, but we do not own self,
hence Rust getting mad at us.

impl List {
    pub fn pop(&mut self) -> Option<i32> {
        match self.head {
            Link::Empty => {
                // TODO
            }
            Link::More(node) => {
                // TODO
            }
        };
        unimplemented!()
    }
}
=============================*/
/*=============================
__Why does this fix work?__
We are now using a reference to self.head.

__Why doesn't the rest work?
'self.head = node.next;' is attempting to move ownership of node.next to
*/


impl List {
/*
    pub fn pop(&mut self) -> Option<i32> {
        let result;
        match &self.head {
            Link::Empty => {
                result = None;
            }
            Link::More(node) => {
                self.head = node.next;
                result = Some(node.elem);
            }
        };
        result
    }
*/

    pub fn pop2(&mut self) -> Option<i32> {
        let result;
        match &mut self.head {
            Link::Empty => {
                result = None;
            }
            Link::More(ref mut node) => {
                result = Some(node.elem);
                self.head = mem::replace(&mut node.next, Link::Empty);
            }
        };
        result
    }
    pub fn pop(&mut self) -> Option<i32>{
        let result;
        match &mut self.head {
            &mut Link::Empty => {
                result = None;
            }
            &mut Link::More(ref mut node) => {
                result = Some(node.elem);
                self.head = mem::replace(&mut node.next, Link::Empty);
            }
        };
        result
    }
}
/*
=============================*/
/*=============================
__Why does this fix work?__
We are replacing

impl List {
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}


qn
i'm not entirely understanding why the following doesn't work
```rs
struct List {
  head: Link,
}

impl List {
  fn pop(&mut self) -> Option<i32> {
    let result;
    match &self.head {
      Link::Empty => {
        result = None;
      }
      Link::More(node) => {
        result = node.elem;
        self.head = node.next; // Issue
      }
    }
    return result;
  }
}

enum Link {
  Empty,
  More(Box<Node>),
}

struct Node {
  elem: i32,
  next: Link
}
```
*/
/*
//doesn't work
pub fn pop(&mut self) -> Option<i32> {
  let result;
  match &self.head {
    Link::Empty => {
      result = None
    }
    Link::More(node) => {
      result = Some(node.nex);
      self.head = node.next;
//                ^^^^^^^^^ now only fails here?
//                "cannot move out of 'node.nex' which is behind a shared reference"
    }
  }
}
//works?
pub fn pop(&mut self) -> Option<i32> {
  match mem::replace(&mut self.head, Link::Empty) {
    Link::Empty => None,
    Link::More(node) => {
      self.head = node.next;
      Some(node.elem)
    }
  }
}
```
*/
/*
this doesn't work because we're borrowing self.head
```rs
pub fn pop2(&mut self) -> Option<i32> {
  let result;
  match &mut self.head {
    Link::Empty => {
      result = None;
    }
    Link::More(mut node) => {
      result = Some(node.elem);
      self.head = mem::replace(&mut node.next, Link::Empty);
    }
  }
}
```
but this would take ownership of self.head which we can then change?
```rs
pub fn pop2(&mut self) -> Option<i32> {
  let result;
  match std::mem:replace(&mut self.head, Link::Empty) { //!!
    Link::Empty => {
      result = None;
    }
    Link::More(mut node) => {
      result = Some(node.elem);
      self.head = mem::replace(&mut node.next, Link::Empty);
    }
  }
}
```
*/
