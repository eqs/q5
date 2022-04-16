# -*- coding: utf-8 -*-
import q5
import math

N = 8


def draw_face():
    q5.rect(0, 0, 200, 150)
    q5.circle(-50, 25, 25)
    q5.circle(50, 25, 25)
    q5.rect(0, -25, 100, 25)


class App(q5.BaseApp):
    def setup(self):
        q5.title('q5 app')

    def update(self):
        pass

    def draw(self):
        q5.background(220)

        q5.stroke_weight(1.0)
        draw_face()

        q5.stroke_weight(2.0)
        for k in range(N):
            t = q5.frame_count / 300.0 * 2.0 * math.pi
            phi = k / N * 2.0 * math.pi

            q5.push_matrix()

            q5.rotate(t + phi)
            q5.translate(200.0, 0.0)
            q5.scale(0.5, 0.5)

            q5.push_matrix()
            q5.rotate(-math.pi / 2.0)
            draw_face()
            q5.pop_matrix()

            q5.pop_matrix()


if __name__ == '__main__':
    app = App()
    app.run()
