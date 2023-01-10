import sys, os

def validate(file1, file2):
    with open(file1, 'r') as f:
        f1array = (int(line.split()[1]) for line in f.readlines())
    
    with open(file2, 'r') as f:
        f2array = (int(line.split()[1]) for line in f.readlines())

    for (a, b) in zip(f1array, f2array):
        s = "{} {}\t{}\t{}".format("Expected", a, "Got", b)
        print(s)

        if a != 0 or b != 0:
            # make sure a and b have same signs if definite win
            assert(a * b > 0)
        else:
            # check if both are ties
            assert(a == b)

    print("OK")

if __name__ == "__main__":
    os.makedirs(name='test_outputs', exist_ok=True)

    if len(sys.argv) == 1:
        files = [os.path.basename(f) for f in os.listdir("test_outputs")]
        files = ["test_inputs/" + f[:-4] for f in files]
        print(files)
    else:
        files = sys.argv[1:]

    for file_to_check in files:
        basename = os.path.basename(file_to_check)
        outputfile = f'test_outputs/{basename}.log'
        os.system(f'cargo rr -- test {file_to_check} > {outputfile}')
        validate(file_to_check, outputfile)
        os.system(f'tail -n7 test_outputs/{basename}.log')
