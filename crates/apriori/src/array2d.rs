/// A lower triangle 2D square matrix in the form of a 1D array.
#[derive(Debug, Default)]
pub struct Array2D<T>(Vec<T>);
impl<T: Copy> Array2D<T> {
    /// Gets the element at row and col
    pub fn get(&self, row: usize, col: usize) -> T {
        self.0[self.get_index(row, col)]
    }
}
impl<T: Copy + Default> Array2D<T> {
    /// Constructor with the number of rows
    pub fn new(rows: usize) -> Self {
        Array2D(vec![T::default(); (rows * (rows - 1)) / 2])
    }
}
impl<T> Array2D<T> {
    /// Gets the index into the 1D array based on row and col
    fn get_index(&self, row: usize, col: usize) -> usize {
        assert!(row != col);
        // The row must be greater than column
        let (row, col) = if row > col { (row, col) } else { (col, row) };
        let index = (row * (row - 1)) / 2 + col;
        assert!(index < self.0.len());
        index
    }
    /// Sets value into the 2D array
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        let index = self.get_index(row, col);
        self.0[index] = value;
    }
    /// Iterator over all the element of the 2D array.
    pub fn iter(&self) -> Array2DIterator<T> {
        Array2DIterator::new(self)
    }
}
impl Array2D<u64> {
    /// Increments at row, col
    pub fn increment(&mut self, row: usize, col: usize) {
        let index = self.get_index(row, col);
        self.0[index] += 1;
    }
    /// Adds up the corresponding elements in the 2D Array
    /// Both arrays must have equal sizes.
    pub fn add_assign(&mut self, rhs: &Array2D<u64>) {
        assert!(self.0.len() == rhs.0.len());
        for i in 0..self.0.len() {
            self.0[i] += rhs.0[i];
        }
    }
}
/// The Iterator for the 2D Array
#[derive(Debug)]
pub struct Array2DIterator<'a, T> {
    data: &'a Array2D<T>,
    /// The current row
    row: usize,
    /// The current column
    col: usize,
    /// The current index
    idx: usize,
}

impl<'a, T> Array2DIterator<'a, T> {
    /// Constructor
    fn new(data: &'a Array2D<T>) -> Self {
        Self {
            data,
            row: 1,
            col: 0,
            idx: 0,
        }
    }
}
impl<T: Copy> Iterator for Array2DIterator<'_, T> {
    type Item = (usize, usize, T);
    fn next(&mut self) -> Option<Self::Item> {
        // Iterated over everything
        if self.idx >= self.data.0.len() {
            return None;
        }
        // Gets the element at the current position
        let element = (self.row, self.col, self.data.0[self.idx]);
        // Increments the position
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
