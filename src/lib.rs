use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None,
    }
  }

  pub fn to_sub_tree(node: TreeNode) -> Option<Rc<RefCell<Self>>> {
    Some(Rc::new(RefCell::new(node)))
  }

  pub fn new_left_right(val: i32, left: i32, right: i32) -> Self {
    TreeNode {
      val,
      left: Self::to_sub_tree(Self::new(left)),
      right: Self::to_sub_tree(Self::new(right)),
    }
  }

  pub fn new_left(val: i32, left: i32) -> Self {
    TreeNode {
      val,
      left: Self::to_sub_tree(Self::new(left)),
      right: None,
    }
  }

  pub fn new_right(val: i32, right: i32) -> Self {
    let right = Self::new(right);
    TreeNode {
      val,
      left: None,
      right: Some(Rc::new(RefCell::new(right))),
    }
  }
}

pub struct Solution {}
impl Solution {
  /* takeaways
     - first the tree is a binary search tree, which
       means left child < root < right child in node
       value
     - visit the tree inorder to produce a list
       of node values in ascending order, and start
       inserting them to the queue up to k elements
       while visting the tree. Once the queue reach
       k elements we start updating the queue and
       maintain the number of elements in the queue
       at any given time is k.

     - how we adjust the queue is the key to understand
       the whole design. Let's use test_fixture_one as
       an example to explain.
       - let say queue has been filled up to 2 elements
         [1,2]
       - next comes 3. 3 is closer to 3.714286
         than 1 is, which is the smallest and at the
         very front of the queue.
      - We kick 1 out from the queue as the target is closer
        to a larger number in this case 3. so 1 stands no chance
        to win when comparing to 2.
      - so the queue becomes [2,3]; 2 is fighting for its
        existence as the target already is closer to 3.
      - next comes 4, which is closer to the target than 2 is.
      - we kick 2 out and add 4 to the queue for the same
        reason

  */

  /* notes on how Rust dealing with tree data structures
     - if you design the tree in a traditional way in Rust
       you are in for a lot of surprises
     - this is especially true when you visit the tree
       iteratively
     - as simple as declare variable to point at
       the node you are currently visiting becomes
       a confusing and challenging process due to
       the ownership
     - implement arena-allocated tress as an
       alternative if possible
  */

  pub fn closest_k_values(root: &Option<Rc<RefCell<TreeNode>>>, target: f64, k: usize) -> Vec<i32> {
    let mut k_values: VecDeque<i32> = VecDeque::new();

    let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];

    /*
      - root is of Option type; just borrow the content
        of the it using as_ref() as root is behind a
        shared reference and can can't be moved into
        Rc::clone
      - we then create a new shared reference to root
        using Rc::clone
        - why? so we don't run into all kinds of
          ownership issues later
      - we can use unwrap right away as it's assumed
        that the tree is not empty
    */
    let mut node_rc = Rc::clone(root.as_ref().unwrap());
    let mut has_node = true;

    while stack.len() > 0 || has_node {
      /* visiting the left child */
      while has_node {
        /*
          - we add a new pointer to the
            stack, which points to the
            same node the node_rc is
            pointing to, to bypass
            ownership moved issue
        */
        stack.push(Rc::clone(&node_rc));

        /*
          - we need to borrow from a copy
            of node_rc instead of borrowing
            from node_rc directly
          - why? if we borrow from node_rc
            directly we can't later reassign
            it to the left child as it's
            been borrowed and can't be
            re-assigned
        */
        let copied = Rc::clone(&node_rc);
        let node = copied.borrow();

        /* we have right child */
        if let Some(left_rc) = &node.left {
          node_rc = Rc::clone(left_rc);
        } else {
          has_node = false
        }
      }

      node_rc = stack.pop().unwrap();
      /*
        - we are done borrowing once
          we have the node value
        - the node value is an i32
          which implements Copy trait
          so we are fine here; we
          don't need to create a
          copy for node_rc
      */
      let node_val = node_rc.borrow().val;

      /* the key
        - fill up the queue up to k elements
        - maintain the queue
      */
      if k_values.len() < k {
        k_values.push_back(node_val);
      } else {
        let smallest = *k_values.get(0).unwrap();
        if f64::abs(smallest as f64 - target) > f64::abs(node_val as f64 - target) {
          //bye bye little one
          k_values.pop_front();
          // welcome big buy!
          k_values.push_back(node_val);
        } else {
          /*
             - the next one will be bigger than
               the big guy who has lost
               already; no point to continue
               we are done here.
          */
          break;
        }
      }

      let copied = Rc::clone(&node_rc);
      let node = copied.borrow();
      /* we have right child; visit it */
      if let Some(right_rc) = &node.right {
        node_rc = Rc::clone(right_rc);
        has_node = true;
      } else {
        has_node = false
      }
    }

    /*
      - use copied to convert &i32 to i32
    */
    k_values.iter().copied().collect()
  }
}

pub struct TestFixtures {}

impl TestFixtures {
  pub fn test_fixture_1() -> Option<Rc<RefCell<TreeNode>>> {
    let left = TreeNode::new_left_right(2, 1, 3);
    let right = TreeNode::new(5);
    let mut root = TreeNode::new(4);

    root.left = TreeNode::to_sub_tree(left);
    root.right = TreeNode::to_sub_tree(right);

    TreeNode::to_sub_tree(root)
  }
  pub fn test_fixture_2() -> Option<Rc<RefCell<TreeNode>>> {
    let root = TreeNode::new(1);
    TreeNode::to_sub_tree(root)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_1() {
    /*
       Input: root = [4,2,5,1,3], target = 3.714286, k = 2
       Output: [4,3]
    */

    let result = Solution::closest_k_values(&TestFixtures::test_fixture_1(), 3.714286, 2);
    assert_eq!(result, vec![3, 4]);
  }

  #[test]
  fn sample_2() {
    /*
        Input: root = [1], target = 0.000000, k = 1
        Output: [1]
    */
    let result = Solution::closest_k_values(&TestFixtures::test_fixture_2(), 0.000000, 1);
    assert_eq!(result, vec![1]);
  }
}
