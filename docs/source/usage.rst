Usage
=====

.. _installation:

Installation
------------

To use stratpy, first install it using pip:

.. code-block:: console

   (.venv) $ pip install stratpy

Creating games
----------------

To create a new game use the  ``Game()`` constructor:

.. autofunction:: statpy.Game()

Optional parameters:
``Title``: enter an optional string as a title: (e.x.) ``"Game 1"``, ``"Prisoner's Dilemma"``


For example:

>>> from stratpy import *
>>> my_game = Game("My Game")

