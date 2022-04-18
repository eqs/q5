# q5

[![PyPI version](https://badge.fury.io/py/q5.svg)](https://badge.fury.io/py/q5)
[![Downloads](https://pepy.tech/badge/q5)](https://pepy.tech/project/q5)

Creative coding framework for Python

## Installation

You can install q5 with:

```
pip install q5
```

## Usage

### Examples

See [examples](https://github.com/eqs/q5/tree/main/examples).

### Project Template

```python
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


if __name__ == '__main__':
    app = App()
    app.run()
```

## Development

1. Install Rust lang: https://www.rust-lang.org/
1. Run commands:
```
pip install setuptools-rust
python setup.py develop
```

## License

MIT
