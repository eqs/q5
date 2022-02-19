from .engine import run


class BaseApp:
    def run(self):
        run(self.setup, self.update, self.draw)

    def setup(self):
        pass

    def update(self):
        pass

    def draw(self):
        pass
