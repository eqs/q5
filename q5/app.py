import math
from typing import Union
from dataclasses import dataclass
from .engine import run


@dataclass
class Vector:
    x: float
    y: float
    z: float = 0.0

    def square_mag(self):
        return self.x**2 + self.y**2 + self.z**2

    def mag(self):
        return math.sqrt(self.square_mag())

    def dist(self):
        return self.mag()

    def dot(self, v):
        return self @ v

    def lerp(self, v, t: float):
        return (1.0 - t) * self + t * v

    def normalize(self):
        return self.mult(1.0 / self.mag())

    def mult(self, f: float):
        return Vector(f * self.x, f * self.y, f * self.z)

    def __add__(self, v):
        return Vector(self.x + v.x, self.y + v.y, self.z + v.z)

    def __sub__(self, v):
        return Vector(self.x - v.x, self.y - v.y, self.z - v.z)

    def __neg__(self):
        return Vector(-self.x, -self.y, -self.z)

    def __mul__(self, v):
        return self.x * v.x + self.y * v.y + self.z * v.z

    def __matmul__(self, v):
        return self.x * v.x + self.y * v.y + self.z * v.z

    def __eq__(self, v):
        return self.x == v.x and self.y == v.y and self.z == v.z


class BaseApp:
    def run(self):
        run(self.setup, self.update, self.draw)

    def setup(self):
        pass

    def update(self):
        pass

    def draw(self):
        pass
