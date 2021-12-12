import argparse
import numpy as np
import pystencils as ps
import sympy as sp
import itertools

OFFSETS = list(itertools.product(range(-1, 2), repeat=2))


def make_kernel(shape):
    src, dst = ps.fields('src, dst: int32[2d]')

    flash_assignment = ps.AssignmentCollection({
        dst[0, 0]: sp.Piecewise((0, sp.Eq(src[0, 0], 0)),
                                (0, src[0, 0] > 9),
                                (src[0, 0] + sum([sp.Piecewise((1, src[o[0], o[1]] > 9), (0, True)) for o in OFFSETS]), True)),
    })

    ast = ps.create_kernel(flash_assignment)
    ps.show_code(ast)
    return ast.compile()


def step(kernel, input, dst):
    input[1:-1, 1:-1] += 1

    kernel(src=input, dst=dst)
    input, dst = dst, input
    while not np.all((input == dst)):
        kernel(src=input, dst=dst)
        input, dst = dst, input

    return np.sum(input[1:-1, 1:-1] == 0)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('input_file')
    args = parser.parse_args()

    with open(args.input_file) as f:
        input = f.read()
    input = np.array([[int(c) for c in line] for line in input.split('\n') if line], np.int32)
    input = np.pad(input, 1)
    dst = np.zeros_like(input)
    print(f"{input=}")

    kernel = make_kernel(input.shape)

    num_flashes = 0
    for step_number in range(1, 101):
        num_flashes += step(kernel, input, dst)
    print(f"{num_flashes=}")
    while not np.all(input == 0):
        num_flashes += step(kernel, input, dst)
        step_number += 1
    print(f"{step_number=}")


if __name__ == '__main__':
    main()
