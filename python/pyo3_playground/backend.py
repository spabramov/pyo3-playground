import time
from typing import Protocol

CALLS: list[int] = []


class Repo(Protocol):
    def rs_sleep(self, seconds: int) -> int: ...


class Service:
    def __init__(self, repo: Repo | None = None) -> None:
        self.repo: Repo | None = repo

    def py_sleep(self, seconds: int) -> int:
        print(f"PYTHON: Sleeping for {seconds} seconds")
        time.sleep(seconds)
        return seconds

    def repo_sleep(self, seconds: int) -> int:
        assert self.repo
        CALLS.append(seconds)
        print("PYTHON: Invoking rs_sleep")
        return self.repo.rs_sleep(seconds)
