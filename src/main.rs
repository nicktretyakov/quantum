use pyo3::prelude::*;

use pyo3::types::PyDict;

use pyo3::types::IntoPyDict;

fn main() -> PyResult<()> {

    Python::with_gil(|py| {

        // Импортируем Python-модуль

        let sys = py.import("sys")?;


        // Используем IntoPyDict для конвертации Python объекта в PyDict

        let path: &PyDict = sys.getattr("path")?.extract()?;


        // Добавляем новый путь в список path

        sys.getattr("path")?.call_method1("append", (".",))?;


        let qiskit_wrapper = py.import("qiskit_wrapper")?;


        // Вызов функции из Python-модуля

        let result = qiskit_wrapper.call_method0("run_quantum_circuit")?;


        println!("Результаты измерений: {:?}", result);


        Ok(())

    })

}