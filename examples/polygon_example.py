import math
import q5


class App(q5.BaseApp):
    def __init__(self):
        self.N = 5
        self.pts = []

    def update(self):
        t = q5.frame_count
        u = t / 60.0 * 2 * math.pi * 0.25

        self.pts = []
        for n in range(self.N * 2):
            p = n * math.pi / self.N
            r = 200.0 if n % 2 == 0 else 100.0
            x = r * math.cos(u + p)
            y = r * math.sin(u + p)
            self.pts.append((x, y))

    def draw(self):
        q5.background(0)
        q5.no_fill()
        q5.stroke(220, 220, 0)
        q5.stroke_weight(5.0)
        q5.polygon(self.pts)


if __name__ == '__main__':
    app = App()
    app.run()
