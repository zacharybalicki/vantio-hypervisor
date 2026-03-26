import sys
import os

# [ ∅ VANTIO ] INJECTING THE SDK INTO THE SYSTEM PATH
sys.path.append('sdk/python')
from vantio.session import VantioSession

# [ ∅ VANTIO ] SIMULATING ROGUE AI PROCESS EXECUTION
if __name__ == "__main__":
    with VantioSession('Rogue-Agent-01'):
        os.system('echo "[ PHANTOM ENGINE ] EXECUTING AI PAYLOAD INSIDE BOUNDARY"')
