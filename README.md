# Battlesnake

Project I worked on last winter. Game involves 2-4 snakes, and last one on the board wins. To improve speed and search depth, my engine can only handle duels.

# How it works

The board state is represented by a 2d array of int values. first digit is which snake (0 or 1), and then the 7 others encode how much longer before the snake is gone. Food is represented as 0xFF.

It uses minimax with alpha-beta pruning to remove redundant search paths. Each node is evaluated with floodfill. The main philosophy behind this is to restrict the cells the opponent can reach, thus applying pressure and waiting for a mistake.

# Limitations

The biggest problem of the engine is a classic one: the horizon problem. I've done a lot to attempt to minimize the negative effects using certain heuristics such as a bias towards the centre and chasing tails, but these have had limited effects.

# Current Status

The snake is still online at battlesnake.com, and is under the name "Camo"
At its peak, it was #4 in the world in duels, but has since dropped off as other snakes have been created.
