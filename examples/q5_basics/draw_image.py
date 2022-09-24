import numpy as np
import q5


class App(q5.BaseApp):
    def setup(self):
        img = np.zeros((200, 200, 3))
        self.img = q5.QImage(img)

    def update(self):
        pass

    def draw(self):
        q5.background(220)
        q5.image(self.img, 0.0, 0.0)


if __name__ == '__main__':
    app = App()
    app.run()
