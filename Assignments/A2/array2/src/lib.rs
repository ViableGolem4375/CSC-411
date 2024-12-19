/// Elements contained must implement `Clone`.
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

//2D Array
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Array2<T: Clone> {
    array: Vec<T>,
    width: usize,
    height: usize,
}

//Error handler for 2D Array.
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// The given indices were out of bounds.
    IndicesOutOfBounds(usize, usize),
    /// The given index in row or column major order was out of bounds.
    IndexOutOfBounds(usize),
    /// The dimensions given did not match the elements provided
    DimensionMismatch,
    /// There were not enough elements to fill the array.
    #[allow(dead_code)]
    NotEnoughElements,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IndicesOutOfBounds(row, column) => write!(f, "indices ({row}, {column}) out of bounds"),
            Error::IndexOutOfBounds(index) => write!(f, "index {index} out of bounds"),
            Error::DimensionMismatch => write!(f, "dimension mismatch"),
            Error::NotEnoughElements => write!(f, "not enough elements"),
        }
    }
}

impl std::error::Error for Error {}

    /// Creates a new `Array2`.
    ///
    /// Arguments: 
    ///
    /// * `width`: the width of the `Array2`.
    /// * `height`: the height of the `Array2`.
impl<T: Clone> Array2<T> {
    #[allow(dead_code)]

    /// Creates a new Array2 from a slice of rows, each of which is a
    /// Vec of elements.
    ///
    /// Arguments:
    /// 
    /// * 'elements': The Vec being passed into the function.
    /// Returns an Array2.
    /// Returns an error if the rows are not all the same size.
pub fn from_rows(elements: &[Vec<T>]) -> Result<Self, Error>
where
    T: Clone, {
    let row_len = elements.get(0).map(Vec::len).unwrap_or(0);
    if !elements.iter().all(|row| row.len() == row_len) {
        return Err(Error::DimensionMismatch);
    }
    Ok(Array2 {
        array: flatten(elements),
        height: elements.len(),
        width: row_len,
    })
}    

/// Creates a new Array2 from a slice of columns, each of which
/// contains a Vec of elements.
///
/// Arguments:
/// 
/// * 'elements': The Vec being passed into the function.
/// Returns an Array2.
/// Returns an error if the columns are not all the same size.
#[allow(dead_code)]
pub fn from_columns(elements: &[Vec<T>]) -> Result<Self, Error>
    where
        T: Clone,
    {
        let column_len = elements.get(0).map(Vec::len).unwrap_or(0);
        if !elements.iter().all(|column| column.len() == column_len) {
            return Err(Error::DimensionMismatch);
        }
        let height = column_len;
        let width = elements.len();
        let array = indices_row_major(height, width)
            .map(|(row, column)| elements[column][row].clone())
            .collect();
        Ok(Array2 {
            array,
            height,
            width,
        })
    }

    /// Creates a new Array2 in column major order. 
    /// Arguments:
    ///
    /// * `width`: the width of the `Array2`.
    /// * `height`: the height of the `Array2`.
    /// * 'elements': The Vec being passed into the function.
    /// Returns an Array2.
    /// Returns an error if the dimensions don't match.
#[allow(dead_code)]
pub fn from_column_major(elements: &[T], height: usize, width: usize) -> Result<Self, Error> 
    where
        T: Clone,
    {
        let total_len = height * width;
        if total_len != elements.len() + 1 {
            return Err(Error::DimensionMismatch);
        }
        let indices_row_major =
            (0..height).flat_map(move |row| (0..width).map(move |column| (row, column)));
        let _array: Vec<T> = indices_row_major
            .map(|(row, column)| {
                let index = column * height + row;
                elements[index].clone()
            })
            .collect();
        Ok(Array2 {
            array: elements.to_vec(),
            height,
            width,
        })
    }

