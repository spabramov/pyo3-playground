from .pyo3_playground import repository


def service(value: int) -> str:
    print(f"PYTHON: entering service: {value=}, saving doubled value")

    result = repository(value * 2)
    print(f"PYTHON: exiting service, {result=}")

    return result
