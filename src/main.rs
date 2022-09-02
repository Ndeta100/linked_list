fn main() {
    let x = vec![1, 3, 4, 5, 6];
    let node = Node {
        elem: 9,
        next: Some(Box::new(Node {
            elem: 3,
            next: None,
        })),
    };
}

type Link<T> = Option<Box<Node<T>>>;
struct Node<T> {
    elem: T,
    next: Link<T>,
}
pub struct LinkedList<T> {
    head: Link<T>,
}

pub struct IterMut<'a, T: 'a>(Option<&'a mut Node<T>>);
impl<T> LinkedList<T> {
    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut(self.head.as_mut().map(|node| &mut **node))
    }
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            self.0 = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}
