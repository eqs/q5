import math
import q5

import numpy as np


class App(q5.BaseApp):
    def __init__(self):
        self.N = 200
        self.pts = []

    def update(self):
        t = q5.frame_count
        u = t / 60.0 * 2 * math.pi

        self.pts = []
        x = np.linspace(-np.pi, np.pi, self.N)
        y = 100.0 * np.sin(x + u)
        x = x / (2*np.pi) * 300.0

        self.pts = np.vstack([x, y]).T.tolist()
        self.pts = list(map(tuple, self.pts))

    def draw(self):
        q5.background(0)
        q5.fill(255, 0, 0)
        q5.stroke(220)
        q5.stroke_weight(2.0)
        q5.polyline(self.pts)


if __name__ == '__main__':
    app = App()
    app.run()
