from stratpy import *

normal_utility = [[(1, 1), (2, 2)],
                  [(3, 3), (4, 4)]]

a1, a2, b1, b2, c1, c2, d1, d2 = (Variable("A_1"), Variable("A_2"), Variable("B_1"), Variable("B_2"),
                                  Variable("C_1"), Variable("C_2"), Variable("D_1"), Variable("D_2"))

normal_variable = [[(a1, a2), (b1, b2)],
                   [(c1, c2), (d1, d2)]]

normal = Game("Normal Game with numerals", utility=normal_utility)
var = Game("Normal Game with variables", variable=normal_variable)

p1, p2 = var.player[1], var.player[2]
p1.name = "South Korea"

p1.actions = ["Negotiate", "Refrain"]
p2.name = "Japan"
p2.actions = ["Negotiate", "Retaliate"]

var.export_latex(3.5, "output/normal.tex")
