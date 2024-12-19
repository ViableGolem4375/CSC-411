Matthew Langton

I completed this assignment almost entirely on my own with a bit of help on some aspects from the 
professor and TAs.

Design Checklist:

What is the abstract thing you are trying to represent?

 - I am trying to represent a two dimensional list of items all of the same type that has the items arranged into a grid-like order with rows and columns.

What functions will you offer, and what are the contracts that those functions must meet?

 - I will be using these functions:

pub fn from_column_major(elements: &[T], height: usize, width: usize) -> Result<Self, Error> {
    // Creates a new Array2 in column major order. 
    // Arguments:
    //
    // * `width`: the width of the `Array2`.
    // * `height`: the height of the `Array2`.
    // Returns an Array2.
    // Returns an error if the dimensions don't match.
}

pub fn from_row_major(elements: &[T], height: usize, width: usize) -> Result<Self, Error> {
    // Creates a new Array2 in row major order. 
    // Arguments:
    //
    // * `width`: the width of the `Array2`.
    // * `height`: the height of the `Array2`.
    // Returns an Array2.
    // Returns an error if the dimensions don't match.
}

pub fn num_rows(&self) -> usize {
    // Arguments: 
    //
    // &self: the array being used.
    // Returns the number of rows in the Array2.
}

pub fn num_columns(&self) -> usize {
    // Arguments: 
    //
    // &self: the array being used.
    // Returns the number of columns in the Array2.
}

pub fn num_elements(&self) -> usize {
    // Arguments: 
    //
    // &self: the array being used.
    // Returns the total number of elements in the Array2.
}

pub fn row_len(&self) -> usize {
    // Arguments: 
    //
    // &self: the array being used.
    // Returns the number of elements in each row.
}

pub fn column_len(&self) -> usize {
    // Arguments: 
    //
    // &self: the array being used.
    // Returns the number of elements in each column.
}

pub fn get(&self, row: usize, column: usize) -> Option<&T> {
    // Finds a specific element within the Array2.
    // Arguments: 
    //
    // &self: the array being used.
    // * `width`: the width of the `Array2`.
    // * `height`: the height of the `Array2`.
    // Returns the element that was searched for or none if its out of bounds.
}

pub fn get_row_major(&self, index: usize) -> Option<&T> {
    // Finds a specific element within the Array2 in row major order.
    // Arguments: 
    //
    // &self: the array being used.
    // * `index`: the index of the element in the array.
    // Returns the element that was searched for or none if its out of bounds.
}

pub fn get_column_major(&self, index: usize) -> Option<&T> {
    // Finds a specific element within the Array2.
    // Arguments: 
    //
    // &self: the array being used.
    // * `index`: the index of the element in the array.
    // Returns the element that was searched for or none if its out of bounds.
}

pub fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut T> {
    // Finds a specific element within the Array2.
    // Arguments: 
    //
    // &mut self: the array being used.
    // * `row`: the specified row within the Array2.
    // * `column`: the specified column within the Array2.
    // Returns a mutable reference to the element that was searched for or none if its out of bounds.
}

pub fn get_mut_row_major(&mut self, row: usize, column: usize) -> Option<&mut T> {
    // Finds a specific element within the Array2 in row major order.
    // Arguments: 
    //
    // &mut self: the array being used.
    // * `row`: the specified row within the Array2.
    // * `column`: the specified column within the Array2.
    // Returns a mutable reference to the element that was searched for or none if its out of bounds.
}

pub fn get_mut_column_major(&mut self, row: usize, column: usize) -> Option<&mut T> {
    // Finds a specific element within the Array2 in column major order.
    // Arguments: 
    //
    // &mut self: the array being used.
    // * `row`: the specified row within the Array2.
    // * `column`: the specified column within the Array2.
    // Returns a mutable reference to the element that was searched for or none if its out of bounds.
}

pub fn set(&mut self, row: usize, column: usize, element: T) -> Result<(), Error> {
    // Sets a specified element in the Array2 to a new value.
    // Arguments: 
    //
    // &mut self: the array being used.
    // * `row`: the specified row within the Array2.
    // * `column`: the specified column within the Array2.
    // * `element`: the element being changed.
    // Will return Success if it was successful, and will return an error if it was not.
}

pub fn set_row_major(&mut self, row: usize, column: usize, element: T) -> Result<(), Error> {
    // Sets a specified element in the Array2 to a new value in row major order.
    // Arguments: 
    //
    // &mut self: the array being used.
    // * `row`: the specified row within the Array2.
    // * `column`: the specified column within the Array2.
    // * `element`: the element being changed.
    // Will return Success if it was successful, and will return an error if it was not.
}

pub fn set_column_major(&mut self, row: usize, column: usize, element: T) -> Result<(), Error> {
    // Sets a specified element in the Array2 to a new value in column major order.
    // Arguments: 
    //
    // &mut self: the array being used.
    // * `row`: the specified row within the Array2.
    // * `column`: the specified column within the Array2.
    // * `element`: the element being changed.
    // Will return Success if it was successful, and will return an error if it was not.
}

pub fn row_iter(&self, row_index: usize) -> Result<impl DoubleEndedIterator<Item = &T> + Clone, Error> {
    // Iterates over the elements in a given row.
    // Arguments: 
    //
    // &self: the array being used.
    // * `row_index`: the row being iterated over.
    // Returns an iterator over the elements in a given row, or an error if it was not successful.
}

pub fn column_iter(&self, column_index: usize) -> Result<impl DoubleEndedIterator<Item = &T> + Clone, Error> {
    // Iterates over the elements in a given column.
    // Arguments: 
    //
    // &self: the array being used.
    // * `column_index`: the column being iterated over.
    // Returns an iterator over the elements in a given column, or an error if it was not successful.
}

pub fn elements_row_major_iter(&self) -> impl DoubleEndedIterator<Item = &T> + Clone {
    // Iterates over the elements in the Array2 in row major order.
    // Arguments: 
    //
    // &self: the array being used.
    // Returns an iterator over the elements in the Array2.
}

pub fn elements_column_major_iter(&self) -> impl DoubleEndedIterator<Item = &T> + Clone {
    // Iterates over the elements in the Array2 in column major order.
    // Arguments: 
    //
    // &self: the array being used.
    // Returns an iterator over the elements in the Array2.
}

pub fn rows_iter(&self) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &T> + Clone> + Clone {
    // Iterates over the elements in all rows.
    // Arguments: 
    //
    // &self: the array being used.
    // Returns an iterator over the elements in all rows.
}

pub fn columns_iter(&self) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &T> + Clone> + Clone {
    // Iterates over the elements in all columns.
    // Arguments: 
    //
    // &self: the array being used.
    // Returns an iterator over the elements in all columns.
}

What representation will you use, and what invariants will it satisfy?

 - I will be using a vector of vectors (Vec<Vec<T>> in Rust format), to represent a two dimensional array in my implementation.
 - This implementation will satisfy these invariants:

  - The vector of vectors is rectangular, i.e., the length of each row is either the same or less than the length of the first row.
  - The vector of vectors has a fixed number of rows, i.e., the length of the outer vector does not change.
  - The vector of vectors has a fixed number of columns, i.e., the length of each inner vector is the same and does not change.
 - The vector of vectors as it is being created will always have less than n elements (where n is the total number of elements in the vectors being used to construct the 2D array) and the iterator will stop running once the number of elements in the 2D array is equal to n.


All parts of this assignment should be properly implemented.

I have spent approximately 9-10 hours completing this assignment including all parts and the design 
document.
