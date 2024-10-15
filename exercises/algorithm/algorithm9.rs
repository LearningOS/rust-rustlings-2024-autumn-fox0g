/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        
        //TODO
        self.count += 1;
        self.items.push(value);
        
        if self.count >1 {
  
            // 本就有元素
            // 调整堆
            // 开始调整位置
            let init_idx = self.count / 2;
            for init_idx in (1..(self.count / 2)+1).rev() {
                let l_c_i = self.left_child_idx(init_idx);
                let r_c_i = self.right_child_idx(init_idx);
                let mut comp_c_i = l_c_i;
                if r_c_i <= self.count {
                    match (self.comparator)(&self.items[l_c_i], &self.items[r_c_i]) {
                        true => comp_c_i = l_c_i,
                        false => comp_c_i = r_c_i,
                    }
                }
                match (self.comparator)(&self.items[init_idx], &self.items[comp_c_i]) {
                    true => println!("no"),
                    false => {
                        self.items.swap(init_idx, comp_c_i);                    
                    },
                }
            }
    
        }
   
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		0
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        // if self.count > 0 {
        //     return Some(self.items)
        // }
        if self.count == 0 {
            return None;
        }else {
            self.count -= 1;
            return Some(self.items.remove(1));
        }
		
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new::<i32>();
        // heap.items.push(0);
        heap.add(4); 
        heap.add(2);
        heap.add(9);
        heap.add(11);
        
        assert_eq!(heap.len(), 4);
        println!("{:?}",heap.items);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        println!("+-*/{}",heap.count);
        heap.add(1);
        println!("+-*/{}",heap.count);
        println!("{:?}",heap.items);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();

        heap.add(4);
        heap.add(2);
        println!("{:?}",heap.items);
        heap.add(9);
        println!("{:?}",heap.items);
        heap.add(11);
        println!("{:?}",heap.items);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}