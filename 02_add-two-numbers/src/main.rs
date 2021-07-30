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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1c = l1;
    let mut l2c = l2;
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut current = head.as_mut();
    let mut carry = 0;

    while l1c.is_some() || l2c.is_some() {
        let mut sum = carry;
        if let Some(node) = l1c {
            l1c = node.next;
            sum += node.val;
        }

        if let Some(node) = l2c {
            l2c = node.next;
            sum += node.val;
        }

        carry = sum / 10;

        if let Some(node) = current {
            node.next = Some(Box::new(ListNode::new(sum % 10)));
            current = node.next.as_mut();
        }
    }

    if carry > 0 {
        current.unwrap().next = Some(Box::new(ListNode::new(carry)));
    }

    head.unwrap().next
}

pub fn rec(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    carry: i32,
) -> Option<Box<ListNode>> {
    let mut sum = carry;

    let l1c = if let Some(node) = l1 {
        sum += node.val;
        node.next
    } else {
        None
    };

    let l2c = if let Some(node) = l2 {
        sum += node.val;
        node.next
    } else {
        None
    };

    let mut result = ListNode::new(sum % 10);

    if l1c.is_some() || l2c.is_some() || sum >= 10 {
        result.next = rec(l1c, l2c, sum / 10);
    }

    Some(Box::new(result))
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_iterative() {
        let l1 = ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        };
        let l2 = ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        };
        let result = ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 8, next: None })),
            })),
        };
        assert_eq!(
            add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2))),
            Some(Box::new(result))
        );
    }

    #[test]
    fn test_recursive() {
        let l1 = ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        };
        let l2 = ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        };
        let result = ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 8, next: None })),
            })),
        };
        assert_eq!(
            rec(Some(Box::new(l1)), Some(Box::new(l2)), 0),
            Some(Box::new(result))
        );
    }
}