    /// Creates a new Array2 in row major order. 
    /// Arguments:
    ///
    /// * `width`: the width of the `Array2`.
    /// * `height`: the height of the `Array2`.
    /// * 'elements': The Vec being passed into the function.
    /// Returns an Array2.
    /// Returns an error if the dimensions don't match.
pub fn from_row_major(elements: &[T], height: usize, width: usize) -> Result<Self, Error> 
    where
    T: Clone,
{
    let total_len = height * width;
    if total_len != elements.len() {
        return Err(Error::DimensionMismatch);
    }
    Ok(Array2 {
        array: elements.to_vec(),
        height,
        width,
    })
}

    /// Arguments: 
    ///
    /// &self: the array being used.
    /// Returns the number of rows in the Array2.
pub fn num_rows(&self) -> usize {
    self.height
}

    /// Arguments: 
    ///
    /// &self: the array being used.
    /// Returns the number of columns in the Array2.
pub fn num_columns(&self) -> usize {
    self.width
}

    /// Arguments: 
    ///
    /// &self: the array being used.
    /// Returns the total number of elements in the Array2.
#[allow(dead_code)]
pub fn num_elements(&self) -> usize {
    self.height * self.width
}

    /// Arguments: 
    ///
    /// &self: the array being used.
    /// Returns the number of elements in each row.
pub fn row_len(&self) -> usize {
    self.num_columns()
}

    /// Arguments: 
    ///
    /// &self: the array being used.
    /// Returns the number of elements in each column.
#[allow(dead_code)]
pub fn column_len(&self) -> usize {
    self.num_rows()
}

    /// Iterate through the array in row major order along with the corresponding indices.
    /// 
    /// Arguments:
    /// 
    /// &self: the array being used.
    /// 
    /// Returns the elements in the array with their indices.
#[allow(dead_code)]
pub fn enumerate_row_major(&self,) -> impl DoubleEndedIterator<Item = ((usize, usize), &T)> + Clone {
    self.indices_row_major().map(move |i| (i, &self[i]))
}

    /// Iterate through the array in column major order along with the corresponding indices.
    /// 
    /// Arguments:
    /// 
    /// &self: the array being used.
    /// 
    /// Returns the elements in the array with their indices.
#[allow(dead_code)]
pub fn enumerate_column_major(&self,) -> impl DoubleEndedIterator<Item = ((usize, usize), &T)> + Clone {
    self.indices_column_major().map(move |i| (i, &self[i]))
}

    /// Finds the index of an element in the Array2.
    /// Arguments:
    /// 
    /// &self: the array being used.
    /// * `row`: the specified row in the `Array2`.
    /// * `column`: the specified column in the `Array2`.
fn get_index(&self, row: usize, column: usize) -> Option<usize> {
    if row < self.num_rows() && column < self.num_columns() {
        Some(row * self.row_len() + column)
    } else {
        None
    }
}

    // Finds a specific element within the Array2.
    /// Arguments: 
    ///
    /// &self: the array being used.
    /// * `width`: the width of the `Array2`.
    /// * `height`: the height of the `Array2`.
    /// Returns the element that was searched for or none if its out of bounds.
pub fn get(&self, row: usize, column: usize) -> Option<&T> {
    self.get_index(row, column).map(|index| &self.array[index])
}

    /// Finds a specific element within the Array2 in row major order.
    /// Arguments: 
    ///
    /// &self: the array being used.
    /// * `index`: the index of the element in the array.
    /// Returns the element that was searched for or none if its out of bounds.
#[allow(dead_code)]
pub fn get_row_major(&self, index: usize) -> Option<&T> {
    self.array.get(index)
}

    /// Finds a specific element within the Array2.
    /// Arguments: 
    ///
    /// &self: the array being used.
    /// * `index`: the index of the element in the array.
    /// Returns the element that was searched for or none if its out of bounds.
#[allow(dead_code)]
pub fn get_column_major(&self, index: usize) -> Option<&T> {
    let column = dbg!(dbg!(index) / self.num_rows());
    let row = dbg!(index % self.num_rows());
    self.get(row, column)
}

