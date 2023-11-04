from typing import IO, List, Optional, Tuple

import abc
import argparse
import inspect
import os
import pathlib
import shutil
import subprocess
import sys
import unittest

SUCCESS = 0
FAILURE = 1


class Util:
    class TestRegression(unittest.TestCase, abc.ABC):
        name: str
        expected_files: List[str] = ["output.txt"]

        def test(self) -> None:
            work_dir = self.get_work_dir()
            self.setup_dir(work_dir)

            self.run_command(work_dir)

            self.diff_against_baselines(work_dir)

        def get_work_dir(self) -> pathlib.Path:
            return (
                pathlib.Path(os.path.dirname(os.path.abspath(__file__)))
                / "work_dirs"
                / self.name
            )

        def get_baseline_dir(self) -> pathlib.Path:
            return (
                pathlib.Path(os.path.dirname(os.path.abspath(__file__)))
                / "baselines"
                / self.name
            )

        def setup_dir(
            self, work_dir: pathlib.Path, clear_if_exists: bool = True
        ) -> None:
            if work_dir.exists() and clear_if_exists:
                shutil.rmtree(work_dir)

            work_dir.mkdir(parents=True, exist_ok=not clear_if_exists)

        def run_command(self, work_dir: pathlib.Path) -> str:
            program_path = (
                pathlib.Path(os.path.dirname(os.path.realpath(__file__)))
                / ".."
                / "target"
                / "release"
                / "quickbms-lsp"
            )
            command = [program_path]

            with open(self.get_baseline_dir() / "input.txt", "r") as input_stream:
                stdin = input_stream.read()

            process = subprocess.Popen(
                command,
                cwd=work_dir,
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
            )
            stdout, _ = process.communicate(input=stdin.encode("utf-8"))

            with open(work_dir / "output.txt", "wb") as output_stream:
                output_stream.write(stdout)

        def diff_against_baselines(self, work_dir: pathlib.Path) -> None:
            baseline_dir = self.get_baseline_dir()
            self.assertTrue(baseline_dir.exists())

            for filename in self.expected_files:
                actual = work_dir / filename
                baseline = baseline_dir / filename

                self.assertTrue(actual.exists())
                self.assertTrue(baseline.exists())

                with open(baseline, "r") as baseline_stream:
                    with open(actual, "r") as actual_stream:
                        self.assertEqual(list(baseline_stream), list(actual_stream))

        def update_baseline(self) -> None:
            work_dir = self.get_work_dir()
            self.setup_dir(work_dir)

            self.run_command(work_dir)

            baseline_dir = self.get_baseline_dir()

            shutil.copy(work_dir / "output.txt", baseline_dir / "output.txt")


class TestStartupAndShutdown(Util.TestRegression):
    name = "startup_and_shutdown"


def update_baselines(tests: List[type[Util.TestRegression]]) -> None:
    for test in tests:
        print(f"Updating baseline: {test.name}")
        t = test()
        t.update_baseline()


if __name__ == "__main__":
    tests = []
    for name, obj in inspect.getmembers(sys.modules[__name__]):
        if (
            inspect.isclass(obj)
            and obj != Util
            and issubclass(obj, Util.TestRegression)
        ):
            tests.append(obj)

    update_baselines(tests)

