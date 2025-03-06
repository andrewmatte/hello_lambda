"""This utility makes it easy to go from python data to binary data stored in persistent memory. Note that it accommodates geo-data too.
It is on pypi as https://pypi.org/project/serde-bin-vec/
You will probablby have to change the vec_length to meet your use case.
"""

import struct


# num_bytes = 32
# vec_length = 1024


def save(uuid, vector: list, geo = [0,0], filename='data.bin'):
    to_write = uuid.encode()
    to_write += b''.join([struct.pack('f', f) for f in vector])
    to_write += b''.join([struct.pack('f', f) for f in geo])
    f = open(filename, 'ab')
    f.write(to_write)
    f.close()


def read_all(num_bytes, vec_length, filename='data.bin'):
    f = open(filename, 'rb')
    output = []
    Is = 'B'*num_bytes
    Fs = 'f'*vec_length
    vec_len = vec_length*4
    while True:
        try:
            tmp = [f.read(num_bytes),
                   struct.unpack(Fs, f.read(vec_len)),
                   struct.unpack('ff', f.read(num_bytes))]
            output.append(tmp)
        except:
            break
    f.close()
    return output


def save_all(list_of_lists, filename):
    for item in list_of_lists:
        save(item[0], item[1], [0,0], filename)
