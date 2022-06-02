from asyncio.subprocess import PIPE
import glob
from multiprocessing import Event
import os
from re import T
import sys
import subprocess
import threading
import asyncio
from queue import Queue
import time

class Stdout(threading.Thread):
    def __init__(self, p: subprocess.Popen[bytes]):
        threading.Thread.__init__(self)
        self.daemon = True
        self.kill_event = Event()
        self.p = p

    def run(self) -> None:
        while not self.p.poll():
            try:
                lines = self.p.stdout.readlines()

                print('\n'.join([l.decode() for l in lines]))
            except:
                print("Process is terminated, no stdout")
                break

            if self.kill_event.is_set():
                break

    def kill(self):
        self.kill_event.set()
        

class Stderr(threading.Thread):
    def __init__(self, p: subprocess.Popen[bytes]):
        threading.Thread.__init__(self)
        self.daemon = True
        self.kill_event = Event()
        self.p = p

        

    def run(self) -> None:
       while not self.p.poll():
            try:
                lines = self.p.stderr.readlines()

                print('\n'.join([l.decode() for l in lines]))
            except:
                print("Process is terminated, no stderr")
                break

            if self.kill_event.is_set():
                break


    def kill(self):
        self.kill_event.set()



class Stdin(threading.Thread):
    def __init__(self, p: subprocess.Popen[bytes]):
        threading.Thread.__init__(self)
        self.daemon = True
        self.message = ""
        self.event = threading.Event()
        self.kill_event = Event()
        self.p = p

    def run(self) -> None:
        self.event.wait()

        res = self.p.communicate(self.message.encode())

        self.event.clear()

        if self.kill_event.is_set():
            self.exit()
        
    

    def __call__(self, *args, **kwg):
        sep = kwg.get("sep") if kwg.get("sep") is not None else " "
        self.message = sep.join(args)
        self.event.set()


    def kill(self):
        self.kill_event.set()

                


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
                f"rust-lldb {file_path}", 
                stdout=subprocess.PIPE, 
                stderr=subprocess.PIPE,
                stdin=subprocess.PIPE,
                shell=True,
            )

            event = Event()
            queue = Queue()            

            stdin = Stdin(subp) 
            stdin.start()
            
            stdout = Stdout(subp)
            stdout.start()
            
            stderr = Stderr(subp)
            stderr.start()
            
            async def get_and_parse_input():
                std_in = sys.stdin.readline()
                queue.put(std_in.strip())
                event.set()
                
                br = breakout()
                if br:
                    queue.put("break")

                return True

            async def quit():
                global stdin, stdout, stderr

                print("Exiting LLDB...")  


                stdin.kill()
                stdout.kill()
                stderr.kill()

                subp.communicate(b'quit')

                subp.kill()
                
                print("Exited process with SIGKILL")

                return True


            def breakout():
                poll = subp.poll()

                if poll is not None:
                    stdin.kill()
                    stdout.kill()
                    stderr.kill()
                    print(f"Process exited with status {poll}")
                    return True

                else:
                    return False

            async def main_task():
                while True:                 
                    await get_and_parse_input()

                    if event.is_set():
                        event.clear()                    

                        popped = queue.get()
                

                        match popped:
                            case 'exit': 
                                await quit()
                                return True 
                            case 'break':
                                return True
                            case _:
                                stdin(popped)

                    
                


            async def main_loop():
                task = asyncio.create_task(main_task())
                done, _ = await asyncio.wait({ task })

                if task in done:
                    print("Task is done")
                    return True
        
            
            asyncio.run(main_loop())

            print("Exiting script...")


            sys.exit()

