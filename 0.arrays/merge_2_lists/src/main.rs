// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
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
struct Solution {}

impl Solution {
    pub fn merge_two_lists_rec(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (None, Some(l2)) => Some(l2),
            (Some(l1), None) => Some(l1),
            (Some(l1), Some(l2)) => {
                if l1.val <= l2.val {
                    Some(Box::new(ListNode {
                        val: l1.val,
                        next: Solution::merge_two_lists_rec(l1.next, Some(l2)),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: l2.val,
                        next: Solution::merge_two_lists_rec(Some(l1), l2.next),
                    }))
                }
            }
        }
    }

    pub fn merge_two_lists_iter(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(-1));
        let mut cur_node = &mut dummy;

        let (mut l1, mut l2) = (&l1, &l2);

        loop {
            if l1.is_none() {
                cur_node.next = l2.clone();
                break;
            }
            if l2.is_none() {
                cur_node.next = l1.clone();
                break;
            }
            if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
                cur_node.next = l1.clone();
                l1 = &l1.as_ref().unwrap().next;
            } else {
                cur_node.next = l2.clone();
                l2 = &l2.as_ref().unwrap().next;
            }
            cur_node = cur_node.next.as_mut().unwrap();
        }
        dummy.next
    }
}

pub fn vec_to_list(v: &Vec<i32>) -> Box<ListNode> {
    let mut head = Box::new(ListNode::new(v[0]));
    let mut cur_node = &mut head;
    for node in v[1..].iter() {
        let new_node = Box::new(ListNode::new(*node));
        cur_node.next = Some(new_node);
        cur_node = cur_node.next.as_mut().unwrap();
    }
    head
}

pub fn main() {
    let l1 = vec_to_list(&vec![1, 2, 4]);
    let l2 = vec_to_list(&vec![1, 3, 4]);

    let l3 = Solution::merge_two_lists_iter(Some(l1), Some(l2));
    println!("{:?}", l3);
}
