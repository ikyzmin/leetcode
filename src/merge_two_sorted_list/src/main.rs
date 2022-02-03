fn main() {
    let mut list1 = None;
    let mut list2 = Some(Box::new(ListNode::new(1)));
    println!("{:?}", merge_two_lists(list1, list2));
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut first = list1;
    let mut second = list2;
    let mut result_head = None;
    let mut result_node = &mut result_head;
    //println!("{:?} {:?}", first, second);
    while first != None || second != None {
        if let Some(ref mut first_node) = first {
            if let Some(ref mut second_node) = second {
                if first_node.val < second_node.val {
                    *result_node = Some(Box::new(ListNode::new(first_node.val)));
                    first = first_node.next.take();
                } else {
                    *result_node = Some(Box::new(ListNode::new(second_node.val)));
                    second = second_node.next.take();
                }
            } else {
                *result_node = Some(Box::new(ListNode::new(first_node.val)));
                first = first_node.next.take();
            }
        } else {
            if let Some(ref mut second_node) = second {
                *result_node = Some(Box::new(ListNode::new(second_node.val)));
                second = second_node.next.take();
            }
        }

        //println!("{:?} {:?} {:?}", first, second, result_node);
        if let Some(node) = result_node {
            result_node = &mut node.next;
        }
    }
    result_head
}
