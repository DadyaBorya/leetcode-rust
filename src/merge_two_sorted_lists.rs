#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
    pub fn from_vec(vec: &[i32]) -> Option<Box<ListNode>> {
        let mut result = None;
        for entry in vec.iter().rev() {
            let mut node = Self::new(*entry);
            node.next = result;
            result = Some(Box::new(node))
        }

        result
    }
}

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        return match (list1, list2) {
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (None, None) => None,
            (Some(l), Some(r)) => {
                if l.val <= r.val {
                    Some(Box::new(ListNode { next: Solution::merge_two_lists(l.next, Some(r)), val: l.val }))
                } else {
                    Some(Box::new(ListNode { next: Solution::merge_two_lists(Some(l), r.next), val: r.val }))
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::merge_two_lists(ListNode::from_vec(&vec![1, 2, 4]),
                                      ListNode::from_vec(&vec![1, 3, 4])),
            ListNode::from_vec(&vec![1, 1, 2, 3, 4, 4])
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::merge_two_lists(ListNode::from_vec(&vec![]),
                                      ListNode::from_vec(&vec![])),
            ListNode::from_vec(&vec![])
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::merge_two_lists(ListNode::from_vec(&vec![]),
                                      ListNode::from_vec(&vec![0])),
            ListNode::from_vec(&vec![0])
        );
    }
}

