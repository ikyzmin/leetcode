fn main() {
    println!("{:?}",delete_duplicates());
}

 #[derive(PartialEq, Eq, Clone, Debug)]
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

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none(){
        return None;
    }
    let mut temp_head = head;
    let mut first = temp_head.as_mut().unwrap();
    while first.next.is_some() {
        if let Some(second) = &mut first.next {
            if first.val == second.val {
                first.next = second.next.take()
            } else {
                first = first.next.as_mut().unwrap();
            }
        }
    }
    temp_head
}

