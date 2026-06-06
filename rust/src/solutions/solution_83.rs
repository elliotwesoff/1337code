use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut hs = HashSet::new();
    let mut current = &mut head;
    
    while let Some(node) = current {
        if hs.contains(&node.val) {
            *current = node.next.take();
        } else {
            hs.insert(node.val);
            current = &mut current.as_mut().unwrap().next;
        }
    }

    head
}

#[cfg(test)]
mod tests {
    use crate::solutions::solution_83::{ListNode, delete_duplicates};
    fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &v in vec.iter().rev() {
            let mut node = ListNode::new(v);
            node.next = current;
            current = Some(Box::new(node));
        }
        current
    }
    #[test]
    fn test_delete_duplicates() {
        let l1 = to_list(vec![1,1,2]);
        let expected = to_list(vec![1, 2]);
        assert_eq!(expected, delete_duplicates(l1));
    }
}
