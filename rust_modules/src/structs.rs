pub mod structs {
    use pyo3::prelude::*;
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct Stock {
        pub name: String,
        pub price: f64,
        pub profit: f64,
        pub earnings: f64,
    }

    impl PartialEq for Stock {
        fn eq(&self, other: &Self) -> bool {
            self.name == other.name 
            && self.price == other.price 
            && self.profit == other.profit 
            && self.earnings == other.earnings
        }
    }

    impl IntoPy<PyObject> for Stock {
        fn into_py(self, py: Python<'_>) -> PyObject {
            let mut serialized_object: HashMap<&str, Py<PyAny>> = HashMap::new();
            serialized_object.insert("name", self.name.into_py(py));
            serialized_object.insert("price", self.price.into_py(py));
            serialized_object.insert("profit", self.profit.into_py(py));
            serialized_object.insert("earnings", self.earnings.into_py(py));
            serialized_object.into_py(py)
        }
    }
}
