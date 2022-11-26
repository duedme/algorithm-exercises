/*pub struct Solution;

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
*/

type List = Option<Box<ListNode>>;

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

 fn to_list(vector: Vec<i32>) -> List {
    let mut cur = None;

    for &place in vector.iter().rev() {
        let mut new_node = ListNode::new(place);
        new_node.next = cur;
        cur = Some(Box::new(new_node));
    }

    cur
 }
 pub struct Solution;

 impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut returning_node: ListNode = ListNode::new(0);
        let mut current = &mut returning_node;
        let mut carry = 0;
        let mut l1 = &l1;
        let mut l2 = &l2;
        
        //Aún no contempla fin del ciclo.
        while l1.is_some() || l2.is_some() || carry == 1 {
            //Usar unwrap no contempla cuando se acabe la lista y el resultado sea None.
            let mut all_none = false;
            let save: i32 = match (l1, l2) {
                (None, None) => {
                    all_none = true;
                    carry
                }
                (Some(n1), Some(n2)) => {
                    l1 = &n1.next;
                    l2 = &n2.next;
                    n1.val + n2.val + carry
                },
                (Some(n1), None) => {
                    l1 = &n1.next;
                    n1.val + carry
                },
                (None, Some(n2)) => {
                    l2 = &n2.next;
                    n2.val + carry
                },
            };

            carry = if save > 9 { 1 } else { 0 };
            *current = ListNode::new(save % 10 );

            if l1.is_some() || l2.is_some() || carry != 0 {
                current.next = Some(Box::new(ListNode::new(0)));
                current = current.next.as_mut().unwrap();
            };


            if all_none && carry == 0 { break; };

        }

        Some(Box::new(returning_node))
    }
}

fn main() {
    assert_eq!(
        Solution::add_two_numbers(to_list(vec![2, 2, 2]), to_list(vec![2, 8])),
        to_list(vec![4, 0, 3])
    );
}
