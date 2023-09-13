from stratpy import Game, GameType


game = Game("title", 2, GameType.Normal)
print("hi")

print(f"{game.title} {game.players} {game.gametype}")
