==================
Exporting to latex
==================

To export to your game model to latex, use the export_latex() method of the game class.

>>> example_game.export_latex()

.. autofunction:: example_game.export_latex(scale, file_path)

| parameters:
| ``scale:`` An optional scale in float, scaling the final resulting latex figure (default 2.5)
| ``file_path:`` an optional filepath and filename of the output.tex file

Usage:

>>> # Exports the game with scale 1.5 as figure.tex in an output folder
>>> example_game.export_latex(1.5, "output/figure.tex")
>>> game.export_latex() # exports the raw latex code to the terminal

Make sure the output folder is created, or an error will occur.

After creating the latex, further styling can be done manually.
Instead of bogging the function down with optional parameters,
simplicity was favored.

Example:

>>> # Game Configurations
>>> game5_1a = Game(title="Model 1a", player_num=2)
>>> p1, p2 = game5_1a.player[1], game5_1a.player[2]
>>> p1.name = "South Korea"
>>> p2.name = "Japan"
>>> # Creating Utility Variables
>>> s1, s2 = Variable("S_1"), Variable("S_2")
>>> d1, d2 = Variable("D_1"), Variable("D_2")
>>> g, c = Variable("G"), Variable("C")
>>> # setting the players preferences
>>> g > s1 > d1
>>> s2 > c > d2
>>> # Adding nodes
>>> (game5_1a + Decision(p1, "Refrain", variable=(s1, s2))
>>>           + (Decision(p1, "Persist") + Decision(p2, "Punish", variable=(d1, d2)) +
>>>                                        Decision(p2, "Ignore", variable=(g, c))))
>>>
>>> game5_1a.export_latex(1.5, output_dir + "5-1a.tex")

This code will result in latex code that will generate the following:

.. tikz::
   :align: left

   % Style for nodes
    \begin{normalsize}
    \tikzstyle{solid}=[circle,draw,inner sep=1.2,fill=black]
    \tikzstyle{hollow}=[circle,draw,inner sep=1.2]
    % Spacing for every level of the tree
    \tikzstyle{level 1}=[level distance=20mm,sibling distance=20mm]
    \tikzstyle{level 2}=[level distance=20mm,sibling distance=20mm]
    % The Tree
    \node(0)[solid,label=above:{South Korea}]{}child{node(1)[hollow, label=below:{$(S_1, S_2)$}]{}
    edge from parent node[left]{$Refrain$}}child{node(2)[solid, label=above right:{Japan}]{}
    child{node(3)[hollow, label=below:{$(D_1, D_2)$}]{}
    edge from parent node[left]{$Punish$}}child{node(4)[hollow, label=below:{$(G, C)$}]{}
    edge from parent node[right]{$Ignore$}}edge from parent node[right]{$Persist$}};
    \end{normalsize}
