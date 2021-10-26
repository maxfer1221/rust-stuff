// struct Node<'a> {
//     next: Option<&'a Node<'a>>,
//     val: Type,
// }
//
// enum Type {
//     I(i32),
//     S(String),
//     B(bool),
// }
//
// fn main() {
//     let mut first: Node = Node {
//         next: None,
//         val: Type::I(1),
//     };
//     let mut second: Node = Node {
//         next: None,
//         val: Type::I(1),
//     };
//
//     first.next = Some(&second);
//     second.next = Some(&first);
// }

struct List {
    head: Box<Node>
}

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    fn push(&mut self, num: i32) {
        let new_node = Link::More(Box::new(Node { elem: num, next: Link::Empty }));
        let mut last_node: Box<Node> = self.head;
        loop {
            match std::mem::replace(&mut last_node.next, Link::Empty) {
                Link::Empty => break,
                Link::More(mut node) => { last_node = node; }
            }
        }
        std::mem::replace(&mut last_node.next, new_node);
    }
}


fn main() {
    let list: List = List {
        head: Box::new(Node {
            elem: 0,
            next: Link::Empty
        })
    };
}
