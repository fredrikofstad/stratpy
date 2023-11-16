==================
Exporting to latex
==================

To export to your game model to latex, use the export_latex() method of the game class.

>>> example_game.export_latex()

.. autofunction:: example_game.export_latex(scale, file_path)

| parameters:
| ``scale:`` An optional scale in float, scaling the final resulting latex figure (default 2.5)
| ``file_path:`` an optional filepath and filename of the output.tex file

Example:

>>> # Exports the game with scale 1.5 as figure.tex in an output folder
>>> example_game.export_latex(1.5, "output/figure.tex")
>>> game.export_latex() # exports the raw latex code to the terminal

Make sure the output folder is created, or an error will occur.

After creating the latex, further styling can be done manually.
Instead of bogging the function down with optional parameters,
simplicity was favored.

.. tikz:: Tree test
   :align: left

   % Style for nodes
    \tikzstyle{solid}=[circle,draw,inner sep=1.2,fill=black]
    \tikzstyle{hollow}=[circle,draw,inner sep=1.2]
    % Spacing for every level of the tree
    \tikzstyle{level 1}=[level distance=10mm,sibling distance=25mm]
    \tikzstyle{level 2}=[level distance=15mm,sibling distance=15mm]
    % The Tree
    \node(0)[solid,label=above:{Nature}]{}child{node(11)[solid, label=above left:{}]{}
        child{node(13)[hollow, label=below:{$(S_1, S_2)$}]{}
        edge from parent node[left]{$Refrain$}}child{node(14)[solid, label=above right:{Japan}]{}
        child{node(17)[hollow, label=below:{$(D_1, D_2)$}]{}
        edge from parent node[left]{$Punish$}}edge from parent node[right]{$Refrain$}}edge from parent node[left]{$p$}}child{node(12)[solid, label=above right:{}]{}
        child{node(15)[hollow, label=below:{$(S_1, S_2)$}]{}
        edge from parent node[left]{$Refrain$}}child{node(16)[solid, label=above right:{Japan}]{}
        child{node(18)[hollow, label=below:{$(D_1, D_2)$}]{}
        edge from parent node[left]{$Punish$}}child{node(19)[hollow, label=below:{$(S_1 + G, S_2 - A)$}]{}
        edge from parent node[right]{$Ignore$}}edge from parent node[right]{$Refrain$}}edge from parent node[right]{$1 - p$}};% information set
    \draw[dashed,rounded corners=10]($(11) + (-.1,.125)$)rectangle($(12) +(.1,-.125)$);
    \node at ($(11)!.5!(12)$) {South Korea};