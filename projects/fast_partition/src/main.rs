struct Node<T>{
    elem: T,
    prev: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

fn main() {
    // let n = 5;
    // let mut nodes = Vec::<Node<i32>>::new();
    // for i in 1..=n{
    //     nodes.push(Node{elem:i, prev:None});
    // }
    // while nodes.len()>0 {
    //     match nodes.last() {
    //         Some(node) => {
    //             match &node.prev{
    //                 Some() => 
    //                 None => 
    //             }
    //         },
    //         None => return,
    //     }
    // }
}