    /// Finds a specific element within the Array2.
    /// Arguments: 
    ///
    /// &mut self: the array being used.
    /// * `row`: the specified row within the Array2.
    /// * `column`: the specified column within the Array2.
    /// Returns a mutable reference to the element that was searched for or none if its out of bounds.
pub fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut T> {
    self.get_index(row, column).map(move |index| &mut self.array[index])
}

    /// Finds a specific element within the Array2 in row major order.
    /// Arguments: 
    ///
    /// &mut self: the array being used.
    /// * `index': the index of the element in the array.
    /// Returns a mutable reference to the element that was searched for or none if its out of bounds.
#[allow(dead_code)]
pub fn get_mut_row_major(&mut self, index: usize) -> Option<&mut T> {
    self.array.get_mut(index)
}

    /// Finds a specific element within the Array2 in column major order.
    /// Arguments: 
    ///
    /// &mut self: the array being used.
    /// * `index': the index of the element in the array.
    /// Returns a mutable reference to the element that was searched for or none if its out of bounds.
#[allow(dead_code)]
pub fn get_mut_column_major(&mut self, index: usize) -> Option<&mut T> {
    let column = index / self.num_rows();
    let row = index % self.num_rows();
    self.get_mut(row, column)
}

    /// Sets a specified element in the Array2 to a new value.
    /// Arguments: 
    ///
    /// &mut self: the array being used.
    /// * `row`: the specified row within the Array2.
    /// * `column`: the specified column within the Array2.
    /// * `element`: the element being changed.
    /// Will return Success if it was successful, and will return an error if it was not.
#[allow(dead_code)]
pub fn set(&mut self, row: usize, column: usize, element: T) -> Result<(), Error> {
    self.get_mut(row, column).map(|location| {*location = element;}).ok_or(Error::IndicesOutOfBounds(row, column))
}

    /// Sets a specified element in the Array2 to a new value in row major order.
    /// Arguments: 
    ///
    /// &mut self: the array being used.
    /// * `index': the index of the element in the array.
    /// * `element`: the element being changed.
    /// Will return Success if it was successful, and will return an error if it was not.
#[allow(dead_code)]
pub fn set_row_major(&mut self, index: usize, element: T) -> Result<(), Error> {
    self.get_mut_row_major(index).map(|location| {*location = element;}).ok_or(Error::IndexOutOfBounds(index))
}

    /// Sets a specified element in the Array2 to a new value in column major order.
    /// Arguments: 
    ///
    /// &mut self: the array being used.
    /// * `index': the index of the element in the array.
    /// * `element`: the element being changed.
    /// Will return Success if it was successful, and will return an error if it was not.
#[allow(dead_code)]
pub fn set_column_major(&mut self, index: usize, element: T) -> Result<(), Error> {
    self.get_mut_column_major(index).map(|location| {*location = element;}).ok_or(Error::IndexOutOfBounds(index))
}

    /// Iterates over the elements in a given row.
    /// Arguments: 
    ///
    /// &self: the array being used.
    /// * `row_index`: the row being iterated over.
    /// Returns an iterator over the elements in a given row, or an error if it was not successful.
pub fn row_iter(&self, row_index: usize) -> Result<impl DoubleEndedIterator<Item = &T> + Clone, Error> {
    let start = self.get_index(row_index, 0).ok_or(Error::IndicesOutOfBounds(row_index, 0))?;
    let end = start + self.row_len();
    Ok(self.array[start..end].iter())
}

    /// Iterates over the elements in a given column.
    /// Arguments: 
    ///
    /// &self: the array being used.
    /// * `column_index`: the column being iterated over.
    /// Returns an iterator over the elements in a given column, or an error if it was not successful.
#[allow(dead_code)]
pub fn column_iter(&self, column_index: usize) -> Result<impl DoubleEndedIterator<Item = &T> + Clone, Error> {
    if column_index >= self.num_columns() {
        return Err(Error::IndicesOutOfBounds(0, column_index));
    }
    Ok((0..self.column_len()).map(move |row_index| &self[(row_index, column_index)]))
}

