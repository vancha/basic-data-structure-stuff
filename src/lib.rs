#[derive(Debug)]
pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new(s:Option<Vec<T>>)->Self {
        match s {
            Some(x)=>{Stack { stack: x }},
            _ =>{Stack { stack: vec![]}},
        }
    }
    
    pub fn push(&mut self, element: T) {
        self.stack.push(element);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
    pub fn isEmpty(&self) -> bool {
        false
    }
    pub fn top(&self) -> Option<&T> {
        self.stack.get(0)
    }
}

struct Node<T>{
    data:T,
    next:Option<Box::<Node<T>>>,
}

pub struct LinkedList<T> {
    head:Option<Node<T>>,
}
/*
impl LinkedList {
    fn insertAtEnd(&mut self){}
    fn insertAtHead(&mut self ){}
    fn delete(&mut self){}
    fn deleteAtHead(&mut self){}
    fn search(&self,elem:T){}
    fn isEmpty(&self)->bool{}
}
*/
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stack_add_works() {
        let mut s = Stack::new();
        s.push(6);
        assert_eq!(s.pop(),Some(6))
    }
}
