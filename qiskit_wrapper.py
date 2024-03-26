from qiskit import QuantumCircuit, Aer, execute
def run_quantum_circuit():
# Создание квантовой схемы
circuit = QuantumCircuit(2, 2)
circuit.h(0)
circuit.cx(0, 1)
circuit.measure([0, 1], [0, 1])
# Выполнение квантовой схемы на симуляторе
simulator = Aer.get_backend('qasm_simulator')
job = execute(circuit, simulator, shots=1000)
result = job.result()
# Возвращение результатов измерений
counts = result.get_counts(circuit)
return counts
