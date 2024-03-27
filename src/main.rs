use pyo3::prelude::*;

use pyo3::types::PyDict;

use pyo3::types::IntoPyDict;

fn main() -> PyResult<()> {
    let mut qubit = Qubit::new(); // Создаем кубит в состоянии |0⟩.
println!("Исходное состояние кубита: |0⟩ = {}, |1⟩ = {}", qubit.state_zero, qubit.state_one);
qubit.hadamard(); // Применяем операцию Адамара к кубиту.
println!("Состояние кубита после операции Адамара: |0⟩ = {}, |1⟩ = {}", qubit.state_zero, qubit.state_one);

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