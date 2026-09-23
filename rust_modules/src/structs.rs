pub mod structs {
    use pyo3::{ prelude::*, types::PyDict };

    #[derive(Debug, PartialEq)]
    pub struct Stock {
        pub name: String,
        pub price: f64,
        pub profit: f64,
        pub earnings: f64,
    }

    impl Stock {
        pub fn into_python(self, py: Python<'_>) -> PyResult<Py<PyDict>> {
            let serialized_object = PyDict::new(py);

            serialized_object.set_item("name", self.name)?;
            serialized_object.set_item("price", self.price)?;
            serialized_object.set_item("profit", self.profit)?;
            serialized_object.set_item("earnings", self.earnings)?;

            Ok(serialized_object.unbind())
        }
    }
}
