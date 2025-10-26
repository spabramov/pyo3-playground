import time

from .pyo3_playground import repository, rs_sleep


def py_sleep(seconds: int) -> int:
    print(f"PYTHON: Sleeping for {seconds} seconds")
    time.sleep(seconds)
    return seconds


def py_rs_sleep(seconds: int) -> int:
    print("Invoking rs_sleep")
    return rs_sleep(seconds)


def service(value: int) -> str:
    print(f"PYTHON: entering service: {value=}, saving doubled value")

    result = repository(value * 2)
    print(f"PYTHON: exiting service, {result=}")

    return result
