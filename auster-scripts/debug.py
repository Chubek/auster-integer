from asyncio.subprocess import PIPE
import glob
from multiprocessing import Event
import os
import sys
import subprocess
import threading
import asyncio
from queue import Queue

class Stdout(threading.Thread):
    def __init__(self, p: subprocess.Popen[bytes]):
        super.__init__()

        self.p = p

    def run(self) -> None:
        while True:
            line = self.p.stdout.readline()

            print(line)

class Stderr(threading.Thread):
    def __init__(self, p: subprocess.Popen[bytes]):
        super.__init__()

        self.p = p

    def run(self) -> None:
        while True:
            line = self.p.stderr.readline()

            print(line)


class Stdin(threading.Thread):
    def __init__(self, p: subprocess.Popen[bytes]):
        super.__init__()
        self.message = ""
        self.event = threading.Event()
        self.p = p

    def run(self) -> None:
        while True:
            if self.event.is_set():
                print(f"Stdin got: {line}")
                self.event.clear()
                line = self.p.stdin.write(self.message)
    

    def __call__(self, *args, **kwg):
        self.message = kwg.get("sep").join(args)
        self.event.set()

                


if __name__ == "__main__":
    args = [arg.lower() for arg in sys.argv[1:]]

    match args[0]:
        case 'setup':
            stat = subprocess.call("sudo apt-get install lldb rust-lldb".split(" "))

            match stat:
                case 0: print("Successfully installed LLDB and Rust-LLDB")
                case _: print("An error occured? Status is not 0")

        case 'into':
            sub, hash = args[1:3]
            
            path = os.path.join(os.getcwd(), os.path.join("target", "debug", "deps"))
            if not os.path.exists(path):
                raise Exception("Path does not exist")
            
            os.chdir(path)           
            latest = ""

            match hash:
                case 'latest':
                    fname = f"auster_{sub}-*"
                    files = glob.glob(fname)

                    latest = max(files, key=os.path.getctime)

                case _:
                    fname = f"*{sub}-{hash}"
                    files = glob.glob(fname)

                    if len(files) == 0:
                        raise Exception("Hash does not exsit")

                    latest = max(files, key=os.path.getctime)
            
            latest = latest.replace("rmeta", "rlib")

            file_path = os.path.join(path, latest)

            subp = subprocess.Popen(
                f"rust-lldb {file_path}".split(" "), 
                stdout=subprocess.PIPE, 
                stderr=subprocess.PIPE,
                stdin=subprocess.PIPE,
                shell=True,
            )
            
            event = Event()
            queue = Queue()
            
            stdin = Stdin()
            stdout = Stdout()
            sys.stderr = Stderr()

            def get_and_parse_input():
                std_in = sys.stdin.readline()
                queue.put(std_in)
                event.set()

            async def main_loop():
                while True:
                    if event.is_set:
                        event.clear()
                        popped = queue.get()

                        match popped:
                            case 'exit': 
                                return
                            case _:
                                stdin(popped)


            loop = asyncio.get_event_loop()

            loop.run_until_complete(main_loop)


