#![no_std]
#![feature(strict_provenance)]
use core::ptr::NonNull;

#[derive(Default, Clone, Copy)]
pub struct Node {
    next: Option<NonNull<Node>>,
    prev: Option<NonNull<Node>>,
}

#[derive(Default, Clone, Copy)]
pub struct BareList {
    head: Option<NonNull<Node>>,
}

impl BareList {
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn push(&mut self, mut node: NonNull<Node>) {
        if let Some(mut head) = self.head {
            unsafe {
                node.as_mut().next = Some(head);
                head.as_mut().prev = Some(node);
            }
        }

        self.head = Some(node);
    }

    pub fn head(&self) -> Option<NonNull<Node>> {
        self.head
    }

    pub fn pop(&mut self) -> Option<NonNull<Node>> {
        if let Some(head) = self.head {
            self.head = unsafe { head.as_ref().next };

            return Some(head);
        }
        None
    }

    pub fn remove(&self, addr: u64) -> bool {
        let mut head = self.head;
        while let Some(mut node) = head {
            if node.addr().get() as u64 == addr {
                unsafe {
                    if let Some(mut prev) = node.as_mut().prev {
                        prev.as_mut().next = node.as_mut().next;
                    }

                    if let Some(mut next) = node.as_mut().next {
                        next.as_mut().prev = node.as_mut().prev;
                    }
                }

                return true;
            }

            head = unsafe { node.as_ref().next };
        }

        false
    }

    pub fn contains(&self, addr: u64) -> bool {
        let mut head = self.head;
        while let Some(node) = head {
            if node.addr().get() as u64 == addr {
                return true;
            }

            head = unsafe { node.as_ref().next };
        }

        false
    }
}

impl Node {
    pub fn new() -> Self {
        Self {
            next: None,
            prev: None,
        }
    }

    pub fn from_addr(addr: u64) -> NonNull<Node> {
        let ptr = addr as *mut Node;
        unsafe { NonNull::new_unchecked(ptr) }
    }
}

#[cfg(test)]
mod test {
    use core::mem::size_of;

    use crate::{BareList, Node};

    #[test]
    fn basic_test() {
        let mut bare_ll = BareList::default();
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
