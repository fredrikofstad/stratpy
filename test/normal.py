from stratpy import *

normal_utility = [[(1, 1), (2, 2)],
                  [(3, 3), (4, 4)]]

a1, a2, b1, b2, c1, c2, d1, d2 = (Variable("a_1"), Variable("a_2"), Variable("b_1"), Variable("b_2"),
                                  Variable("c_1"), Variable("c_2"), Variable("d_1"), Variable("d_2"))

normal_variable = [[(a1, a2), (b1, b2)],
                   [(c1, c2), (d1, d2)]]

normal_game = Game("Normal Game", utility=normal_utility)
var_game = Game("Normal Game (var)", variable=normal_variable)

print(normal_game.utility)
print(var_game.variable)
