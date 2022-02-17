from .engine import run


class BaseApp:
    def run(self):
        run(self.update, self.draw)

    def update(self):
        pass

    def draw(self):
        pass
