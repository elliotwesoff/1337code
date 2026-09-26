#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(mut n1), Some(mut n2)) => {
            if n1.val <= n2.val {
                n1.next = merge_two_lists(n1.next, Some(n2));
                Some(n1)
            } else {
                n2.next = merge_two_lists(Some(n1), n2.next);
                Some(n2)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::solution_21::{ListNode, merge_two_lists};

    // Helper function to turn a Vector into a Linked List
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
    fn test_merge() {
        let l1 = to_list(vec![1, 2, 4]);
        let l2 = to_list(vec![1, 3, 4]);
        let expected = to_list(vec![1, 1, 2, 3, 4, 4]);

        assert_eq!(merge_two_lists(l1, l2), expected);
    }

    #[test]
    fn test_empty() {
        assert_eq!(merge_two_lists(None, None), None);
    }
}
