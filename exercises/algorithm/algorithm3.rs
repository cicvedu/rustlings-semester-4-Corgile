/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
fn quick_sort<T: PartialOrd>(q: &mut [T], l: usize, r: usize) {
    if l >= r {
        return;
    }
    let mut pivot_index = (l + r) / 2;
    let mut i = l;
    let mut j = r;
    while i <= j {
        // Compare elements against the pivot value at pivot_index
        while q[i] < q[pivot_index] {
            i += 1;
            if i == q.len() { break; }
        }
        while q[j] > q[pivot_index] {
            if j == 0 { break; }
            j -= 1;
        }
        if i <= j {
            q.swap(i, j);
            // Update the pivot_index in case we swapped the pivot element
            if i == pivot_index {
                pivot_index = j;
            } else if j == pivot_index {
                pivot_index = i;
            }
            if i < q.len() - 1 { i += 1; }
            if j > 0 { j -= 1; }
        }
    }
    quick_sort(q, l, j);
    quick_sort(q, i, r);
}

fn sort<T: PartialOrd>(array: &mut [T]) {
    quick_sort(array, 0, array.len() - 1);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}