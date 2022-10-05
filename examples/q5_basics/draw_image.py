import numpy as np
import q5


class App(q5.BaseApp):
    def setup(self):
        img = np.zeros((480, 480, 3))

        ii, jj = np.meshgrid(
            np.arange(480), np.arange(480)
        )

        img[ii, jj, 0] = ii / 480 * 255
        img[ii, jj, 1] = jj / 480 * 255
        img[ii, jj, 2] = 0.0

        self.img = q5.QImage.from_ndarray(img)

    def update(self):
        pass

    def draw(self):
        q5.background(220)

        t = q5.frame_count / 60.0 * np.pi * 0.25
        for k in range(8):
            p = k / 8 * np.pi * 2.0
            x = 200.0 * np.cos(p + t)
            y = 200.0 * np.sin(p + t)
            q5.push_matrix()
            q5.translate(x, y)
            q5.scale(0.15)
            q5.rotate(p + t)
            q5.image(self.img, 0.0, 0.0)
            q5.pop_matrix()


if __name__ == '__main__':
    app = App()
    app.run()
