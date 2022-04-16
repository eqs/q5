import math
import random
from typing import Tuple
from dataclasses import dataclass
import q5

N_STARS = 64
colors = [
    (224, 221, 0), (112, 22, 224), (224, 72, 11), (22, 224, 192)
]


class Star:

    N: int
    radius: float
    pos: q5.Vector
    vel: q5.Vector
    angle_vel: float
    color: Tuple

    def __init__(self, pos: q5.Vector, vel: q5.Vector,
                 radius: float = 32.0, angle_vel: float = 0.25,
                 color: Tuple = (220, 220, 0)):
        self.N = 5
        self.radius = radius
        self.pos = pos
        self.vel = vel
        self.angle_vel = angle_vel
        self.color = color

    def update(self):
        if self.pos.x < -q5.width / 2.0 - self.radius:
            self.pos.x = self.pos.x + q5.width + self.radius * 2.0
        if self.pos.x > q5.width / 2.0 + self.radius:
            self.pos.x = self.pos.x - q5.width - self.radius * 2.0
        if self.pos.y < -q5.height / 2.0 - self.radius:
            self.pos.y = self.pos.y + q5.height + self.radius * 2.0
        if self.pos.y > q5.height / 2.0 + self.radius:
            self.pos.y = self.pos.y - q5.height - self.radius * 2.0

        self.pos = self.pos + self.vel

    def draw(self):
        t = q5.frame_count
        u = t / 60.0 * 2 * math.pi * self.angle_vel

        pts = []
        for n in range(self.N * 2):
            p = n * math.pi / self.N
            s = self.radius if n % 2 == 0 else self.radius / 2.0
            x = s * math.cos(u + p)
            y = s * math.sin(u + p)
            pts.append((x, y))

        q5.no_fill()
        q5.stroke(*self.color)
        q5.stroke_weight(2.0)

        q5.push_matrix()
        q5.translate(self.pos.x, self.pos.y)
        q5.polygon(pts)
        q5.pop_matrix()


def generate_star(x=0.0, y=0.0):
    mag = random.uniform(1.0, 6.0)
    angle = random.uniform(0.0, math.pi * 2.0)
    radius = random.uniform(16.0, 48.0)
    angle_vel = random.uniform(0.15, 0.75)
    if random.random() > 0.5:
        angle_vel = -angle_vel

    star = Star(
        q5.Vector(x, y),
        q5.Vector.from_angle(angle, mag=mag),
        radius=radius,
        angle_vel=angle_vel,
        color=random.choice(colors)
    )

    return star


class App(q5.BaseApp):
    def setup(self):
        self.stars = []
        for k in range(N_STARS):
            star = generate_star()
            self.stars.append(star)

    def update(self):
        for star in self.stars:
            star.update()

    def draw(self):
        q5.background(0, 0, 0)
        for star in self.stars:
            star.draw()

    def mouse_pressed(self):
        if q5.mouse_button == q5.MOUSE_LEFT:
            star = generate_star(q5.mouse_x, q5.mouse_y)
            self.stars.append(star)


if __name__ == '__main__':
    app = App()
    app.run()
