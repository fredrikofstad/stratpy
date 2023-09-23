from stratpy import *

game = Game("Prisoner's Dilemma")

# Create Players:
player1 = Player("South Korea")
player2 = Player("Japan")

# create utility:
a = Variable("A")
b = Variable("B")
c = Variable("C")
a > b == c

dec1 = Decision(player1, "1")
dec2 = Decision(player1, "2")
dec3 = Decision(player1, "3")
dec4 = Decision(player1, "4")
dec5 = Decision(player1, "5")
dec6 = Decision(player1, "6")
dec7 = Decision(player1, "7")

game + dec1 + dec2 + dec3 + (dec4 + dec5 + dec6) + dec7

dec4 + dec6
print(dec4.children)

for name in dec4.children:
    print(name.name)

print("-----")

for name in game.root.children:
    print(name.name)

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

