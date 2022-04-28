# Tictactoe min-max AI 0.0.1
Testing out a min-max algo

USAGE:
    tictactoe-ai [FLAGS] [OPTIONS]

FLAGS:
    -h, --help          Prints help information
    -n, --no-verbose    Only print the final line of a game
    -o, --o-ai          Is Player O an AI?
    -V, --version       Prints version information
    -x, --x-ai          Is Player X an AI?

OPTIONS:
        --it <iterations>      How many games to play
        --o-bad <o-bad>        How often should O choose a random move? (0 = never, 1 = 50%, 2 = 33%, etc)
        --o-depth <o-depth>    How deep will O search? (Default: infinite)
        --o-rand <o-rand>      How often should O choose a move just as good? (0 = never, 1 = 50%, 2 = 33%, etc)
        --x-bad <x-bad>        How often should X choose a random move? (0 = never, 1 = 50%, 2 = 33%, etc)
        --x-depth <x-depth>    How deep will X search? (Default: infinite)
        --x-rand <x-rand>      How often should X choose a move just as good? (0 = never, 1 = 50%, 2 = 33%, etc)

