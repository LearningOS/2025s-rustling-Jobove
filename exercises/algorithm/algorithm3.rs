/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T>(array: &mut [T])
where T: PartialOrd {
    if array.len() < 2 {
        return;
    }

    let mut smaller_ptr = 0;
    for i in 1..array.len() {
        if array[i] < array[0] {
            smaller_ptr += 1;
            array.swap(i, smaller_ptr);
        }
    }
    array.swap(0, smaller_ptr);
    sort(&mut array[1..=smaller_ptr]);
    sort(&mut array[smaller_ptr + 1..]);
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