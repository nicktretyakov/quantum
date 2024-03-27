use wasm_bindgen::prelude::*;
// Аннотация для экспорта в JavaScript
#[wasm_bindgen]
struct Qubit {
    // Комплексные амплитуды для состояний |0⟩ и |1⟩.
    state_zero: f64,
    state_one: f64,
}
    impl Qubit {
    // Функция для создания нового кубита в состоянии |0⟩.
    fn new() -> Self {
    Qubit {
    state_zero: 1.0,
    state_one: 0.0,
    }
   }
    // Пример операции - операция Адамара.
    fn hadamard(&mut self) {
    let new_state_zero = (self.state_zero + self.state_one) / 2f64.sqrt();
    let new_state_one = (self.state_zero - self.state_one) / 2f64.sqrt();
    self.state_zero = new_state_zero;
    self.state_one = new_state_one;
   }
}
pub fn create_and_measure_qubit() -> String {
// Представление суперпозиции с помощью случайного выбора
let qubit_state = if js_sys::Math::random() < 0.5 { 0 } else { 1 };
// Имитация измерения кубита
if qubit_state == 0 {
"Кубит в состоянии |0>".to_string()
} else {
"Кубит в состоянии |1>".to_string()
}
}
