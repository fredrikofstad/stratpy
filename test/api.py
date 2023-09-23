from stratpy import *
game = Game("Prisoner's Dilemma")
game2 = Game("Game 1", Type.Extensive)

# Create Players:
player1 = Player("South Korea")
player2 = Player("Japan")
player3 = Player(name="USA")

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

(game
 + (dec1 + dec5 + dec6)
 + (dec2 + dec3 + dec4))

print(game.root.children)
print(dec1.children)
print(dec2.children)


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

