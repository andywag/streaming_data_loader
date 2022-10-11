import zmq
import pickle
import zmq

def test_transport(host, port):
    ctx = zmq.Context()
    socket = ctx.socket(zmq.REQ)
    socket.connect(f"tcp://{host}:{port}")

    socket.send_string("Hello")
    data = socket.recv()
    result = pickle.loads(data)
    print(result)



test_transport("localhost",4000)
