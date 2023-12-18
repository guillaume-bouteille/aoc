
import subprocess
import os
import re
import logging
import time
from enum import IntEnum

class ExoStatus(IntEnum):
    OK = 0
    SKIPPED = 1
    COMPILATION_ERROR = 3
    MISSING_INPUT = 4
    EXECUTION_ERROR = 5
    WRONG_RESULT_FORMAT = 6
    WRONG_REFERENCE_FORMAT = 7
    WRONG_RESULT = 8

def parse_result( result: list[str] ) -> tuple[str, str] | None:
    if len(result) != 2:
        return None
    m_res1 = re.match("^P1=(.*)$", result[0])
    if m_res1 is None:
        return None
    m_res2 = re.match("^P2=(.*)$", result[1])
    if m_res2 is None:
        return None
    return (m_res1.group(1), m_res2.group(1))

class Exo:
    def __init__(self, root_dir : str, year : int, day : int):
        self.root_dir = root_dir
        self.year = year
        self.day = day
        self.name = f"Exo-{year:04d}-{day:02d}"
        self.source_path = os.path.join(root_dir, str(year), f"day{day:02d}", "main.rs")
        self.obj_path = os.path.join(root_dir, "obj", str(year), f"day{day:02d}")
        self.executable_path = os.path.join(self.obj_path, "main")
        self.data_path = os.path.join(root_dir, "data", str(year), f"day{day:02d}")
        self.logger = logging.getLogger(self.name)

    def compile(self) -> ExoStatus:

        if not os.path.exists(self.obj_path):
            os.makedirs(self.obj_path)

        self.logger.debug("Compiling %s", self.source_path)
        process = subprocess.Popen(["rustc", self.source_path, "-o", self.executable_path],
                             stdout=subprocess.PIPE,
                             stderr=subprocess.PIPE)
        stdout, stderr = process.communicate()
        if len(stderr) > 0:
            self.logger.warning("Compilation logs for %s", self.source_path)
            for l in stderr.decode("utf-8").strip().split("\n"):
                self.logger.warning("> %s", l)
            self.logger.warning("End of compilation logs for %s", self.source_path)
        if process.returncode != 0:
            self.logger.error("Compilation error on %s", self.source_path)
            return ExoStatus.COMPILATION_ERROR
        self.logger.debug("Compiled %s", self.source_path)

        return ExoStatus.OK

    def execute(self, is_example : bool) -> ExoStatus:
        if is_example:
            input_path = os.path.join(self.data_path, "input_example.txt")
            ref_path = os.path.join(self.data_path, "ref_example.txt")
        else:
            input_path = os.path.join(self.data_path, "input.txt")
            ref_path = os.path.join(self.data_path, "ref.txt")

        if not os.path.exists(input_path):
            self.logger.error("Input file %s does not exist", input_path)
            return ExoStatus.MISSING_INPUT

        input_f = open(input_path, "r")
        start_time = time.time()
        process = subprocess.Popen([self.executable_path], stdin=input_f,
                         stdout=subprocess.PIPE,
                         stderr=subprocess.PIPE)
        stdout, stderr = process.communicate()
        end_time = time.time()
        exec_time = end_time-start_time

        self.logger.debug("Executing time %fs", exec_time)
        if len(stderr) > 0:
            self.logger.debug("Execution logs")
            for l in stderr.decode("utf-8").strip().split("\n"):
                self.logger.debug("> %s", l)
            self.logger.debug("End of execution logs")

        if process.returncode != 0:
            self.logger.error("Execution error")
            return ExoStatus.EXECUTION_ERROR

        result_str = stdout.decode("utf-8").strip().split("\n")
        self.logger.debug("Execution result")
        for l in result_str:
            self.logger.debug("> %s", l)
        self.logger.debug("End of execution result")

        result = parse_result(result_str)
        if result is None:
            self.logger.error("Failed to parse results")
            return ExoStatus.WRONG_RESULT_FORMAT

        self.logger.debug("Result part 1 = %s part 2 = %s", *result)

        if not os.path.exists(ref_path):
            self.logger.error("Reference file %s not found", ref_path)
            return ExoStatus.WRONG_REFERENCE_FORMAT

        reference = parse_result(open(ref_path, "r").readlines())
        if reference is None:
            self.logger.error("Failed to parse reference")
            return ExoStatus.WRONG_REFERENCE_FORMAT

        self.logger.debug("Reference part 1 = %s part 2 = %s", *reference)

        if result != reference:
            self.logger.error("Result %s does not match reference %s", result, reference)
            return ExoStatus.WRONG_RESULT

        self.logger.info("Results (part 1 = %s part 2 = %s) are correct (time = %fs)", *result, exec_time)
        return ExoStatus.OK

    def full_monty(self) -> ExoStatus:
        sts = self.compile()
        if sts != ExoStatus.OK:
            return sts

        input_path = os.path.join(self.data_path, "input_example.txt")
        ref_path = os.path.join(self.data_path, "ref_example.txt")
        if os.path.exists(input_path) and os.path.exists(ref_path):
            sts = self.execute(is_example = True)
            if sts != ExoStatus.OK:
                return sts
        else:
            self.logger.debug("Skipping example")

        sts = self.execute(is_example = False)
        return sts
        

if __name__ == "__main__":
    class CustomFormatter(logging.Formatter):
        """Custom log formatter"""
        blue = "\x1b[36;20m"
        green = "\x1b[32;20m"
        yellow = "\x1b[33;20m"
        red = "\x1b[31;20m"
        bold_red = "\x1b[31;1m"
        reset = "\x1b[0m"
        fmt_str = "%(name)s - %(levelname)s - %(message)s"
        FORMATS = {
            logging.DEBUG: blue + fmt_str + reset,
            logging.INFO: green + fmt_str + reset,
            logging.WARNING: yellow + fmt_str + reset,
            logging.ERROR: red + fmt_str + reset,
            logging.CRITICAL: bold_red + fmt_str + reset
        }
        def format(self, record):
            log_fmt = self.FORMATS.get(record.levelno)
            formatter = logging.Formatter(log_fmt)
            return formatter.format(record)

    ch = logging.StreamHandler()
    ch.setFormatter(CustomFormatter())
    logging.basicConfig(handlers=[ch], level=logging.INFO) # Change to logging.DEBUG if issues

    root_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))


    # Comment / Uncomment lines to skip exos or not
    exos = [
        Exo(root_dir, year=2022, day=1),
        Exo(root_dir, year=2022, day=2),
        Exo(root_dir, year=2022, day=3),
        Exo(root_dir, year=2022, day=4),
        Exo(root_dir, year=2022, day=5),
        Exo(root_dir, year=2023, day=1),
        Exo(root_dir, year=2023, day=2),
        Exo(root_dir, year=2023, day=3),
        Exo(root_dir, year=2023, day=4),
        Exo(root_dir, year=2023, day=5),
        Exo(root_dir, year=2023, day=6),
        Exo(root_dir, year=2023, day=7),
        Exo(root_dir, year=2023, day=8),
        Exo(root_dir, year=2023, day=9),
        Exo(root_dir, year=2023, day=10),
        Exo(root_dir, year=2023, day=11),
        Exo(root_dir, year=2023, day=12),
        Exo(root_dir, year=2023, day=13),
    ]
    res = []

    for exo in exos:
        sts = exo.full_monty()
        res.append((exo, sts))

    print("Synthesis:")
    for exo, sts in res:
        print("%s -> %s" % (exo.name, sts.name))
