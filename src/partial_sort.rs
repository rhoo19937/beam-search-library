use core::cmp::Ordering::{self,*};
use core::mem::ManuallyDrop;
use core::ptr;


struct Hole<'a, T: 'a> {
    data: &'a mut [T],
    elt: ManuallyDrop<T>,
    pos: usize,
}


impl<'a, T> Hole<'a, T> {
    #[inline]
    unsafe fn new(data: &'a mut [T], pos: usize) -> Self {
        let elt = unsafe { ptr::read(data.get_unchecked(pos)) };
        Hole { data, elt: ManuallyDrop::new(elt), pos }
    }

    #[inline]
    fn element(&self) -> &T {
        &self.elt
    }

    #[inline]
    unsafe fn get(&self, index: usize) -> &T {
        unsafe { self.data.get_unchecked(index) }
    }

    unsafe fn move_to(&mut self, index: usize) {
        unsafe {
            let ptr = self.data.as_mut_ptr();
            let index_ptr: *const _ = ptr.add(index);
            let hole_ptr = ptr.add(self.pos);
            ptr::copy_nonoverlapping(index_ptr, hole_ptr, 1);
        }
        self.pos = index;
    }
}


impl<T> Drop for Hole<'_, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let pos = self.pos;
            ptr::copy_nonoverlapping(&*self.elt, self.data.get_unchecked_mut(pos), 1);
        }
    }
}


unsafe fn sift_up<T, F: FnMut(&T, &T) -> Ordering>(slice:&mut [T], start: usize, pos: usize, mut compare: F) -> usize {
    let mut hole = unsafe { Hole::new(slice, pos) };

    while hole.pos > start {
        let parent = (hole.pos - 1) / 2;

        if compare(hole.element(), unsafe { hole.get(parent) }) != Greater {
            break;
        }

        unsafe { hole.move_to(parent) };
    }

    hole.pos
}


unsafe fn sift_down_range<T, F: FnMut(&T, &T) -> Ordering>(slice:&mut [T], pos: usize, end: usize, mut compare: F) {
    let mut hole = unsafe { Hole::new(slice, pos) };
    let mut child = 2 * hole.pos + 1;

    while child <= end.saturating_sub(2) {
        child += unsafe { compare(hole.get(child), hole.get(child + 1)) != Greater} as usize;

        if compare(unsafe { hole.get(child) }, hole.element()) != Greater{
            return;
        }

        unsafe { hole.move_to(child) };
        child = 2 * hole.pos + 1;
    }

    if child == end - 1 && compare(hole.element(), unsafe { hole.get(child) }) == Less {
        unsafe { hole.move_to(child) };
    }
}


#[inline]
unsafe fn sift_down<T, F: FnMut(&T, &T) -> Ordering>(slice:&mut [T], pos: usize, compare: F) {
    let len = slice.len();
    unsafe { sift_down_range(slice, pos, len, compare) };
}


unsafe fn sift_down_to_bottom<T, F: FnMut(&T, &T) -> Ordering>(slice:&mut [T], mut pos: usize, mut compare: F) {
    let end = slice.len();
    let start = pos;

    let mut hole = unsafe { Hole::new(slice, pos) };
    let mut child = 2 * hole.pos + 1;

    while child <= end.saturating_sub(2) {
        child += unsafe { compare(hole.get(child), hole.get(child + 1)) != Greater } as usize;

        unsafe { hole.move_to(child) };
        child = 2 * hole.pos + 1;
    }

    if child == end - 1 {
        unsafe { hole.move_to(child) };
    }
    pos = hole.pos;
    drop(hole);

    unsafe { sift_up(slice, start, pos, compare) };
}


pub trait Heap{
    type T;
    fn to_heap<F: FnMut(&Self::T, &Self::T) -> Ordering>(&mut self, compare: F);
    fn to_sort<F: FnMut(&Self::T, &Self::T) -> Ordering>(&mut self, compare: F);
    fn shift_up_heap<F: FnMut(&Self::T, &Self::T) -> Ordering>(&mut self, compare: F);
    fn shift_down_heap<F: FnMut(&Self::T, &Self::T) -> Ordering>(&mut self, compare: F);
}


impl<T> Heap for [T]{
    type T = T;
    fn to_heap<F: FnMut(&T, &T) -> Ordering>(&mut self, mut compare: F){
        let mut n = self.len() / 2;
        while n > 0 {
            n -= 1;
            unsafe { sift_down(self, n, &mut compare) };
        }
    }

    fn to_sort<F: FnMut(&T, &T) -> Ordering>(&mut self, mut compare: F){
        let mut end = self.len();
        while end > 1 {
            end -= 1;
            unsafe {
                let ptr = self.as_mut_ptr();
                ptr::swap(ptr, ptr.add(end));
            }
            unsafe { sift_down_range(self, 0, end, &mut compare) };
        }
    }

