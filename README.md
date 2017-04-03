# Connect Four

Connect Four is a two-player game similar to tic-tac-toe where the object is to get either four Xs or four Os in a horizontal, vertical, or diagonal line. 


<b>Installation</b>

This version of the game runs in the command line and requires Rust, which can be downloaded and installed at https://www.rust-lang.org


To download the game, open your terminal or command line and run: <code>git clone https://github.com/jimmycuadra/connect_four.git</code>

<b>Running the game</b>

Now you need to navigate to the repository in the terminal. If you downloaded the repository directly to your home directory, you can just type: <code>cd connect_four</code>

If you downloaded the repository someplace else—your desktop, for example, or a downloads folder—you would need to navigate to that directory first—cd desktop or cd downloads—before going to the repository.

Once you are in the repository, run: <code>cargo run</code>

You should now see the game’s seven columns in your terminal, along with a prompt for Player X to choose a column in which to drop his piece.

<b>How to play</b>

Player X goes first and types a number from 1-7 to indicate which column they wants to drop their piece into and then hits Return. An X will appear at the bottom of that column. Now it’s Player O’s turn to type a number and press Return. The two players will alternate dropping their pieces until one wins or the game is a draw because there’s no space left in which to drop a piece.

## License

[MIT](http://opensource.org/licenses/MIT)
