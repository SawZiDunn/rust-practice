trait Stack {
    type Item;
    fn push(&mut self, item: Self::Item);
    fn pop(&mut self) -> Option<Self::Item>;
    fn peek(&self) -> Option<&Self::Item>;
}

struct VecStack<T> {
    items: Vec<T>,
}

impl<T> Stack for VecStack<T> {
    type Item = T;

    fn push(&mut self, item: Self::Item) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&Self::Item> {
        self.items.last()
    }
}

fn main() {
    let mut stack = VecStack { items: Vec::new() };
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("Top item: {:?}", stack.peek());
    println!("Popped item: {:?}", stack.pop());
    println!("New top item: {:?}", stack.peek());
}