    #[inline]
    fn shift_up_heap<F: FnMut(&T, &T) -> Ordering>(&mut self, compare: F){
        assert!(!self.is_empty());
        unsafe{ sift_up(self, 0, self.len() - 1, compare) };
    }

    #[inline]
    fn shift_down_heap<F: FnMut(&T, &T) -> Ordering>(&mut self, compare: F){
        assert!(!self.is_empty());
        unsafe{ sift_down_to_bottom(self, 0, compare) }
    }
}


#[inline]
fn heap_sort<T, F: FnMut(&T, &T) -> Ordering>(slice:&mut [T], mut compare: F){
    slice.to_heap(&mut compare);
    slice.to_sort(&mut compare);
}


fn select_nth_unstable_heap<T, F: FnMut(&T, &T) -> Ordering>(slice:&mut [T], index:usize, mut compare: F){
    assert!(index < slice.len());
    slice[..index].to_heap(&mut compare);
    for i in index..slice.len(){
        if compare(&slice[0], &slice[i]) == Greater{
            slice.swap(0, i);
            slice[..index].shift_down_heap(&mut compare);
        }
    }

    // slice::select_nth_unstableと同じようにスコアがだいたい昇順になるようreverseする
    slice[..index].reverse();
}


fn partial_sort<T, F: FnMut(&T, &T) -> Ordering>(slice:&mut [T], index:usize, mut compare: F){
    assert!(index <= slice.len());
    slice[..index].to_heap(&mut compare);
    for i in index..slice.len(){
        if compare(&slice[0], &slice[i]) == Greater{
            slice.swap(0, i);
            slice[..index].shift_down_heap(&mut compare);
        }
    }

    slice[..index].to_sort(&mut compare);
}


pub trait HeapSort{
    type T;
    fn heap_sort(&mut self) where Self::T: Ord;
    fn heap_sort_by<F: FnMut(&Self::T, &Self::T) -> Ordering>(&mut self, compare: F);
    fn heap_sort_by_key<K: Ord, F: FnMut(&Self::T) -> K>(&mut self, f: F);
    fn select_nth_unstable_heap(&mut self, index: usize) where Self::T: Ord;
    fn select_nth_unstable_heap_by<F: FnMut(&Self::T, &Self::T) -> Ordering>(&mut self, index: usize, compare: F);
    fn select_nth_unstable_heap_by_key<K: Ord, F: FnMut(&Self::T) -> K>(&mut self, index: usize, f: F);
    fn partial_sort(&mut self, index: usize) where Self::T: Ord;
    fn partial_sort_by<F: FnMut(&Self::T, &Self::T) -> Ordering>(&mut self, index: usize, compare: F);
    fn partial_sort_by_key<K: Ord, F: FnMut(&Self::T) -> K>(&mut self, index: usize, f: F);
}


impl<T> HeapSort for [T]{
    type T = T;

    #[inline]
    fn heap_sort(&mut self) where T: Ord{
        heap_sort(self, T::cmp)
    }

    #[inline]
    fn heap_sort_by<F: FnMut(&T, &T) -> Ordering>(&mut self, compare: F){
        heap_sort(self, compare)
    }

    #[inline]
    fn heap_sort_by_key<K: Ord, F: FnMut(&T) -> K>(&mut self, mut f: F){
        heap_sort(self, |a, b| f(a).cmp(&f(b)));
    }

    #[inline]
    fn select_nth_unstable_heap(&mut self, index: usize) where T: Ord{
        select_nth_unstable_heap(self, index, T::cmp);
    }

    #[inline]
    fn select_nth_unstable_heap_by<F: FnMut(&T, &T) -> Ordering>(&mut self, index: usize, compare: F){
        select_nth_unstable_heap(self, index, compare);
    }

    #[inline]
    fn select_nth_unstable_heap_by_key<K: Ord, F: FnMut(&T) -> K>(&mut self, index: usize, mut f: F){
        select_nth_unstable_heap(self, index, |a, b| f(a).cmp(&f(b)));
    }

    #[inline]
    fn partial_sort(&mut self, index: usize) where T: Ord{
        partial_sort(self, index, T::cmp);
    }

    #[inline]
    fn partial_sort_by<F: FnMut(&T, &T) -> Ordering>(&mut self, index: usize, compare: F){
        partial_sort(self, index, compare);
    }

    #[inline]
    fn partial_sort_by_key<K: Ord, F: FnMut(&T) -> K>(&mut self, index: usize, mut f: F){
        partial_sort(self, index, |a, b| f(a).cmp(&f(b)));
    }
}