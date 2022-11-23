pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {next: None, val}
    }
}

pub fn to_list(vec: Vec<i32>) -> List {
    let mut current = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }

    current
}

impl Solution {
    pub fn add_two_numbers(l1: List, l2: List) -> List {
        let mut l1 = &l1;
        let mut l2 = &l2;
        let mut sum: i32;
        let result = None;
        //let mut dummy = Some(Box::new(ListNode::new(0)));
        //let mut current = &mut dummy;
        let mut current: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut extra: i32;
        let mut agregar: i32 = 0;

        while l1.is_some() || l2.is_some() {
            sum = 0;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = &node.next;
            }

            if let Some(node) = l2{
                sum += node.val;
                l2 = &node.next;
            }

            if sum >= 10 { 
                extra = 1;
            } else { 
                extra = 0;
            }
            
            if agregar == 1 {
                if sum + extra + agregar < 10{
                    current = Some(Box::new(ListNode::new(sum + extra + agregar)));
                    agregar = 0;
                } else if sum + extra + agregar >= 10 {
                    current = Some(Box::new(ListNode::new(sum % 10 + agregar)));
                    agregar = 1;
                }
            } else {
                if sum + extra < 10{
                    current = Some(Box::new(ListNode::new(sum + extra)));
                    agregar = 0;
                } else if sum + extra >= 10 {
                    current = Some(Box::new(ListNode::new(sum % 10)));
                    agregar = 1;
                } 
            }

            current.unwrap().next = current;
            
            println!("current: {:?}, sum: {}",current, sum);
        }
        
        println!("{:?}", current);

        result
    }
}

type List = Option<Box<ListNode>>;

fn main() {
    assert_eq!(
        Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
        to_list(vec![7, 0, 8])
    );
}
