import sys, os

def validate(file1, file2):
    with open(file1, 'r') as f:
        f1array = (int(line.split()[1]) for line in f.readlines())
    
    with open(file2, 'r') as f:
        f2array = (int(line.split()[1]) for line in f.readlines())

    for (a, b) in zip(f1array, f2array):
        s = "{}\t{}\t{}\t{}".format(file1, a, b, file2)
        print(s)
        if a != 0 or b != 0:
            # make sure a and b have same signs if definite win
            assert(a * b > 0)
        else:
            # check if both are ties
            assert(a == b)

    print("OK")

if __name__ == "__main__":
    file_to_check = sys.argv[1]
    os.makedirs(name='test_outputs', exist_ok=True)
    basename = os.path.basename(file_to_check)
    outputfile = f'test_outputs/{basename}.log'
    os.system(f'cargo run --release {file_to_check} >> {outputfile}')
    validate(file_to_check, outputfile)
