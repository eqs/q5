import math
import q5


class App(q5.BaseApp):
    def setup(self):
        self.x = 0.0
        self.y = 0.0

    def update(self):
        t = q5.frame_count
        u = t / 60.0 * 2 * math.pi
        self.x = 100.0 * math.cos(u)
        self.y = 100.0 * math.sin(u)

    def draw(self):
        q5.background(220)
        q5.translate(self.x, self.y)
        q5.ellipse(0.0, 0.0, 100.0, 100.0)


if __name__ == '__main__':
    app = App()
    app.run()
