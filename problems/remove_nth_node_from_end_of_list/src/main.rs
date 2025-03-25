// Definition for singly-linked list
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

fn main() {}

fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    // Push all the nodes in a stack, while figuring out the size of the list
    let mut current_node: Option<&Box<ListNode>> = head.as_ref();
    let mut stack: Vec<i32> = vec![];
    while let Some(node) = current_node {
        stack.push(node.val);
        current_node = node.next.as_ref();
    }

    // Pop the list from the stack up until the node before the nth node
    let mut next_node: Option<Box<ListNode>> = None;
    for i in 0..stack.len() {
        if i != (n as usize) - 1 {
            let mut new_node = Box::new(ListNode::new(stack.pop().unwrap()));
            new_node.next = next_node;
            next_node = Some(new_node);
        } else {
            // Skip nth element
            stack.pop();
        }
    }

    next_node
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list_from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in vec.iter().rev() {
            let new_node = Box::new(ListNode { val, next: head });
            head = Some(new_node);
        }
        head
    }

    fn vec_from_list(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::new();
        while let Some(node) = head {
            vec.push(node.val);
            head = node.next;
        }
        vec
    }

    #[test]
    fn test_case_1() {
        let head = list_from_vec(vec![1, 2, 3, 4, 5]);
        let n = 2;
        let expected = vec![1, 2, 3, 5];
        assert_eq!(vec_from_list(remove_nth_from_end(head, n)), expected);
    }

    #[test]
    fn test_case_2() {
        let head = list_from_vec(vec![1]);
        let n = 1;
        let expected: Vec<i32> = vec![];
        assert_eq!(vec_from_list(remove_nth_from_end(head, n)), expected);
    }

    #[test]
    fn test_case_3() {
        let head = list_from_vec(vec![1, 2]);
        let n = 1;
        let expected = vec![1];
        assert_eq!(vec_from_list(remove_nth_from_end(head, n)), expected);
    }
}

