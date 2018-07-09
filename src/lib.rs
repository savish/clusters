/// Functionality to determine the proximity between datapoints
///
/// This crate provides low level traits specifically for distance based
/// clustering algorithms. However, the 'distance' function result need not
/// correspond to the classical definition of the word. It could be a time
/// difference or any other measurable and comparable difference between two
/// data points
pub trait Proximity<Other = Self> {
    type Output: PartialOrd + Copy;

    /// Returns the 'distance' between this datapoint and another
    ///
    /// # Example
    ///
    /// ```rust
    /// use clusters::Proximity;
    ///
    /// // Create a struct
    /// struct Num(i32);
    ///
    /// // Implement this trait
    /// impl Proximity for Num {
    ///     type Output = i32;
    ///
    ///     fn distance(&self, other: &Num) -> i32 {
    ///         let Num(me) = self;
    ///         let Num(you) = other;
    ///         you - me
    ///     }
    /// }
    /// ```
    fn distance(&self, other: &Other) -> Self::Output;

    /// Returns `true` if the two data points are 'close' to each other
    ///
    /// For this default implementation, 'close' means any distance that is
    /// less than or equal to the `epsilon` parameter value.
    ///
    /// ## Params
    /// - `other` The other datapoint
    /// - `epsilon` The concept of proximity is determined by this parameter
    fn is_near(&self, other: &Other, epsilon: Self::Output) -> bool {
        self.distance(other) <= epsilon
    }
}

/// Functionality for accessing clustered data
///
/// Implementing this trait provides a common interface into clustered data,
/// without restricting the structure of the data to a specific, concrete type.
/// For instance, clustered data could be stored in a `HashMap` or a `Vec<(,)>`
pub trait Clustered<T> {
    /// Returns a list of clustered datapoints
    fn clusters(&self) -> Vec<Vec<T>>;

    /// Returns a list of all the datapoints that didn't fit into any clusters
    fn noise(&self) -> Vec<T>;
}

/// Base functionality of a clustering algorithm
pub trait Algorithm<T> {
    /// Cluster the algorithm's data
    ///
    /// To accomodate the various required parameters of different algorithms,
    /// it is expected that this trait is implemented by types that have
    /// access to their requirements. For instance, as fields in a struct.
    fn cluster(&self, clusterables: &[T]) -> Box<Clustered<T>>;
}
