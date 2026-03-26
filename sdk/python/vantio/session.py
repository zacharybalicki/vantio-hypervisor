import os
import uuid

class VantioSession:
    def __init__(self, agent_name: str):
        self.trace_id = str(uuid.uuid4())
        self.agent_name = agent_name

    def __enter__(self):
        # [ ∅ VANTIO ] Inject trace ID into OS environment for eBPF extraction
        os.environ["VANTIO_TRACE_ID"] = self.trace_id
        print(f"[ ∅ VANTIO VECTOR ] Session Boundary Locked. Trace: {self.trace_id}")
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        del os.environ["VANTIO_TRACE_ID"]
        print(f"[ ∅ VANTIO VECTOR ] Session Boundary Terminated.")
