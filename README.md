
*“A great discovery solves a great problem, but there is a grain of discovery in the solution of any problem. Your problem may be modest, but if it challenges your curiosity and brings into play your inventive faculties, and if you solve it by your own means, you may experience the tension and enjoy the triumph of discovery.”*

<small>George Pólya: <em>How to Solve It: A New Aspect of Mathematical Method</em></small>

![Planar embedding of Cube and Discocubes](imgs/planar_emb.png?raw=true "Planar embedding of Cube and Discocubes")
<small><em>Planar embedding of a cube and a discocube. From the set of all graphs G, where the order of G is of the ***Uncentered octahedral numbers*** [A130809](https://oeis.org/A130809), only the first two instances shown above; n[0] and n[1] are planarly embeddable i.e., it can be represented on a two-dimensional surface without any of its edges crossing.</em></small>

An algorithm for solving the Hamiltonian cycle problem deterministically and in linear time on all instances of discocube graphs (tested for graphs with over 8 billion vertices, the world's population). Discocube graphs are
3-dimensional grid graphs derived from: a polycube of an octahedron | a Hauy construction of an octahedron with cubes as identical building blocks | the accretion of cubes around a central cube forming an octahedron at the limit |
the set of points in a 3-dimensional square grid graph contained within an octahedron | a 3d L1-norm unit ball. 

This algorithm is an artist's rendering of his muse, a graph object, using programming as a language (instead of painting flowers and apples or singing hymns about angels) and a means by which to describe his muse's body as an endless contour drawing or,  in graph theory terms, a Hamiltonian cycle.

![Alt text](imgs/elumina-inspiration-one-line-picasso-drawings-the-three-dancers-thumbnail.jpg?version%3D1680306692540)
<small>Pablo Picasso:  <em>Trois Danseuses (The Three Dancers)</em></small>

What links an endless contour line drawing and a Hamiltonian cycle? Both entail tracing a path without interruption, using a continuous line to describe the subject/object to be depicted. An endless contour line drawing is one in which the artist uses a single, uninterrupted line to describe a subject's form and shape.
Similarly, a Hamiltonian cycle describes a graph by tracing a path along edges of the graph, visiting vertex precisely once before returning to the beginning vertex. The path connects the vertices in a continuous line, describing the graph object.

![Alt text](imgs/hambw.png)
<small><em>The graphs from all the platonic solids are Hamiltonian.</em></small>

<div style="display: flex;">
  <div style="flex: 1;">
    <img src="imgs/icosian.jpg" alt="Hauy's wooden crystal models" width="100%">
  </div>
  <div style="flex: 2; padding-left: 20px;">
    <p>
      <em>Named after Sir William Rowan Hamilton, the Hamiltonian cycle problem is a classic graph theory problem solved by finding a closed loop in a graph that visits every node exactly once and ending at the starting point. Hamilton formulated the problem in the rules of his Icosian game, in which players insert numbered pegs into holes on a wooden board to represent steps in a path. The objective is to insert the pegs in order along a path to form a closed loop, much like the Hamiltonian cycle problem where the pegs represent nodes in a graph and the path of inserted pegs represents the cycle.</em>
    </p>
  </div>
</div>

After pages of studies, drawings, and a little math, this algorithm is the result of using the artistic process to solve a mathematical problem without having the means by which to solve it mathematically. A love-child of banging your head against the wall until a hole appears (learning something) and bending the wall with your mind through pure will and imagination (winging it until you fly). When a graph becomes an artist's muse, how does the artist go about rendering their vision as a painter would paint a portrait, making it their own? Can one claim authorship by merely copying a form? If my new material is programming, would functions fill the studio of my mind otherwise filled with machines and material? Will I find patterns in the [music I make from finding solutions?](https://soundcloud.com/ro-yu-489928073/tracks)

An artist manipulates their medium to create forms, using brush strokes to describe how the curve of the neck disappears behind the back, or playing with colors and contrasts to subtly bring the skin of a subject living 500 years ago back to life. For many, the medium is the end to itself, like painting a painting or photographing a photo, rather than the medium being wholly dependent on the concept or idea to be executed. In my artistic practice the medium used for any particular project is dependent on the first finding the right language and then developing it into a visual language. In this project, I studied the discocube visually as a body, imagining each turn not as a discrete mathematical object but as a series of possible movements, as an endlessly iterated dance captured in a long camera exposure, resulting in less equating and more doodling, and wishing I knew more math. The result is a family of algorithms for finding Hamiltonian cycles with varying degrees of refinement (edge distribution), with the weave algorithm producing the least refinement. The other algorithms are concerned with finding an initial Hamiltonian cycle with a higher mutation rate and whose edges are more uniformly spread across the three axes x, y, and z, and through subsequent processes, the solution is polished like a diamond, i.e., the initial tour improved upon until the edges are evenly distributed across all three axes, culminating in an always-turning Hamiltonian cycle. Owing to the regularity and consistency of the solution produced by this determined and predictive algorithm, using weave() to obtain a diamond-grade discocube would take an inconceivable period of time, necessitating the development of other algorithms capable of accomplishing this reasonably.

Why weave? Finding the solution to the problem reminded me of macramé, of tying knots, weaving, and spinning yarn. I thought of how patterns in handwoven fabric are actually unwitting recordings of a knitter's hand movements, like how a piano roll is a recording of the pianist's finger hitting ebony, or how a seismograph records the motion of the earth, or how our skin is a type of recording of our life... I followed the thought further and asked myself: was there a pattern to expose and use to construct the discocube, level by level, similar to how one would knit a scarf, row by row, until the desired result is reached? To illustrate the intention of the code succinctly, I've structured the code to mimic the process of weaving a piece of tapestry, from spinning the yarn to incorporating the weft into the warps.

![First 11 discocubes and their order (number of nodes)](imgs/rect5857.png?raw=true "Discocubes orders")
*The first eleven discocubes and their respective orders (number of nodes)*

To paraphrase Hauy: 

*When solving problems that involve analyzing how nature progresses, we are led by very rapid methods to results that are not immediately obvious. These results may appear paradoxical and surprising. However, if we take the time to carefully examine the steps we took to reach these results, we will begin to understand the underlying principles that led to these outcomes. By going back over the process step by step, we can better understand the logic behind the final results.*

What started as a hack-your-own version of a depth-first search with shortcuts for the discocube graph (solving up to 960 vertices) metastasized into pages of overgrown mixin classes mysteriously coupled to each other like overgrown vines, pushing me deeper and deeper into the underbelly of its mutant tentacles. Although able to solve instances with over a million vertices, the algorithm had the clarity of primordial soup. So, as a sadistic gardener, I painstakingly pruned my own pubicity (my unescapable web of thorny vines) into presentable tiny bonsai trees. So what is a bonsai if not a tree in intimate scope?

The result of this creative process is a family of algorithms developed specifically to solve various graph problems on dodecahedron graphs, 3D grid graphs, and hexprism honeycomb diamond graphs.
The algorithm presented in this repository is the least complex, making it the fastest. It does the job, solving the Hamiltonian cycle problem for over millions of vertices in seconds and graphing with over a billion vertices in less than an hour and a graph with over 8 billion vertices in less than 5 hours, while other algorithms in the family take longer but also have other objectives, like forming an always-turning cycle with even edge distribution across all axes. But that's beyond the scope of this repository.

This algorithm has no while loops and will terminate after a definitive set of steps. The strength of this algorithm is knowing exactly when, where, and what is to happen, thereby reducing the amount of calculations needed (which is surprising as the creative process in creating this was anything but deterministic). It is a construction algorithm, constructing the path layer by layer until loops are produced, which are then joined using cycle merging. Further optimizations of the algorithm have also discarded the memory-heavy adjacency list, choosing instead to perform individual calculations where needed. Making and solving a graph with over a billion vertices, where n = 1000, won't require a distributed graph engine on the cloud anymore, and it takes a little over ten minutes.

### Links:
![NP-Completeness explained](https://youtu.be/ctwX--JEzSA)

![Discocubes](imgs/dcviews.png?raw=true "Discocubes")
*Discocubes 8 - 1760*

![Hexprism Honeycomb Diamond](imgs/hexhoneydiamond.png?raw=true "Hexprism Honeycomb Diamond")
*Hexprism Honeycomb Diamond*

### digital discocubes
As each solution is as unique as a fingerprint, or a diamond it allows one to have their own digital version of a discocube, which is also an instruction for building your own.

![Discocube 3640 view](imgs/icy_cube.png?raw=true "icy cube") 
![Discocube 3640 view](imgs/icy_cube3.png?raw=true "confetti cube")
*Discocubes as glb, using different mirrored texture yields personalized results and unique reflections meaning each discocube has its own reflection/shadow fingerprint! With millions of combinations available (glass texture/image/color, mirror texture/image/color, edge texture/image/color), the possibilities are endless!*

The always turning hamiltonian cycle digital discocubes are not produced by the algorithm in this repository, but by another polynomial-time algorithm.

![Solution to a 79040 node graph](imgs/solution.png)
*Detail for a Hamiltonian cycle for a graph with 79,040 nodes.*

## Command line usage
To use the package via the command line, navigate to the root directory of the project in your terminal and run the following command:
```
cargo run --release [Graph start instance] [Graph end instance]
```
```
cargo run --release 1 100
```
build > run > make > solve > certify > for each graph starting from 32 to 1.373 million vertices.

## Plotting the solution
The solution can be plotted using pandas, numpy and plotly. I've put together an easy to use python module: https://github.com/discocube/plot_solution to plot and very the solution visually instead of only programmatically.

![Very first discocube in Berghain](imgs/ako.png)
*Me and Discocube in Berghain*

## Running times
![Running times from 8 to over 8 billion vertices](imgs/8_to_8billion.png?raw=true "Runtimes 8 to over 8 billion")
<small>Running times from 8 to over 8 billion vertices</small>

#### Running times for graphs with 8 to over 9 billion vertices (solved in under 2 hours):
```
| 🇳    1 | ⭕️          8 | 🕗 0.0013826390 | 
| 🇳    2 | ⭕️         32 | 🕗 0.0000033670 | 
| 🇳    3 | ⭕️         80 | 🕗 0.0000614270 | 
| 🇳    4 | ⭕️        160 | 🕗 0.0000562860 | 
| 🇳    5 | ⭕️        280 | 🕗 0.0000977450 | 
| 🇳    6 | ⭕️        448 | 🕗 0.0000710550 | 
| 🇳    7 | ⭕️        672 | 🕗 0.0000990080 | 
| 🇳    8 | ⭕️        960 | 🕗 0.0001850410 | 
| 🇳    9 | ⭕️       1320 | 🕗 0.0001343750 | 
| 🇳   10 | ⭕️       1760 | 🕗 0.0000918140 | 
| 🇳   11 | ⭕️       2288 | 🕗 0.0001320300 | 
| 🇳   12 | ⭕️       2912 | 🕗 0.0001228330 | 
| 🇳   13 | ⭕️       3640 | 🕗 0.0002028740 | 
| 🇳   14 | ⭕️       4480 | 🕗 0.0001844800 | 
| 🇳   15 | ⭕️       5440 | 🕗 0.0001880170 | 
| 🇳   16 | ⭕️       6528 | 🕗 0.0002569970 | 
| 🇳   17 | ⭕️       7752 | 🕗 0.0002698410 | 
| 🇳   18 | ⭕️       9120 | 🕗 0.0003315880 | 
| 🇳   19 | ⭕️      10640 | 🕗 0.0003165100 | 
| 🇳   20 | ⭕️      12320 | 🕗 0.0003873640 | 
| 🇳   21 | ⭕️      14168 | 🕗 0.0004467160 | 
| 🇳   22 | ⭕️      16192 | 🕗 0.0004656830 | 
| 🇳   23 | ⭕️      18400 | 🕗 0.0005002280 | 
| 🇳   24 | ⭕️      20800 | 🕗 0.0006206960 | 
| 🇳   25 | ⭕️      23400 | 🕗 0.0007041440 | 
| 🇳   26 | ⭕️      26208 | 🕗 0.0007454930 | 
| 🇳   27 | ⭕️      29232 | 🕗 0.0008369460 | 
| 🇳   28 | ⭕️      32480 | 🕗 0.0007692680 | 
| 🇳   29 | ⭕️      35960 | 🕗 0.0010591570 | 
| 🇳   30 | ⭕️      39680 | 🕗 0.0011221260 | 
| 🇳   31 | ⭕️      43648 | 🕗 0.0012309219 | 
| 🇳   32 | ⭕️      47872 | 🕗 0.0013338469 | 
| 🇳   33 | ⭕️      52360 | 🕗 0.0014716280 | 
| 🇳   34 | ⭕️      57120 | 🕗 0.0022411051 | 
| 🇳   35 | ⭕️      62160 | 🕗 0.0018336640 | 
| 🇳   36 | ⭕️      67488 | 🕗 0.0016571690 | 
| 🇳   37 | ⭕️      73112 | 🕗 0.0021818129 | 
| 🇳   38 | ⭕️      79040 | 🕗 0.0023685270 | 
| 🇳   39 | ⭕️      85280 | 🕗 0.0029147810 | 
| 🇳   40 | ⭕️      91840 | 🕗 0.0027667009 | 
| 🇳   41 | ⭕️      98728 | 🕗 0.0029103830 | 
| 🇳   42 | ⭕️     105952 | 🕗 0.0031225050 | 
| 🇳   43 | ⭕️     113520 | 🕗 0.0039126719 | 
| 🇳   44 | ⭕️     121440 | 🕗 0.0039667338 | 
| 🇳   45 | ⭕️     129720 | 🕗 0.0033332449 | 
| 🇳   46 | ⭕️     138368 | 🕗 0.0041362960 | 
| 🇳   47 | ⭕️     147392 | 🕗 0.0044866591 | 
| 🇳   48 | ⭕️     156800 | 🕗 0.0059593292 | 
| 🇳   49 | ⭕️     166600 | 🕗 0.0052569988 | 
| 🇳   50 | ⭕️     176800 | 🕗 0.0055113100 | 
| 🇳   51 | ⭕️     187408 | 🕗 0.0056320592 | 
| 🇳   52 | ⭕️     198432 | 🕗 0.0067745540 | 
| 🇳   53 | ⭕️     209880 | 🕗 0.0068207099 | 
| 🇳   54 | ⭕️     221760 | 🕗 0.0076169488 | 
| 🇳   55 | ⭕️     234080 | 🕗 0.0079567730 | 
| 🇳   56 | ⭕️     246848 | 🕗 0.0070261802 | 
| 🇳   57 | ⭕️     260072 | 🕗 0.0083173262 | 
| 🇳   58 | ⭕️     273760 | 🕗 0.0089565674 | 
| 🇳   59 | ⭕️     287920 | 🕗 0.0093969814 | 
| 🇳   60 | ⭕️     302560 | 🕗 0.0100456905 | 
| 🇳   61 | ⭕️     317688 | 🕗 0.0114625338 | 
| 🇳   62 | ⭕️     333312 | 🕗 0.0109217586 | 
| 🇳   63 | ⭕️     349440 | 🕗 0.0123157697 | 
| 🇳   64 | ⭕️     366080 | 🕗 0.0129186623 | 
| 🇳   65 | ⭕️     383240 | 🕗 0.0133856675 | 
| 🇳   66 | ⭕️     400928 | 🕗 0.0134386374 | 
| 🇳   67 | ⭕️     419152 | 🕗 0.0149184512 | 
| 🇳   68 | ⭕️     437920 | 🕗 0.0149342101 | 
| 🇳   69 | ⭕️     457240 | 🕗 0.0159061421 | 
| 🇳   70 | ⭕️     477120 | 🕗 0.0168236196 | 
| 🇳   71 | ⭕️     497568 | 🕗 0.0158135761 | 
| 🇳   72 | ⭕️     518592 | 🕗 0.0182943065 | 
| 🇳   73 | ⭕️     540200 | 🕗 0.0198891666 | 
| 🇳   74 | ⭕️     562400 | 🕗 0.0205049831 | 
| 🇳   75 | ⭕️     585200 | 🕗 0.0210833997 | 
| 🇳   76 | ⭕️     608608 | 🕗 0.0211512279 | 
| 🇳   77 | ⭕️     632632 | 🕗 0.0235105567 | 
| 🇳   78 | ⭕️     657280 | 🕗 0.0242030285 | 
| 🇳   79 | ⭕️     682560 | 🕗 0.0257073808 | 
| 🇳   80 | ⭕️     708480 | 🕗 0.0268503632 | 
| 🇳   81 | ⭕️     735048 | 🕗 0.0277219154 | 
| 🇳   82 | ⭕️     762272 | 🕗 0.0293443687 | 
| 🇳   83 | ⭕️     790160 | 🕗 0.0303378720 | 
| 🇳   84 | ⭕️     818720 | 🕗 0.0314716287 | 
| 🇳   85 | ⭕️     847960 | 🕗 0.0333153717 | 
| 🇳   86 | ⭕️     877888 | 🕗 0.0370478109 | 
| 🇳   87 | ⭕️     908512 | 🕗 0.0389449447 | 
| 🇳   88 | ⭕️     939840 | 🕗 0.0391761325 | 
| 🇳   89 | ⭕️     971880 | 🕗 0.0382880606 | 
| 🇳   90 | ⭕️    1004640 | 🕗 0.0394676253 | 
| 🇳   91 | ⭕️    1038128 | 🕗 0.0413489603 | 
| 🇳   92 | ⭕️    1072352 | 🕗 0.0428150967 | 
| 🇳   93 | ⭕️    1107320 | 🕗 0.0443020724 | 
| 🇳   94 | ⭕️    1143040 | 🕗 0.0463217683 | 
| 🇳   95 | ⭕️    1179520 | 🕗 0.0484620035 | 
| 🇳   96 | ⭕️    1216768 | 🕗 0.0499678627 | 
| 🇳   97 | ⭕️    1254792 | 🕗 0.0525957234 | 
| 🇳   98 | ⭕️    1293600 | 🕗 0.0535336509 | 
| 🇳   99 | ⭕️    1333200 | 🕗 0.0558778644 | 
| 🇳  100 | ⭕️    1373600 | 🕗 0.0584105998 | 
| 🇳  101 | ⭕️    1414808 | 🕗 0.0608741567 | 
| 🇳  102 | ⭕️    1456832 | 🕗 0.0625775084 | 
| 🇳  103 | ⭕️    1499680 | 🕗 0.0650352836 | 
| 🇳  104 | ⭕️    1543360 | 🕗 0.0671452209 | 
| 🇳  105 | ⭕️    1587880 | 🕗 0.0693750381 | 
| 🇳  106 | ⭕️    1633248 | 🕗 0.0724435151 | 
| 🇳  107 | ⭕️    1679472 | 🕗 0.0783749223 | 
| 🇳  108 | ⭕️    1726560 | 🕗 0.0773797855 | 
| 🇳  109 | ⭕️    1774520 | 🕗 0.0789434463 | 
| 🇳  110 | ⭕️    1823360 | 🕗 0.0824571997 | 
| 🇳  111 | ⭕️    1873088 | 🕗 0.0859410912 | 
| 🇳  112 | ⭕️    1923712 | 🕗 0.0879727229 | 
| 🇳  113 | ⭕️    1975240 | 🕗 0.0928090289 | 
| 🇳  114 | ⭕️    2027680 | 🕗 0.0942272767 | 
| 🇳  115 | ⭕️    2081040 | 🕗 0.0977791846 | 
| 🇳  116 | ⭕️    2135328 | 🕗 0.1012796536 | 
| 🇳  117 | ⭕️    2190552 | 🕗 0.1072939634 | 
| 🇳  118 | ⭕️    2246720 | 🕗 0.1110917255 | 
| 🇳  119 | ⭕️    2303840 | 🕗 0.1152364016 | 
| 🇳  120 | ⭕️    2361920 | 🕗 0.1183052659 | 
| 🇳  121 | ⭕️    2420968 | 🕗 0.1224953979 | 
| 🇳  122 | ⭕️    2480992 | 🕗 0.1259232759 | 
| 🇳  123 | ⭕️    2542000 | 🕗 0.1327860057 | 
| 🇳  124 | ⭕️    2604000 | 🕗 0.1386287063 | 
| 🇳  125 | ⭕️    2667000 | 🕗 0.1428631693 | 
| 🇳  126 | ⭕️    2731008 | 🕗 0.1472443640 | 
| 🇳  127 | ⭕️    2796032 | 🕗 0.1515980214 | 
| 🇳  128 | ⭕️    2862080 | 🕗 0.1645432115 | 
| 🇳  129 | ⭕️    2929160 | 🕗 0.1619233787 | 
| 🇳  130 | ⭕️    2997280 | 🕗 0.1706316918 | 
| 🇳  131 | ⭕️    3066448 | 🕗 0.1746450812 | 
| 🇳  132 | ⭕️    3136672 | 🕗 0.1779252738 | 
| 🇳  133 | ⭕️    3207960 | 🕗 0.1927954853 | 
| 🇳  134 | ⭕️    3280320 | 🕗 0.1925167143 | 
| 🇳  135 | ⭕️    3353760 | 🕗 0.2030750662 | 
| 🇳  136 | ⭕️    3428288 | 🕗 0.2283407897 | 
| 🇳  137 | ⭕️    3503912 | 🕗 0.2195300609 | 
| 🇳  138 | ⭕️    3580640 | 🕗 0.2305969894 | 
| 🇳  139 | ⭕️    3658480 | 🕗 0.2313528657 | 
| 🇳  140 | ⭕️    3737440 | 🕗 0.2394423783 | 
| 🇳  141 | ⭕️    3817528 | 🕗 0.2491001934 | 
| 🇳  142 | ⭕️    3898752 | 🕗 0.2575227618 | 
| 🇳  143 | ⭕️    3981120 | 🕗 0.2609031200 | 
| 🇳  144 | ⭕️    4064640 | 🕗 0.2720794976 | 
| 🇳  145 | ⭕️    4149320 | 🕗 0.2815293074 | 
| 🇳  146 | ⭕️    4235168 | 🕗 0.2901331186 | 
| 🇳  147 | ⭕️    4322192 | 🕗 0.3000857830 | 
| 🇳  148 | ⭕️    4410400 | 🕗 0.3129457235 | 
| 🇳  149 | ⭕️    4499800 | 🕗 0.3267289996 | 
| 🇳  150 | ⭕️    4590400 | 🕗 0.3270561695 | 
| 🇳  151 | ⭕️    4682208 | 🕗 0.3346073925 | 
| 🇳  152 | ⭕️    4775232 | 🕗 0.3408940434 | 
| 🇳  153 | ⭕️    4869480 | 🕗 0.3489655554 | 
| 🇳  154 | ⭕️    4964960 | 🕗 0.3616656661 | 
| 🇳  155 | ⭕️    5061680 | 🕗 0.3684297800 | 
| 🇳  156 | ⭕️    5159648 | 🕗 0.3885279000 | 
| 🇳  157 | ⭕️    5258872 | 🕗 0.3873851299 | 
| 🇳  158 | ⭕️    5359360 | 🕗 0.3986618221 | 
| 🇳  159 | ⭕️    5461120 | 🕗 0.4080869555 | 
| 🇳  160 | ⭕️    5564160 | 🕗 0.4221595824 | 
| 🇳  161 | ⭕️    5668488 | 🕗 0.4315628707 | 
| 🇳  162 | ⭕️    5774112 | 🕗 0.4632537365 | 
| 🇳  163 | ⭕️    5881040 | 🕗 0.4641559124 | 
| 🇳  164 | ⭕️    5989280 | 🕗 0.4688872099 | 
| 🇳  165 | ⭕️    6098840 | 🕗 0.4742944241 | 
| 🇳  166 | ⭕️    6209728 | 🕗 0.5080329776 | 
| 🇳  167 | ⭕️    6321952 | 🕗 0.4972271621 | 
| 🇳  168 | ⭕️    6435520 | 🕗 0.5176514983 | 
| 🇳  169 | ⭕️    6550440 | 🕗 0.5269491673 | 
| 🇳  170 | ⭕️    6666720 | 🕗 0.5267423987 | 
| 🇳  171 | ⭕️    6784368 | 🕗 0.5443713069 | 
| 🇳  172 | ⭕️    6903392 | 🕗 0.5685918927 | 
| 🇳  173 | ⭕️    7023800 | 🕗 0.5912855864 | 
| 🇳  174 | ⭕️    7145600 | 🕗 0.5968688726 | 
| 🇳  175 | ⭕️    7268800 | 🕗 0.6012613773 | 
| 🇳  176 | ⭕️    7393408 | 🕗 0.6188448071 | 
| 🇳  177 | ⭕️    7519432 | 🕗 0.6296640635 | 
| 🇳  178 | ⭕️    7646880 | 🕗 0.6459178925 | 
| 🇳  179 | ⭕️    7775760 | 🕗 0.6719653606 | 
| 🇳  180 | ⭕️    7906080 | 🕗 0.6983801126 | 
| 🇳  181 | ⭕️    8037848 | 🕗 0.7477512360 | 
| 🇳  182 | ⭕️    8171072 | 🕗 0.7238599062 | 
| 🇳  183 | ⭕️    8305760 | 🕗 0.7213981152 | 
| 🇳  184 | ⭕️    8441920 | 🕗 0.7452818155 | 
| 🇳  185 | ⭕️    8579560 | 🕗 0.7602891326 | 
| 🇳  186 | ⭕️    8718688 | 🕗 0.7729542851 | 
| 🇳  187 | ⭕️    8859312 | 🕗 0.7877422571 | 
| 🇳  188 | ⭕️    9001440 | 🕗 0.7986281514 | 
| 🇳  189 | ⭕️    9145080 | 🕗 0.8040807247 | 
| 🇳  190 | ⭕️    9290240 | 🕗 0.8653299809 | 
| 🇳  191 | ⭕️    9436928 | 🕗 0.8680737019 | 
| 🇳  192 | ⭕️    9585152 | 🕗 0.8680514097 | 
| 🇳  193 | ⭕️    9734920 | 🕗 0.9149693251 | 
| 🇳  194 | ⭕️    9886240 | 🕗 0.9180377722 | 
| 🇳  195 | ⭕️   10039120 | 🕗 0.9145927429 | 
| 🇳  196 | ⭕️   10193568 | 🕗 0.9382410645 | 
| 🇳  197 | ⭕️   10349592 | 🕗 0.9585549831 | 
| 🇳  198 | ⭕️   10507200 | 🕗 0.9846464992 | 
| 🇳  199 | ⭕️   10666400 | 🕗 1.0138647556 | 
| 🇳  200 | ⭕️   10827200 | 🕗 1.0235466957 | 
| 🇳  201 | ⭕️   10989608 | 🕗 1.0419590473 | 
| 🇳  202 | ⭕️   11153632 | 🕗 1.0532495975 | 
| 🇳  203 | ⭕️   11319280 | 🕗 1.0645891428 | 
| 🇳  204 | ⭕️   11486560 | 🕗 1.1060914993 | 
| 🇳  205 | ⭕️   11655480 | 🕗 1.1238955259 | 
| 🇳  206 | ⭕️   11826048 | 🕗 1.1372570992 | 
| 🇳  207 | ⭕️   11998272 | 🕗 1.1662840843 | 
| 🇳  208 | ⭕️   12172160 | 🕗 1.1750454903 | 
| 🇳  209 | ⭕️   12347720 | 🕗 1.1938279867 | 
| 🇳  210 | ⭕️   12524960 | 🕗 1.2248406410 | 
| 🇳  211 | ⭕️   12703888 | 🕗 1.2385377884 | 
| 🇳  212 | ⭕️   12884512 | 🕗 1.2811425924 | 
| 🇳  213 | ⭕️   13066840 | 🕗 1.3415226936 | 
| 🇳  214 | ⭕️   13250880 | 🕗 1.3213586807 | 
| 🇳  215 | ⭕️   13436640 | 🕗 1.3648306131 | 
| 🇳  216 | ⭕️   13624128 | 🕗 1.3950002193 | 
| 🇳  217 | ⭕️   13813352 | 🕗 1.3847416639 | 
| 🇳  218 | ⭕️   14004320 | 🕗 1.5492830276 | 
| 🇳  219 | ⭕️   14197040 | 🕗 1.4733085632 | 
| 🇳  220 | ⭕️   14391520 | 🕗 1.4709932804 | 
| 🇳  221 | ⭕️   14587768 | 🕗 1.4947681427 | 
| 🇳  222 | ⭕️   14785792 | 🕗 1.5316812992 | 
| 🇳  223 | ⭕️   14985600 | 🕗 1.5293519497 | 
| 🇳  224 | ⭕️   15187200 | 🕗 1.5741047859 | 
| 🇳  225 | ⭕️   15390600 | 🕗 1.6619278193 | 
| 🇳  226 | ⭕️   15595808 | 🕗 1.6947249174 | 
| 🇳  227 | ⭕️   15802832 | 🕗 1.6594927311 | 
| 🇳  228 | ⭕️   16011680 | 🕗 1.6917359829 | 
| 🇳  229 | ⭕️   16222360 | 🕗 1.7144649029 | 
| 🇳  230 | ⭕️   16434880 | 🕗 1.7248106003 | 
| 🇳  231 | ⭕️   16649248 | 🕗 1.7601759434 | 
| 🇳  232 | ⭕️   16865472 | 🕗 1.8098818064 | 
| 🇳  233 | ⭕️   17083560 | 🕗 1.8060170412 | 
| 🇳  234 | ⭕️   17303520 | 🕗 1.8467803001 | 
| 🇳  235 | ⭕️   17525360 | 🕗 1.8661653996 | 
| 🇳  236 | ⭕️   17749088 | 🕗 1.9097669125 | 
| 🇳  237 | ⭕️   17974712 | 🕗 1.9980155230 | 
| 🇳  238 | ⭕️   18202240 | 🕗 2.0026676655 | 
| 🇳  239 | ⭕️   18431680 | 🕗 2.0125319958 | 
| 🇳  240 | ⭕️   18663040 | 🕗 2.0496003628 | 
| 🇳  241 | ⭕️   18896328 | 🕗 2.0945956707 | 
| 🇳  242 | ⭕️   19131552 | 🕗 2.1428785324 | 
| 🇳  243 | ⭕️   19368720 | 🕗 2.2058691978 | 
| 🇳  244 | ⭕️   19607840 | 🕗 2.1772437096 | 
| 🇳  245 | ⭕️   19848920 | 🕗 2.3052303791 | 
| 🇳  246 | ⭕️   20091968 | 🕗 2.3189456463 | 
| 🇳  247 | ⭕️   20336992 | 🕗 2.3011345863 | 
| 🇳  248 | ⭕️   20584000 | 🕗 2.3204648495 | 
| 🇳  249 | ⭕️   20833000 | 🕗 2.3558764458 | 
| 🇳  250 | ⭕️   21084000 | 🕗 2.4247236252 | 
| 🇳  251 | ⭕️   21337008 | 🕗 2.4059441090 | 
| 🇳  252 | ⭕️   21592032 | 🕗 2.5639183521 | 
| 🇳  253 | ⭕️   21849080 | 🕗 2.6043922901 | 
| 🇳  254 | ⭕️   22108160 | 🕗 2.6231138706 | 
| 🇳  255 | ⭕️   22369280 | 🕗 2.5484957695 | 
| 🇳  256 | ⭕️   22632448 | 🕗 2.6487889290 | 
| 🇳  257 | ⭕️   22897672 | 🕗 2.6668756008 | 
| 🇳  258 | ⭕️   23164960 | 🕗 2.8012568951 | 
| 🇳  259 | ⭕️   23434320 | 🕗 2.8194236755 | 
| 🇳  260 | ⭕️   23705760 | 🕗 2.8554470539 | 
| 🇳  261 | ⭕️   23979288 | 🕗 2.9307649136 | 
| 🇳  262 | ⭕️   24254912 | 🕗 2.8403921127 | 
| 🇳  263 | ⭕️   24532640 | 🕗 2.9430685043 | 
| 🇳  264 | ⭕️   24812480 | 🕗 2.9280705452 | 
| 🇳  265 | ⭕️   25094440 | 🕗 3.1181826591 | 
| 🇳  266 | ⭕️   25378528 | 🕗 3.0550413132 | 
| 🇳  267 | ⭕️   25664752 | 🕗 3.0941684246 | 
| 🇳  268 | ⭕️   25953120 | 🕗 3.1495420933 | 
| 🇳  269 | ⭕️   26243640 | 🕗 3.3661539555 | 
| 🇳  270 | ⭕️   26536320 | 🕗 3.4297580719 | 
| 🇳  271 | ⭕️   26831168 | 🕗 3.5791769028 | 
| 🇳  272 | ⭕️   27128192 | 🕗 4.1912717819 | 
| 🇳  273 | ⭕️   27427400 | 🕗 4.5756549835 | 
| 🇳  274 | ⭕️   27728800 | 🕗 4.5900640488 | 
| 🇳  275 | ⭕️   28032400 | 🕗 4.5449180603 | 
| 🇳  276 | ⭕️   28338208 | 🕗 4.7158527374 | 
| 🇳  277 | ⭕️   28646232 | 🕗 4.7334012985 | 
| 🇳  278 | ⭕️   28956480 | 🕗 4.7770695686 | 
| 🇳  279 | ⭕️   29268960 | 🕗 4.8985295296 | 
| 🇳  280 | ⭕️   29583680 | 🕗 4.8893847466 | 
| 🇳  281 | ⭕️   29900648 | 🕗 4.9792623520 | 
| 🇳  282 | ⭕️   30219872 | 🕗 5.1173505783 | 
| 🇳  283 | ⭕️   30541360 | 🕗 5.2426838875 | 
| 🇳  284 | ⭕️   30865120 | 🕗 5.3007068634 | 
| 🇳  285 | ⭕️   31191160 | 🕗 5.4370975494 | 
| 🇳  286 | ⭕️   31519488 | 🕗 5.3199300766 | 
| 🇳  287 | ⭕️   31850112 | 🕗 5.5194640160 | 
| 🇳  288 | ⭕️   32183040 | 🕗 5.4685339928 | 
| 🇳  289 | ⭕️   32518280 | 🕗 5.5879383087 | 
| 🇳  290 | ⭕️   32855840 | 🕗 5.7158784866 | 
| 🇳  291 | ⭕️   33195728 | 🕗 5.7357177734 | 
| 🇳  292 | ⭕️   33537952 | 🕗 5.8288788795 | 
| 🇳  293 | ⭕️   33882520 | 🕗 5.8788232803 | 
| 🇳  294 | ⭕️   34229440 | 🕗 5.9519829750 | 
| 🇳  295 | ⭕️   34578720 | 🕗 6.0291290283 | 
| 🇳  296 | ⭕️   34930368 | 🕗 6.4236969948 | 
| 🇳  297 | ⭕️   35284392 | 🕗 6.4383215904 | 
| 🇳  298 | ⭕️   35640800 | 🕗 6.3161425591 | 
| 🇳  299 | ⭕️   35999600 | 🕗 6.2798218727 | 
| 🇳  300 | ⭕️   36360800 | 🕗 6.7889757156 | 
| 🇳  301 | ⭕️   36724408 | 🕗 6.6955175400 | 
| 🇳  302 | ⭕️   37090432 | 🕗 6.8109006882 | 
| 🇳  303 | ⭕️   37458880 | 🕗 6.6744613647 | 
| 🇳  304 | ⭕️   37829760 | 🕗 6.7507023811 | 
| 🇳  305 | ⭕️   38203080 | 🕗 6.8174371719 | 
| 🇳  306 | ⭕️   38578848 | 🕗 6.9526643753 | 
| 🇳  307 | ⭕️   38957072 | 🕗 6.9449038506 | 
| 🇳  308 | ⭕️   39337760 | 🕗 7.1995759010 | 
| 🇳  309 | ⭕️   39720920 | 🕗 7.5899724960 | 
| 🇳  310 | ⭕️   40106560 | 🕗 7.6626482010 | 
| 🇳  311 | ⭕️   40494688 | 🕗 7.4772958755 | 
| 🇳  312 | ⭕️   40885312 | 🕗 7.6936206818 | 
| 🇳  313 | ⭕️   41278440 | 🕗 7.6538476944 | 
| 🇳  314 | ⭕️   41674080 | 🕗 8.1868600845 | 
| 🇳  315 | ⭕️   42072240 | 🕗 8.1205129623 | 
| 🇳  316 | ⭕️   42472928 | 🕗 7.9303870201 | 
| 🇳  317 | ⭕️   42876152 | 🕗 8.3430938721 | 
| 🇳  318 | ⭕️   43281920 | 🕗 8.2908372879 | 
| 🇳  319 | ⭕️   43690240 | 🕗 8.5183401108 | 
| 🇳  320 | ⭕️   44101120 | 🕗 8.5323276520 | 
| 🇳  321 | ⭕️   44514568 | 🕗 8.8245182037 | 
| 🇳  322 | ⭕️   44930592 | 🕗 8.7549905777 | 
| 🇳  323 | ⭕️   45349200 | 🕗 8.8928298950 | 
| 🇳  324 | ⭕️   45770400 | 🕗 8.7252101898 | 
| 🇳  325 | ⭕️   46194200 | 🕗 8.6704149246 | 
| 🇳  326 | ⭕️   46620608 | 🕗 9.1117362976 | 
| 🇳  327 | ⭕️   47049632 | 🕗 9.1388578415 | 
| 🇳  328 | ⭕️   47481280 | 🕗 9.2714519501 | 
| 🇳  329 | ⭕️   47915560 | 🕗 9.3886718750 | 
| 🇳  330 | ⭕️   48352480 | 🕗 9.6324033737 | 
| 🇳  331 | ⭕️   48792048 | 🕗 9.4725704193 | 
| 🇳  332 | ⭕️   49234272 | 🕗 9.7474699020 | 
| 🇳  333 | ⭕️   49679160 | 🕗 9.9348745346 | 
| 🇳  334 | ⭕️   50126720 | 🕗 10.0972003937 |
| 🇳  335 | ⭕️   50576960 | 🕗 9.8467826843 | 
| 🇳  336 | ⭕️   51029888 | 🕗 10.2159261703 |
| 🇳  337 | ⭕️   51485512 | 🕗 10.5041999817 |
| 🇳  338 | ⭕️   51943840 | 🕗 10.4478178024 |
| 🇳  339 | ⭕️   52404880 | 🕗 10.4992265701 |
| 🇳  340 | ⭕️   52868640 | 🕗 10.5768079758 |
| 🇳  341 | ⭕️   53335128 | 🕗 10.6925163269 |
| 🇳  342 | ⭕️   53804352 | 🕗 10.6647596359 |
| 🇳  343 | ⭕️   54276320 | 🕗 11.0027532578 |
| 🇳  344 | ⭕️   54751040 | 🕗 11.2498540878 |
| 🇳  345 | ⭕️   55228520 | 🕗 11.3140449524 |
| 🇳  346 | ⭕️   55708768 | 🕗 11.6855001450 |
| 🇳  347 | ⭕️   56191792 | 🕗 11.7512941360 |
| 🇳  348 | ⭕️   56677600 | 🕗 11.7160339355 |
| 🇳  349 | ⭕️   57166200 | 🕗 11.7622117996 |
| 🇳  350 | ⭕️   57657600 | 🕗 11.6369724274 |
| 🇳  351 | ⭕️   58151808 | 🕗 11.8944845200 |
| 🇳  352 | ⭕️   58648832 | 🕗 12.0310468674 |
| 🇳  353 | ⭕️   59148680 | 🕗 12.3611040115 |
| 🇳  354 | ⭕️   59651360 | 🕗 12.4496126175 |
| 🇳  355 | ⭕️   60156880 | 🕗 12.4086904526 |
| 🇳  356 | ⭕️   60665248 | 🕗 12.7565517426 |
| 🇳  357 | ⭕️   61176472 | 🕗 13.0207557678 |
| 🇳  358 | ⭕️   61690560 | 🕗 13.0674562454 |
| 🇳  359 | ⭕️   62207520 | 🕗 12.9388046265 |
| 🇳  360 | ⭕️   62727360 | 🕗 12.9353084564 |
| 🇳  361 | ⭕️   63250088 | 🕗 10.7965621948 |
| 🇳  362 | ⭕️   63775712 | 🕗 13.8567686081 |
| 🇳  363 | ⭕️   64304240 | 🕗 10.1988582611 |
| 🇳  364 | ⭕️   64835680 | 🕗 10.2771816254 |
| 🇳  365 | ⭕️   65370040 | 🕗 10.4544677734 |
| 🇳  366 | ⭕️   65907328 | 🕗 10.3414106369 |
| 🇳  367 | ⭕️   66447552 | 🕗 10.3983774185 |
| 🇳  368 | ⭕️   66990720 | 🕗 10.2743091583 |
| 🇳  369 | ⭕️   67536840 | 🕗 10.5985984802 |
| 🇳  370 | ⭕️   68085920 | 🕗 10.7767019272 |
| 🇳  371 | ⭕️   68637968 | 🕗 10.8243217468 |
| 🇳  372 | ⭕️   69192992 | 🕗 10.9180707932 |
| 🇳  373 | ⭕️   69751000 | 🕗 10.9447994232 |
| 🇳  374 | ⭕️   70312000 | 🕗 11.1755752563 |
| 🇳  375 | ⭕️   70876000 | 🕗 11.3431100845 |
| 🇳  376 | ⭕️   71443008 | 🕗 12.0111923218 |
| 🇳  377 | ⭕️   72013032 | 🕗 11.9959735870 |
| 🇳  378 | ⭕️   72586080 | 🕗 12.0184202194 |
| 🇳  379 | ⭕️   73162160 | 🕗 12.1744956970 |
| 🇳  380 | ⭕️   73741280 | 🕗 12.0333271027 |
| 🇳  381 | ⭕️   74323448 | 🕗 12.0676298141 |
| 🇳  382 | ⭕️   74908672 | 🕗 11.9504508972 |
| 🇳  383 | ⭕️   75496960 | 🕗 11.9967584610 |
| 🇳  384 | ⭕️   76088320 | 🕗 12.4387989044 |
| 🇳  385 | ⭕️   76682760 | 🕗 12.8964834213 |
| 🇳  386 | ⭕️   77280288 | 🕗 12.5736808777 |
| 🇳  387 | ⭕️   77880912 | 🕗 12.7665252686 |
| 🇳  388 | ⭕️   78484640 | 🕗 12.6689815521 |
| 🇳  389 | ⭕️   79091480 | 🕗 13.0222349167 |
| 🇳  390 | ⭕️   79701440 | 🕗 13.0217599869 |
| 🇳  391 | ⭕️   80314528 | 🕗 12.9892301559 |
| 🇳  392 | ⭕️   80930752 | 🕗 13.1229553223 |
| 🇳  393 | ⭕️   81550120 | 🕗 13.5700082779 |
| 🇳  394 | ⭕️   82172640 | 🕗 13.6101007462 |
| 🇳  395 | ⭕️   82798320 | 🕗 13.7113618851 |
| 🇳  396 | ⭕️   83427168 | 🕗 13.9983978271 |
| 🇳  397 | ⭕️   84059192 | 🕗 13.9344682693 |
| 🇳  398 | ⭕️   84694400 | 🕗 14.2043380737 |
| 🇳  399 | ⭕️   85332800 | 🕗 14.2377338409 |
| 🇳  400 | ⭕️   85974400 | 🕗 14.4652614594 |
| 🇳  401 | ⭕️   86619208 | 🕗 14.6931371689 |
| 🇳  402 | ⭕️   87267232 | 🕗 14.8751306534 |
| 🇳  403 | ⭕️   87918480 | 🕗 14.8265619278 |
| 🇳  404 | ⭕️   88572960 | 🕗 14.9483823776 |
| 🇳  405 | ⭕️   89230680 | 🕗 15.1757059097 |
| 🇳  406 | ⭕️   89891648 | 🕗 15.1946620941 |
| 🇳  407 | ⭕️   90555872 | 🕗 15.7824287415 |
| 🇳  408 | ⭕️   91223360 | 🕗 15.5478982925 |
| 🇳  409 | ⭕️   91894120 | 🕗 15.8480682373 |
| 🇳  410 | ⭕️   92568160 | 🕗 15.9264831543 |
| 🇳  411 | ⭕️   93245488 | 🕗 16.0316524506 |
| 🇳  412 | ⭕️   93926112 | 🕗 15.9967460632 |
| 🇳  413 | ⭕️   94610040 | 🕗 16.4165363312 |
| 🇳  414 | ⭕️   95297280 | 🕗 16.4590816498 |
| 🇳  415 | ⭕️   95987840 | 🕗 16.6953544617 |
| 🇳  416 | ⭕️   96681728 | 🕗 16.3148574829 |
| 🇳  417 | ⭕️   97378952 | 🕗 16.5944099426 |
| 🇳  418 | ⭕️   98079520 | 🕗 17.2510967255 |
| 🇳  419 | ⭕️   98783440 | 🕗 17.1849403381 |
| 🇳  420 | ⭕️   99490720 | 🕗 17.1741847992 |
| 🇳  421 | ⭕️  100201368 | 🕗 17.4250850677 |
| 🇳  422 | ⭕️  100915392 | 🕗 17.7100639343 |
| 🇳  423 | ⭕️  101632800 | 🕗 17.9012031555 |
| 🇳  424 | ⭕️  102353600 | 🕗 18.6000957489 |
| 🇳  425 | ⭕️  103077800 | 🕗 18.3186702728 |
| 🇳  426 | ⭕️  103805408 | 🕗 18.3362922668 |
| 🇳  427 | ⭕️  104536432 | 🕗 18.6132698059 |
| 🇳  428 | ⭕️  105270880 | 🕗 19.0045089722 |
| 🇳  429 | ⭕️  106008760 | 🕗 18.8665103912 |
| 🇳  430 | ⭕️  106750080 | 🕗 19.1278896332 |
| 🇳  431 | ⭕️  107494848 | 🕗 18.5327587128 |
| 🇳  432 | ⭕️  108243072 | 🕗 18.7094211578 |
| 🇳  433 | ⭕️  108994760 | 🕗 18.7990989685 |
| 🇳  434 | ⭕️  109749920 | 🕗 19.4284610748 |
| 🇳  435 | ⭕️  110508560 | 🕗 19.8374423981 |
| 🇳  436 | ⭕️  111270688 | 🕗 19.6444931030 |
| 🇳  437 | ⭕️  112036312 | 🕗 20.0687122345 |
| 🇳  438 | ⭕️  112805440 | 🕗 19.9333114624 |
| 🇳  439 | ⭕️  113578080 | 🕗 20.4954319000 |
| 🇳  440 | ⭕️  114354240 | 🕗 21.1962871552 |
| 🇳  441 | ⭕️  115133928 | 🕗 21.2375278473 |
| 🇳  442 | ⭕️  115917152 | 🕗 21.4183235168 |
| 🇳  443 | ⭕️  116703920 | 🕗 21.0277786255 |
| 🇳  444 | ⭕️  117494240 | 🕗 21.5785255432 |
| 🇳  445 | ⭕️  118288120 | 🕗 21.6548614502 |
| 🇳  446 | ⭕️  119085568 | 🕗 21.7635726929 |
| 🇳  447 | ⭕️  119886592 | 🕗 21.7913055420 |
| 🇳  448 | ⭕️  120691200 | 🕗 22.1091384888 |
| 🇳  449 | ⭕️  121499400 | 🕗 22.2276477814 |
| 🇳  450 | ⭕️  122311200 | 🕗 22.6669216156 |
| 🇳  451 | ⭕️  123126608 | 🕗 23.6136722565 |
| 🇳  452 | ⭕️  123945632 | 🕗 23.1733283997 |
| 🇳  453 | ⭕️  124768280 | 🕗 23.9569931030 |
| 🇳  454 | ⭕️  125594560 | 🕗 23.0045166016 |
| 🇳  455 | ⭕️  126424480 | 🕗 23.1410636902 |
| 🇳  456 | ⭕️  127258048 | 🕗 23.3821697235 |
| 🇳  457 | ⭕️  128095272 | 🕗 24.3528671265 |
| 🇳  458 | ⭕️  128936160 | 🕗 24.1306247711 |
| 🇳  459 | ⭕️  129780720 | 🕗 24.6480808258 |
| 🇳  460 | ⭕️  130628960 | 🕗 24.7265701294 |
| 🇳  461 | ⭕️  131480888 | 🕗 25.9914264679 |
| 🇳  462 | ⭕️  132336512 | 🕗 25.3464832306 |
| 🇳  463 | ⭕️  133195840 | 🕗 25.4487552643 |
| 🇳  464 | ⭕️  134058880 | 🕗 25.5008277893 |
| 🇳  465 | ⭕️  134925640 | 🕗 25.3527107239 |
| 🇳  466 | ⭕️  135796128 | 🕗 27.0097198486 |
| 🇳  467 | ⭕️  136670352 | 🕗 26.2712726593 |
| 🇳  468 | ⭕️  137548320 | 🕗 26.1255302429 |
| 🇳  469 | ⭕️  138430040 | 🕗 26.6336402893 |
| 🇳  470 | ⭕️  139315520 | 🕗 27.1713981628 |
| 🇳  471 | ⭕️  140204768 | 🕗 26.6834106445 |
| 🇳  472 | ⭕️  141097792 | 🕗 27.7472381592 |
| 🇳  473 | ⭕️  141994600 | 🕗 28.0326614380 |
| 🇳  474 | ⭕️  142895200 | 🕗 27.3202953339 |
| 🇳  475 | ⭕️  143799600 | 🕗 28.2141380310 |
| 🇳  476 | ⭕️  144707808 | 🕗 27.9727096558 |
| 🇳  477 | ⭕️  145619832 | 🕗 28.1392803192 |
| 🇳  478 | ⭕️  146535680 | 🕗 28.8946266174 |
| 🇳  479 | ⭕️  147455360 | 🕗 29.3382472992 |
| 🇳  480 | ⭕️  148378880 | 🕗 30.0514469147 |
| 🇳  481 | ⭕️  149306248 | 🕗 30.0045185089 |
| 🇳  482 | ⭕️  150237472 | 🕗 30.4353179932 |
| 🇳  483 | ⭕️  151172560 | 🕗 32.0125198364 |
| 🇳  484 | ⭕️  152111520 | 🕗 30.5776271820 |
| 🇳  485 | ⭕️  153054360 | 🕗 30.9182167053 |
| 🇳  486 | ⭕️  154001088 | 🕗 31.0784206390 |
| 🇳  487 | ⭕️  154951712 | 🕗 31.5720348358 |
| 🇳  488 | ⭕️  155906240 | 🕗 32.8261604309 |
| 🇳  489 | ⭕️  156864680 | 🕗 32.9490928650 |
| 🇳  490 | ⭕️  157827040 | 🕗 33.5027732849 |
| 🇳  491 | ⭕️  158793328 | 🕗 33.5297393799 |
| 🇳  492 | ⭕️  159763552 | 🕗 33.8598136902 |
| 🇳  493 | ⭕️  160737720 | 🕗 34.5865631104 |
| 🇳  494 | ⭕️  161715840 | 🕗 34.7499008179 |
| 🇳  495 | ⭕️  162697920 | 🕗 35.7603797913 |
| 🇳  496 | ⭕️  163683968 | 🕗 36.8854141235 |
| 🇳  497 | ⭕️  164673992 | 🕗 35.1132392883 |
| 🇳  498 | ⭕️  165668000 | 🕗 34.8395118713 |
| 🇳  499 | ⭕️  166666000 | 🕗 35.0684356689 |
| 🇳  500 | ⭕️  167668000 | 🕗 35.9405326843 |
| 🇳  600 | ⭕️  289441600 | 🕗 72.9078292847 |
| 🇳  700 | ⭕️  459295200 | 🕗 140.7563018799 |
| 🇳  800 | ⭕️  685228800 | 🕗 240.7478485107 |
| 🇳  900 | ⭕️  975242400 | 🕗 401.6239013672 |
| 🇳 1000 | ⭕️ 1337336000 | 🕗 587.2253417969 |
| 🇳 1100 | ⭕️ 1779509600 | 🕗 877.6129760742 |
| 🇳 1200 | ⭕️ 2309763200 | 🕗 1241.1076660156 |
| 🇳 1300 | ⭕️ 2936096800 | 🕗 1725.5015869141 |
| 🇳 1400 | ⭕️ 3666510400 | 🕗 2321.0217285156 |
| 🇳 1500 | ⭕️ 4509004000 | 🕗 3046.4963378906 |
| 🇳 1600 | ⭕️ 5471577600 | 🕗 3627.0874023438 |
| 🇳 1700 | ⭕️ 6562231200 | 🕗 4490.7988281250 |
| 🇳 1800 | ⭕️ 7788964800 | 🕗 5199.8408203125 |
| 🇳 1850 | ⭕️ 8455861600 | 🕗 5905.9962752852 |
| 🇳 1900 | ⭕️ 9159778400 | 🕗 6484.3529332790 |













```

ESTIMATED SIZE OF SOLUTION

| ORDER (Billions) | SIZE (GB) |
|:-----------------|:----------|
| 1                | 5.88      |
| 1.5              | 8.82      |
| 2                | 11.76     |
| 2.5              | 14.70     |
| 3                | 17.64     |
| 3.5              | 20.58     |
| 4                | 23.53     |
| 4.5              | 26.47     |
| 5                | 29.41     |
| 5.5              | 32.35     |
| 6                | 35.29     |
| 6.5              | 38.23     |
| 7                | 41.18     |
| 7.5              | 44.12     |
| 8                | 47.06     |
| 8.5              | 50.00     |
| 9                | 52.94     |
| 9.5              | 55.88     |
| 10               | 58.82     |
## Licensing:

This package is licensed under the MIT license.
 
Thanks for making it this far!
