from stratpy import *
game = Game("Prisoner's Dilemma", 2)
p1 = game.player[1]
p2 = game.player[2]
p1.name = "South Korea"
p2.name = "Japan"

# create utility:
a = Variable("A")
b = Variable("B")
c = Variable("C")
a > b == c

dec1 = Decision(p1, "Commit")
dec2 = Decision(p1, "Do Nothing")
dec3 = Decision(p2, "Commit")
dec4 = Decision(p2, "Do nothing")
dec5 = Decision(p1, "Retaliate")
dec6 = Decision(p1, "Cooperate")
dec7 = Decision(p2, "Run away")

(game
 + (dec1 + dec5 + dec6))

dec2.add_nodes(dec1, dec2, dec3)

print(dec2)
print(dec2.children)

print(game.export())


'''
print(game.players)
print(game.title)
print(game.gametype)

print(f"{a.name} : {a.id}")
print(f"{b.name} : {b.id}")
print(f"{c.name} : {c.id}")
print("testing::")



print(b.lower)
print(b.equal)
print(b.higher)

'''
# overload < > == to arrange variables

# a > b == c > d

# create a list with values over and values less

