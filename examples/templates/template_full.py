# -*- coding: utf-8 -*-
import q5


class App(q5.BaseApp):
    def setup(self):
        q5.title('q5 app')

    def update(self):
        pass

    def draw(self):
        q5.background(220)
        q5.ellipse(0.0, 0.0, 200.0, 200.0)

    def mouse_pressed(self):
        pass

    def mouse_released(self):
        pass

    def mouse_moved(self):
        pass

    def mouse_entered(self):
        pass

    def mouse_exited(self):
        pass

    def key_pressed(self):
        pass

    def key_released(self):
        pass


if __name__ == '__main__':
    app = App()
    app.run()
