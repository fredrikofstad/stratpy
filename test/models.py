from stratpy import *

# Creating the game models for Chapter 3
# Model 1a
game3_1a = Game(title="Model 1a", player_num=2)

# Creating the players
p1, p2 = game3_1a.player[1], game3_1a.player[2]
p1.name = "South Korea"
p2.name = "Japan"

# Setting up the players utility
s1, s2 = Variable("S_1"), Variable("S_2")
d1, d2 = Variable("D_1"), Variable("D_2")
s1_g, s2_a = Variable("S_1 + G"), Variable("S_2 - A")

# setting the players preferences
s1_g > s1 > d1
s2 > s2_a > d2

# Adding nodes
(game3_1a + Decision(p1, "Refrain", variable=(s1, s2))
          + (Decision(p1, "Persist") + Decision(p2, "Punish", variable=(d1, d2)) +
                                       Decision(p2, "Ignore", variable=(s1_g, s2_a))))

game3_1a.export_latex(1.5, "output/model3-1a.tex")

# Model 1b
game3_1b = Game(title="Model 1b", player_num=2)

# Creating the players
p1, p2 = game3_1b.player[1], game3_1b.player[2]
p1.name = "South Korea"
p2.name = "Japan"

# Adding nodes to the tree
(game3_1b + Decision(p1, "Refrain", utility=(3, 3))
 + (Decision(p1, "Persist") + Decision(p2, "Punish", utility=(1, 1)) +
    Decision(p2, "Ignore", utility=(4, 2))))

game3_1b.export_latex(1.5, "output/model3-1b.tex")


