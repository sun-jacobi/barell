#![no_std]
#![feature(core_intrinsics)]

use core::ptr::NonNull;

#[derive(Default)]
pub struct Node {
    next: Option<NonNull<Node>>,
}

#[derive(Default)]
pub struct BareLL {
    head: Option<NonNull<Node>>,
}

impl BareLL {
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn push(&mut self, mut node: NonNull<Node>) {
        if let Some(head) = self.head {
            unsafe {
                node.as_mut().next = Some(head);
            }
        }

        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<NonNull<Node>> {
        if let Some(head) = self.head {
            self.head = unsafe { head.as_ref().next };

            return Some(head);
        }
        None
    }
}

impl Node {
    pub fn new() -> Self {
        Self { next: None }
    }

    pub fn is_empty(&self) -> bool {
        self.next.is_none()
    }

    pub fn from_addr(addr: u64) -> NonNull<Node> {
        let ptr = addr as *mut Node;
        unsafe { NonNull::new_unchecked(ptr) }
    }
}

#[cfg(test)]
mod test {

    use core::intrinsics::size_of;

    use crate::{BareLL, Node};

    #[test]
    fn basic_test() {
        let mut bare_ll = BareLL::default();
        let pool = [0u64; 4096];
        let addr0 = pool.as_ptr() as u64;

        bare_ll.push(Node::from_addr(addr0));

        let addr1 = addr0 + size_of::<Node>() as u64;

        bare_ll.push(Node::from_addr(addr1));

        assert_eq!(bare_ll.pop(), Some(Node::from_addr(addr1)));
        assert_eq!(bare_ll.pop(), Some(Node::from_addr(addr0)));

        assert!(bare_ll.is_empty());
    }
}
