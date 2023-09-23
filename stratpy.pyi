from stratpy.stratpy import *

class Game:
    """
    The main class representing the game model.
    :param title: optional title of the game, used when displaying the game
    :param gametype: optional parameter for the type of game. Either Type.Normal or Type.Extensive

    """
    def __new__(cls, title: str, gametype: Type) -> Variable: ...

class Decision:
    """
    Class representing the decision nodes of a player.

    :param player: the player who performs the action
    :param name: the name of the decision
    """
    def __new__(cls, player: Player, name: str) -> Decision: ...

class Player:
    """
    Class representing players of a game.
    :param name: the name of the player
    """
    def __new__(cls, name: str) -> Player: ...


class Variable:
    """
    Class representing a variable utility of a player.
    :param name: the display name of the variable
    """
    def __new__(cls, name: str) -> Variable: ...