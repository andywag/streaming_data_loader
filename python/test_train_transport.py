import zmq
import pickle
import zmq
import time
import yaml
import argparse


def test_transport(args, sim_length, report):
    ctx = zmq.Context()
    socket = ctx.socket(zmq.REQ)
    #socket.connect(f"tcp://{host}:{port}")
    tic = time.time()
    for x in range(sim_length):
        socket.connect(args['sink']['config']['address'])
        socket.send_string("Hello")
        data = socket.recv()
        #try:
        result = pickle.loads(data)
        if x % report == report - 1:
            delta = time.time() - tic
            qps = report*args['tokenizer']['config']['batch_size']/delta
            print("Result", qps, time.time() - tic)
            tic = time.time()

        #print("R", x, sim_length)
        #if x == 330:
        #    print(result)
        
        #if isinstance(result,str):
        #    delta = time.time() - tic
        #    qps = x*args['tokenizer']['config']['batch_size']/delta
        #    print("Result", qps, time.time() - tic)
        #    break
    
    
    #print(result)

parser = argparse.ArgumentParser(description='Test Data Loading')
parser.add_argument('--file', type=str, default='../rust/tests/masking_tests.yaml')
parser.add_argument('--config', type=str, default='zmq_ipc')
parser.add_argument('--iterations', type=int, default=5000)
parser.add_argument('--report', type=int, default=100)

def main():
    args = parser.parse_args()
    with open(args.file) as file:
        test = yaml.load(file, Loader=yaml.FullLoader)
    #print(test[args.config])
    test_transport(test[args.config],args.iterations, args.report)

if __name__ == '__main__':
    print("Running Test...")
    main()



#test_transport("localhost",4000)
