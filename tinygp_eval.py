from copy import deepcopy
from typing import Self
import numpy as np

def tinygp_eval(solution: str, X1: np.ndarray, X2: np.ndarray):
    # imports are used inside the eval function
    from numpy import sin, cos
    # TODO implement protected division
    return eval(solution)

class ProtDiv(float):
    def __truediv__(self, __value: float) -> float:
        if isinstance(__value, (float, int)):
            if abs(__value) < 0.001:
                return self
            return ProtDiv(super().__truediv__(__value))
        if isinstance(__value, list):
            unfolded = ProtDivArray([self for _ in len(__value)])
            return unfolded / ProtDivArray(__value)

    def __add__(self, value: float|list) -> Self:
        if isinstance(value, (float, int)):
            return ProtDiv(super().__add__(value))
        if isinstance(value, list):
            return ProtDivArray(value) + self

    def __sub__(self, value: float|list) -> Self:
        if isinstance(value, (float, int)):
            return ProtDiv(super().__sub__(value))
        if isinstance(value, list):
            return (ProtDivArray(value) * (-1) + value)

    def __mul__(self, value: float|list) -> Self:
        if isinstance(value, (float, int)):
            return ProtDiv(super().__mul__(value))
        if isinstance(value, list):
            return ProtDivArray(value) * self


class ProtDivArray(list):
    def __truediv__(self, value: float|list) -> Self:
        if isinstance(value, (float, int)):
            return ProtDivArray(map(lambda x: ProtDiv(x) / value, self))
        if isinstance(value, list):
            return ProtDivArray(map(lambda x: ProtDiv(x[0] / x[1]), zip(self, value)))

    def __add__(self, value: float|list) -> Self:
        if isinstance(value, (float, int)):
            return ProtDivArray(map(lambda x: ProtDiv(x + value), self))
        if isinstance(value, list):
            return ProtDivArray(map(lambda x: ProtDiv(x[0] + x[1]), zip(self, value)))

    def __sub__(self, value: float|list) -> Self:
        if isinstance(value, (float, int)):
            return ProtDivArray(map(lambda x: ProtDiv(x - value), self))
        if isinstance(value, list):
            return ProtDivArray(map(lambda x: ProtDiv(x[0] - x[1]), zip(self, value)))

    def __mul__(self, value: float|list) -> Self:
        if isinstance(value, (float, int)):
            return ProtDivArray(map(lambda x: ProtDiv(x * value), self))
        if isinstance(value, list):
            return ProtDivArray(map(lambda x: ProtDiv(x[0] * x[1]), zip(self, value)))


def protect_division(val: float|np.ndarray):
    if isinstance(val, (float, int)):
        return ProtDiv(val)
    elif isinstance(val, (np.ndarray, list)):
        return ProtDivArray(list(val))

if __name__ == "__main__":
    normal_float = 3.0
    float_zero = 0.0


    prot_float = ProtDiv(normal_float)
    prot_zero = ProtDiv(float_zero)

    assert not isinstance(normal_float, ProtDiv)
    assert isinstance(prot_float, float)
    assert isinstance(prot_float / 2.0, ProtDiv)
    assert isinstance(prot_float * 2.0, ProtDiv)

    divzero = prot_float / prot_zero
    assert divzero == 3.0
    assert divzero / prot_zero == 3.0
    assert prot_float * 3.0 == 9.0

    protstring = "protect_division(3.0) / protect_division(0.0)"
    assert eval(protstring) == 3.0

    solution = "(10) / ((-2) + (X1))"
    solution = solution.replace("(", "protect_division(")
    # print(solution)
    assert solution == "protect_division(10) / protect_division(protect_division(-2) + protect_division(X1))", solution

    linspace = np.linspace(1, 2, 2)
    assert not isinstance(linspace, float)
    assert isinstance(linspace, np.ndarray)
    assert list(linspace) == [1.0, 2.0]
    prot_space = ProtDivArray(linspace)
    assert prot_space == [1.0, 2.0]
    assert prot_space / 2.0 == [0.5, 1.0]
    assert prot_space / [1.0, 2.0] == [1.0, 1.0]
    assert list(np.sin(ProtDivArray([0.0, 0.0]))) == [0.0, 0.0]
    res = prot_space * 2
    assert res == [2.0, 4.0], res

    assert isinstance(prot_space, ProtDivArray)
    assert list(prot_space / 0.0) == list(prot_space)

    assert list(ProtDiv(2.0) + prot_space) == [3.0, 4.0]

    X1 = np.linspace(0, 10, 11)
    print(X1)
    Y = eval(solution)
    print(Y)