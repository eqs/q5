# -*- coding: utf-8 -*-
import q5
import math


def draw_grid():
    q5.no_fill()
    q5.stroke(150)
    q5.stroke_weight(0.5)

    ncols = int(q5.width // 100)
    nrows = int(q5.height // 100)

    for i in range(nrows):
        q5.line(
            -q5.width / 2, i * 100 - q5.height / 2,
            q5.width / 2, i * 100 - q5.height / 2
        )

    for j in range(ncols):
        q5.line(
            j * 100 - q5.width / 2, -q5.height / 2,
            j * 100 - q5.width / 2, q5.height / 2
        )

    q5.stroke(0)
    q5.stroke_weight(1.4)
    q5.arrow(-q5.width / 2, 0, q5.width / 2, 0, 10.0, 10.0)
    q5.arrow(0, -q5.height / 2, 0, q5.height / 2, 10.0, 10.0)


class App(q5.BaseApp):
    def setup(self):
        q5.title('drawing functions')

    def draw(self):
        q5.background(220)

        draw_grid()

        q5.fill(255)
        q5.stroke(0)
        q5.stroke_weight(1.0)

        q5.circle(-200, 200, 200)
        q5.ellipse(100, 200, 250, 50)

        q5.rect(0, 0, 100, 100)
        q5.rect(-200, 0, 20, 100)

        q5.line(-300, -100, 0, -100)
        q5.arrow(-300, -150, 0, -150)
        q5.arrow(-300, -200, 0, -200, 20)
        q5.arrow(-300, -250, 0, -250, 20, 20)

        q5.polygon([(200, 100), (250, -100), (150, 0)])

        pts = []
        for k in range(8):
            t = k / 8 * math.pi * 2
            x = 200 + 100 * math.cos(t)
            y = -200 + 100 * math.sin(t)
            pts.append((x, y))
        q5.polygon(pts)

        q5.polyline([
            (300,    300),
            (300+50, 300-50),
            (300-20, 300-120),
            (300+20, 300-150),
            (300-30, 300-220),
            (300+30, 300-280),
            (300-10, 300-350),
            (300+40, 300-380),
        ])


if __name__ == '__main__':
    app = App()
    app.run()
