#[derive(Debug, Default)]
pub struct Array2D<T>(Vec<T>);
impl<T: Copy> Array2D<T> {
    pub fn get(&self, row: usize, col: usize) -> T {
        self.0[self.get_index(row, col)]
    }
}
impl<T: Copy + Default> Array2D<T> {
    pub fn new(rows: usize) -> Self {
        Array2D(vec![T::default(); (rows * (rows - 1)) / 2])
    }
}
impl<T> Array2D<T> {
    fn get_index(&self, row: usize, col: usize) -> usize {
        assert!(row != col);
        let (row, col) = if row > col { (row, col) } else { (col, row) };
        let index = (row * (row - 1)) / 2 + col;
        index
    }
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        let index = self.get_index(row, col);
        self.0[index] = value;
    }
    pub fn iter(&self) -> Array2DIterator<T> {
        Array2DIterator::new(self)
    }
}
impl Array2D<u64> {
    pub fn increment(&mut self, row: usize, col: usize) {
        let index = self.get_index(row, col);
        self.0[index] += 1;
    }
}

#[derive(Debug)]
pub struct Array2DIterator<'a, T> {
    data: &'a Array2D<T>,
    row: usize,
    col: usize,
    idx: usize,
}

impl<'a, T> Array2DIterator<'a, T> {
    fn new(data: &'a Array2D<T>) -> Self {
        Self {
            data,
            row: 1,
            col: 0,
            idx: 0,
        }
    }
}
impl<'a, T: Copy> Iterator for Array2DIterator<'a, T> {
    type Item = (usize, usize, T);
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.data.0.len() {
            return None;
        }
        let element = (self.row, self.col, self.data.0[self.idx]);
        self.idx += 1;
        self.col += 1;
        if self.col >= self.row {
            self.col = 0;
            self.row += 1;
        }
        Some(element)
    }
}
#[cfg(test)]
mod tests {
    use crate::array2d::Array2D;

    #[test]
    fn test_array2d() {
        let mut array2d = Array2D::new(3);
        array2d.increment(0, 1);
        assert_eq!(array2d.get(0, 1), 1);
        array2d.increment(1, 2);
        assert_eq!(array2d.get(1, 2), 1);
        let mut array2d = Array2D::new(5);
        array2d.increment(4, 3);
        array2d.increment(4, 3);
        assert_eq!(array2d.get(4, 3), 2);
        let mut array2d = Array2D::new(10);
        let mut count = 0;
        for i in 0..10 {
            for j in 0..i {
                array2d.set(i, j, count);
                count += 1;
            }
        }
        for i in 0..45 {
            assert_eq!(array2d.0[i], i as u64);
        }
        for (i, e) in array2d.iter().enumerate() {
            assert_eq!(e.2, i as u64);
            assert_eq!(array2d.get(e.0, e.1), e.2);
        }
    }
}
