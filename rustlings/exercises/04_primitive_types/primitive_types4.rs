fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];
        let mut nice_slice:Vec<u16>=vec![];
        for (key, val) in a.iter().enumerate(){
            if key!=0 && key!=4{
                nice_slice.push(*val);
            }
        }
    assert_eq!(vec![2, 3, 4], nice_slice);
    //  assert_eq!([2, 3, 4], nice_slice);
    }

        fn slice_out_of_array2() {
          let a = [1, 2, 3, 4, 5];
    let nice_slice = &a[1..4];
    assert_eq!(&[2, 3, 4], nice_slice);
    }
}
