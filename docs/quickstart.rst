Quickstart
==========

Installation
------------

Windows & Mac
+++++++++++++

Run the following command:

.. code::

   pip install q5

Usage
-----

To ensure that q5 was installed correctly,
we can verify the installation by running the example code:

.. code::

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