    /// Iterates over the array and finds the indices of the elements in the array in column major order.
    /// Arguments:
    /// 
    /// &self: the array being used.
    /// Returns the indices of the array in column major order.
pub fn indices_column_major(&self) -> impl DoubleEndedIterator<Item = (usize, usize)> + Clone {
    indices_column_major(self.num_rows(), self.num_columns())
}

    /// Iterates over the array and finds the indices of the elements in the array in row major order.
    /// Arguments:
    /// 
    /// &self: the array being used.
    /// Returns the indices of the array in row major order.
pub fn indices_row_major(&self) -> impl DoubleEndedIterator<Item = (usize, usize)> + Clone {
    indices_row_major(self.num_rows(), self.num_columns())
}

    /// Iterates over the elements in the Array2 in row major order.
    /// Arguments: 
    ///
    /// &self: the array being used.
    /// Returns an iterator over the elements in the Array2.
#[allow(dead_code)]
pub fn elements_row_major_iter(&self) -> impl DoubleEndedIterator<Item = &T> + Clone {
    self.array.iter()
}

    /// Iterates over the elements in the Array2 in column major order.
    /// Arguments: 
    ///
    /// &self: the array being used.
    /// Returns an iterator over the elements in the Array2.
#[allow(dead_code)]
pub fn elements_column_major_iter(&self) -> impl DoubleEndedIterator<Item = &T> + Clone {
    self.indices_column_major().map(move |i| &self[i])
}

    /// Iterates over the elements in all rows.
    /// Arguments: 
    ///
    /// &self: the array being used.
    /// Returns an iterator over the elements in all rows.
#[allow(dead_code)]
pub fn rows_iter(&self) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &T> + Clone> + Clone {
    (0..self.num_rows()).map(move |row_index| {
        self.row_iter(row_index)
            .expect("rows_iter should never fail")
    })
}

    /// Iterates over the elements in all columns.
    /// Arguments: 
    ///
    /// &self: the array being used.
    /// Returns an iterator over the elements in all columns.
#[allow(dead_code)]
pub fn columns_iter(&self) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &T> + Clone> + Clone {
    (0..self.num_columns()).map(move |column_index| {
        self.column_iter(column_index)
            .expect("columns_iter should never fail")
    })
}
}

impl<T: std::clone::Clone> Index<(usize, usize)> for Array2<T> {
    type Output = T;
    /// Finds an element at a given row and column.
    /// Arguments:
    /// 
    /// &self: the array being used.
    /// * `row`: the specified row within the Array2.
    /// * `column`: the specified column within the Array2.
    /// Returns the element at the given indices, given as `(row, column)`.
    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        self.get(row, column)
            .unwrap_or_else(|| panic!("Index indices {}, {} out of bounds", row, column))
    }
}

impl<T: std::clone::Clone> IndexMut<(usize, usize)> for Array2<T> {
    /// Finds an element at a given row and column.
    /// Arguments:
    /// 
    /// &self: the array being used.
    /// * `row`: the specified row within the Array2.
    /// * `column`: the specified column within the Array2.
    /// Returns a mutable version of the element at the given indices, given as `(row, column)`.
    fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut Self::Output {
        self.get_mut(row, column)
            .unwrap_or_else(|| panic!("Index mut indices {}, {} out of bounds", row, column))
    }
}

    /// Helper functions for indices-row_major and indices_column_major.
    /// Arguments:
    /// 
    /// * `width`: the width of the `Array2`.
    /// * `height`: the height of the `Array2`.
    /// Functions return an iterator over a flat_map.
fn flatten<T: Clone>(nested: &[Vec<T>]) -> Vec<T> {
    nested.iter().flat_map(|row| row.clone()).collect()
}

fn indices_row_major(
    height: usize,
    width: usize,
) -> impl DoubleEndedIterator<Item = (usize, usize)> + Clone {
    (0..height).flat_map(move |row| (0..width).map(move |column| (row, column)))
}

fn indices_column_major(
    height: usize,
    width: usize,
) -> impl DoubleEndedIterator<Item = (usize, usize)> + Clone {
    (0..width).flat_map(move |column| (0..height).map(move |row| (row, column)))
}
