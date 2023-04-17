## Table of Contents

- [Motivation](#section-a)
- [Digital Discocubes](#section-b)
- [Command Line Usage](#section-c)
- [Plotting the Solution](#section-d)
- [Dependencies](#section-e)
- [Running Times up to 9 billion](#section-f)
- [Licensing](#section-g)
---
<br>

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

<a name="section-a"></a>

### Links:
![NP-Completeness explained](https://youtu.be/ctwX--JEzSA)

![Discocubes](imgs/dcviews.png?raw=true "Discocubes")
*Discocubes 8 - 1760*

![Hexprism Honeycomb Diamond](imgs/hexhoneydiamond.png?raw=true "Hexprism Honeycomb Diamond")
*Hexprism Honeycomb Diamond*

## digital discocubes
As each solution is as unique as a fingerprint, or a diamond it allows one to have their own digital version of a discocube, which is also an instruction for building your own.

![Discocube 3640 view](imgs/icy_cube.png?raw=true "icy cube") 
![Discocube 3640 view](imgs/icy_cube3.png?raw=true "confetti cube")
*Discocubes as glb, using different mirrored texture yields personalized results and unique reflections meaning each discocube has its own reflection/shadow fingerprint! With millions of combinations available (glass texture/image/color, mirror texture/image/color, edge texture/image/color), the possibilities are endless!*

The always turning hamiltonian cycle digital discocubes are not produced by the algorithm in this repository, but by another polynomial-time algorithm.

![Solution to a 79040 node graph](imgs/solution.png)
*Detail for a Hamiltonian cycle for a graph with 79,040 nodes.*
<a name="section-b"></a>

## Command line usage
To use the package via the command line, navigate to the root directory of the project in your terminal and run the following command:
```
cargo run --release [Graph start instance] [Graph end instance] [steps] [repeats]
```
```
cargo run --release 1 100 1 100
```
build > run > make > solve > certify > for each graph starting from 32 to 1.373 million vertices in steps of 1 and run each order 100x.

## Plotting the solution
The solution can be plotted using pandas, numpy and plotly. I've put together an easy to use python module: https://github.com/discocube/plot_solution to plot and very the solution visually instead of only programmatically.

![Very first discocube in Berghain](imgs/ako.png)
*Me and Discocube in Berghain*
<a name="section-c"></a>

## Dependencies

This repository uses the following crates (ordered by most used) for increasing the speed of the algorithm for which it is grateful:

<em>For iterator traits, ndarrays, matrix operations on ndarrays, and parallelizing sequential computations:</em>
- [itertools](https://docs.rs/itertools/latest/itertools/): Extra iterator adaptors, functions and macros.
- [rayon](https://docs.rs/rayon/latest/rayon/): An data-parallelism library to parallelize sequential computations whilst guarateeing data-race freedom. 
- [ndarray](https://docs.rs/ndarray/latest/ndarray/): The ndarray crate provides an n-dimensional container for general elements and for numerics.

<em>For serializing and writing to</em> `.csv` <em>file:</em>
- [csv](https://docs.rs/csv/latest/csv/): The csv crate provides a fast and flexible CSV reader and writer, with support for Serde.
- [serde](https://docs.rs/serde/latest/serde/): Serde is a framework for serializing and deserializing Rust data structures efficiently and generically.

<em>For timestamping:</em>
- [chrono](https://docs.rs/chrono/latest/chrono/): It aims to be a feature-complete superset of the time library.
<a name="section-d"></a>

## Running times
![Running times from 8 to over 8 billion vertices](imgs/8_to_8billion.png?raw=true "Runtimes 8 to over 8 billion")
<small>Running times from 8 to over 8 billion vertices</small>

#### Running times for graphs with 8 to over 9 billion vertices (solved in under 2 hours):
```
| 🇳    1 | ⭕️          8 | 🕗 SOLVE: 0.0000097090
| 🇳    2 | ⭕️         32 | 🕗 SOLVE: 0.0000107500
| 🇳    3 | ⭕️         80 | 🕗 SOLVE: 0.0000213750
| 🇳    4 | ⭕️        160 | 🕗 SOLVE: 0.0000417500
| 🇳    5 | ⭕️        280 | 🕗 SOLVE: 0.0000715000
| 🇳    6 | ⭕️        448 | 🕗 SOLVE: 0.0000695000
| 🇳    7 | ⭕️        672 | 🕗 SOLVE: 0.0000869170
| 🇳    8 | ⭕️        960 | 🕗 SOLVE: 0.0001065000
| 🇳    9 | ⭕️       1320 | 🕗 SOLVE: 0.0001346250
| 🇳   10 | ⭕️       1760 | 🕗 SOLVE: 0.0001462080
| 🇳   11 | ⭕️       2288 | 🕗 SOLVE: 0.0002011250
| 🇳   12 | ⭕️       2912 | 🕗 SOLVE: 0.0002020420
| 🇳   13 | ⭕️       3640 | 🕗 SOLVE: 0.0002368750
| 🇳   14 | ⭕️       4480 | 🕗 SOLVE: 0.0001658330
| 🇳   15 | ⭕️       5440 | 🕗 SOLVE: 0.0002297080
| 🇳   16 | ⭕️       6528 | 🕗 SOLVE: 0.0002664170
| 🇳   17 | ⭕️       7752 | 🕗 SOLVE: 0.0003197080
| 🇳   18 | ⭕️       9120 | 🕗 SOLVE: 0.0003539170
| 🇳   19 | ⭕️      10640 | 🕗 SOLVE: 0.0004737500
| 🇳   20 | ⭕️      12320 | 🕗 SOLVE: 0.0004786670
| 🇳   21 | ⭕️      14168 | 🕗 SOLVE: 0.0005426670
| 🇳   22 | ⭕️      16192 | 🕗 SOLVE: 0.0005195830
| 🇳   23 | ⭕️      18400 | 🕗 SOLVE: 0.0006308750
| 🇳   24 | ⭕️      20800 | 🕗 SOLVE: 0.0005582920
| 🇳   25 | ⭕️      23400 | 🕗 SOLVE: 0.0007257920
| 🇳   26 | ⭕️      26208 | 🕗 SOLVE: 0.0007504160
| 🇳   27 | ⭕️      29232 | 🕗 SOLVE: 0.0008821250
| 🇳   28 | ⭕️      32480 | 🕗 SOLVE: 0.0008918750
| 🇳   29 | ⭕️      35960 | 🕗 SOLVE: 0.0010221250
| 🇳   30 | ⭕️      39680 | 🕗 SOLVE: 0.0010365410
| 🇳   31 | ⭕️      43648 | 🕗 SOLVE: 0.0011945000
| 🇳   32 | ⭕️      47872 | 🕗 SOLVE: 0.0011780420
| 🇳   33 | ⭕️      52360 | 🕗 SOLVE: 0.0012979590
| 🇳   34 | ⭕️      57120 | 🕗 SOLVE: 0.0013822080
| 🇳   35 | ⭕️      62160 | 🕗 SOLVE: 0.0014730840
| 🇳   36 | ⭕️      67488 | 🕗 SOLVE: 0.0015290841
| 🇳   37 | ⭕️      73112 | 🕗 SOLVE: 0.0017460830
| 🇳   38 | ⭕️      79040 | 🕗 SOLVE: 0.0018225830
| 🇳   39 | ⭕️      85280 | 🕗 SOLVE: 0.0019881660
| 🇳   40 | ⭕️      91840 | 🕗 SOLVE: 0.0019867499
| 🇳   41 | ⭕️      98728 | 🕗 SOLVE: 0.0021503749
| 🇳   42 | ⭕️     105952 | 🕗 SOLVE: 0.0023369170
| 🇳   43 | ⭕️     113520 | 🕗 SOLVE: 0.0025609999
| 🇳   44 | ⭕️     121440 | 🕗 SOLVE: 0.0026205841
| 🇳   45 | ⭕️     129720 | 🕗 SOLVE: 0.0029434580
| 🇳   46 | ⭕️     138368 | 🕗 SOLVE: 0.0029239580
| 🇳   47 | ⭕️     147392 | 🕗 SOLVE: 0.0031527090
| 🇳   48 | ⭕️     156800 | 🕗 SOLVE: 0.0032564159
| 🇳   49 | ⭕️     166600 | 🕗 SOLVE: 0.0035632080
| 🇳   50 | ⭕️     176800 | 🕗 SOLVE: 0.0037121661
| 🇳   51 | ⭕️     187408 | 🕗 SOLVE: 0.0039611249
| 🇳   52 | ⭕️     198432 | 🕗 SOLVE: 0.0041897921
| 🇳   53 | ⭕️     209880 | 🕗 SOLVE: 0.0044020829
| 🇳   54 | ⭕️     221760 | 🕗 SOLVE: 0.0046391250
| 🇳   55 | ⭕️     234080 | 🕗 SOLVE: 0.0049297502
| 🇳   56 | ⭕️     246848 | 🕗 SOLVE: 0.0051606670
| 🇳   57 | ⭕️     260072 | 🕗 SOLVE: 0.0054855002
| 🇳   58 | ⭕️     273760 | 🕗 SOLVE: 0.0057385000
| 🇳   59 | ⭕️     287920 | 🕗 SOLVE: 0.0060789580
| 🇳   60 | ⭕️     302560 | 🕗 SOLVE: 0.0063904999
| 🇳   61 | ⭕️     317688 | 🕗 SOLVE: 0.0067481659
| 🇳   62 | ⭕️     333312 | 🕗 SOLVE: 0.0069979592
| 🇳   63 | ⭕️     349440 | 🕗 SOLVE: 0.0073338752
| 🇳   64 | ⭕️     366080 | 🕗 SOLVE: 0.0077237082
| 🇳   65 | ⭕️     383240 | 🕗 SOLVE: 0.0082284156
| 🇳   66 | ⭕️     400928 | 🕗 SOLVE: 0.0085117500
| 🇳   67 | ⭕️     419152 | 🕗 SOLVE: 0.0089867497
| 🇳   68 | ⭕️     437920 | 🕗 SOLVE: 0.0093887504
| 🇳   69 | ⭕️     457240 | 🕗 SOLVE: 0.0099687502
| 🇳   70 | ⭕️     477120 | 🕗 SOLVE: 0.0103635835
| 🇳   71 | ⭕️     497568 | 🕗 SOLVE: 0.0108739575
| 🇳   72 | ⭕️     518592 | 🕗 SOLVE: 0.0113045424
| 🇳   73 | ⭕️     540200 | 🕗 SOLVE: 0.0118363751
| 🇳   74 | ⭕️     562400 | 🕗 SOLVE: 0.0122363344
| 🇳   75 | ⭕️     585200 | 🕗 SOLVE: 0.0129005844
| 🇳   76 | ⭕️     608608 | 🕗 SOLVE: 0.0137065826
| 🇳   77 | ⭕️     632632 | 🕗 SOLVE: 0.0141941672
| 🇳   78 | ⭕️     657280 | 🕗 SOLVE: 0.0148375416
| 🇳   79 | ⭕️     682560 | 🕗 SOLVE: 0.0156086674
| 🇳   80 | ⭕️     708480 | 🕗 SOLVE: 0.0162032507
| 🇳   81 | ⭕️     735048 | 🕗 SOLVE: 0.0171094593
| 🇳   82 | ⭕️     762272 | 🕗 SOLVE: 0.0175603312
| 🇳   83 | ⭕️     790160 | 🕗 SOLVE: 0.0183112919
| 🇳   84 | ⭕️     818720 | 🕗 SOLVE: 0.0187803768
| 🇳   85 | ⭕️     847960 | 🕗 SOLVE: 0.0198742915
| 🇳   86 | ⭕️     877888 | 🕗 SOLVE: 0.0207644999
| 🇳   87 | ⭕️     908512 | 🕗 SOLVE: 0.0215912925
| 🇳   88 | ⭕️     939840 | 🕗 SOLVE: 0.0224560834
| 🇳   89 | ⭕️     971880 | 🕗 SOLVE: 0.0234422926
| 🇳   90 | ⭕️    1004640 | 🕗 SOLVE: 0.0244386680
| 🇳   91 | ⭕️    1038128 | 🕗 SOLVE: 0.0255077500
| 🇳   92 | ⭕️    1072352 | 🕗 SOLVE: 0.0262854602
| 🇳   93 | ⭕️    1107320 | 🕗 SOLVE: 0.0271954164
| 🇳   94 | ⭕️    1143040 | 🕗 SOLVE: 0.0281380415
| 🇳   95 | ⭕️    1179520 | 🕗 SOLVE: 0.0292072073
| 🇳   96 | ⭕️    1216768 | 🕗 SOLVE: 0.0306402072
| 🇳   97 | ⭕️    1254792 | 🕗 SOLVE: 0.0317438766
| 🇳   98 | ⭕️    1293600 | 🕗 SOLVE: 0.0324681662
| 🇳   99 | ⭕️    1333200 | 🕗 SOLVE: 0.0340742096
| 🇳  100 | ⭕️    1373600 | 🕗 SOLVE: 0.0350561664
| 🇳  101 | ⭕️    1414808 | 🕗 SOLVE: 0.0368502922
| 🇳  102 | ⭕️    1456832 | 🕗 SOLVE: 0.0381248333
| 🇳  103 | ⭕️    1499680 | 🕗 SOLVE: 0.0394304171
| 🇳  104 | ⭕️    1543360 | 🕗 SOLVE: 0.0410415009
| 🇳  105 | ⭕️    1587880 | 🕗 SOLVE: 0.0424494557
| 🇳  106 | ⭕️    1633248 | 🕗 SOLVE: 0.0435284562
| 🇳  107 | ⭕️    1679472 | 🕗 SOLVE: 0.0457262471
| 🇳  108 | ⭕️    1726560 | 🕗 SOLVE: 0.0473354161
| 🇳  109 | ⭕️    1774520 | 🕗 SOLVE: 0.0491022095
| 🇳  110 | ⭕️    1823360 | 🕗 SOLVE: 0.0503717065
| 🇳  111 | ⭕️    1873088 | 🕗 SOLVE: 0.0522921681
| 🇳  112 | ⭕️    1923712 | 🕗 SOLVE: 0.0539964996
| 🇳  113 | ⭕️    1975240 | 🕗 SOLVE: 0.0559857935
| 🇳  114 | ⭕️    2027680 | 🕗 SOLVE: 0.0575002469
| 🇳  115 | ⭕️    2081040 | 🕗 SOLVE: 0.0596068762
| 🇳  116 | ⭕️    2135328 | 🕗 SOLVE: 0.0615541674
| 🇳  117 | ⭕️    2190552 | 🕗 SOLVE: 0.0639717057
| 🇳  118 | ⭕️    2246720 | 🕗 SOLVE: 0.0659482107
| 🇳  119 | ⭕️    2303840 | 🕗 SOLVE: 0.0679325461
| 🇳  120 | ⭕️    2361920 | 🕗 SOLVE: 0.0702144951
| 🇳  121 | ⭕️    2420968 | 🕗 SOLVE: 0.0717284977
| 🇳  122 | ⭕️    2480992 | 🕗 SOLVE: 0.0745417550
| 🇳  123 | ⭕️    2542000 | 🕗 SOLVE: 0.0767467916
| 🇳  124 | ⭕️    2604000 | 🕗 SOLVE: 0.0792220384
| 🇳  125 | ⭕️    2667000 | 🕗 SOLVE: 0.0817649588
| 🇳  126 | ⭕️    2731008 | 🕗 SOLVE: 0.0842245817
| 🇳  127 | ⭕️    2796032 | 🕗 SOLVE: 0.0868031681
| 🇳  128 | ⭕️    2862080 | 🕗 SOLVE: 0.0892409608
| 🇳  129 | ⭕️    2929160 | 🕗 SOLVE: 0.0920271650
| 🇳  130 | ⭕️    2997280 | 🕗 SOLVE: 0.0947527513
| 🇳  131 | ⭕️    3066448 | 🕗 SOLVE: 0.0977729186
| 🇳  132 | ⭕️    3136672 | 🕗 SOLVE: 0.1008302048
| 🇳  133 | ⭕️    3207960 | 🕗 SOLVE: 0.1031779572
| 🇳  134 | ⭕️    3280320 | 🕗 SOLVE: 0.1059563756
| 🇳  135 | ⭕️    3353760 | 🕗 SOLVE: 0.1093240380
| 🇳  136 | ⭕️    3428288 | 🕗 SOLVE: 0.1125425845
| 🇳  137 | ⭕️    3503912 | 🕗 SOLVE: 0.1158260778
| 🇳  138 | ⭕️    3580640 | 🕗 SOLVE: 0.1189340428
| 🇳  139 | ⭕️    3658480 | 🕗 SOLVE: 0.1223339587
| 🇳  140 | ⭕️    3737440 | 🕗 SOLVE: 0.1251037866
| 🇳  141 | ⭕️    3817528 | 🕗 SOLVE: 0.1289896667
| 🇳  142 | ⭕️    3898752 | 🕗 SOLVE: 0.1321854591
| 🇳  143 | ⭕️    3981120 | 🕗 SOLVE: 0.1358027905
| 🇳  144 | ⭕️    4064640 | 🕗 SOLVE: 0.1388999522
| 🇳  145 | ⭕️    4149320 | 🕗 SOLVE: 0.1436777413
| 🇳  146 | ⭕️    4235168 | 🕗 SOLVE: 0.1470130831
| 🇳  147 | ⭕️    4322192 | 🕗 SOLVE: 0.1511201560
| 🇳  148 | ⭕️    4410400 | 🕗 SOLVE: 0.1546522528
| 🇳  149 | ⭕️    4499800 | 🕗 SOLVE: 0.1593862921
| 🇳  150 | ⭕️    4590400 | 🕗 SOLVE: 0.1629686654
| 🇳  151 | ⭕️    4682208 | 🕗 SOLVE: 0.1674408019
| 🇳  152 | ⭕️    4775232 | 🕗 SOLVE: 0.1710964888
| 🇳  153 | ⭕️    4869480 | 🕗 SOLVE: 0.1752822846
| 🇳  154 | ⭕️    4964960 | 🕗 SOLVE: 0.1798237413
| 🇳  155 | ⭕️    5061680 | 🕗 SOLVE: 0.1846383363
| 🇳  156 | ⭕️    5159648 | 🕗 SOLVE: 0.1887653768
| 🇳  157 | ⭕️    5258872 | 🕗 SOLVE: 0.1937960535
| 🇳  158 | ⭕️    5359360 | 🕗 SOLVE: 0.1982746720
| 🇳  159 | ⭕️    5461120 | 🕗 SOLVE: 0.2037670016
| 🇳  160 | ⭕️    5564160 | 🕗 SOLVE: 0.2079945803
| 🇳  161 | ⭕️    5668488 | 🕗 SOLVE: 0.2133634984
| 🇳  162 | ⭕️    5774112 | 🕗 SOLVE: 0.2184348255
| 🇳  163 | ⭕️    5881040 | 🕗 SOLVE: 0.2234341353
| 🇳  164 | ⭕️    5989280 | 🕗 SOLVE: 0.2281720042
| 🇳  165 | ⭕️    6098840 | 🕗 SOLVE: 0.2344271243
| 🇳  166 | ⭕️    6209728 | 🕗 SOLVE: 0.2397274971
| 🇳  167 | ⭕️    6321952 | 🕗 SOLVE: 0.2456494272
| 🇳  168 | ⭕️    6435520 | 🕗 SOLVE: 0.2508569956
| 🇳  169 | ⭕️    6550440 | 🕗 SOLVE: 0.2570057809
| 🇳  170 | ⭕️    6666720 | 🕗 SOLVE: 0.2625517249
| 🇳  171 | ⭕️    6784368 | 🕗 SOLVE: 0.2684110105
| 🇳  172 | ⭕️    6903392 | 🕗 SOLVE: 0.2748391926
| 🇳  173 | ⭕️    7023800 | 🕗 SOLVE: 0.2808277905
| 🇳  174 | ⭕️    7145600 | 🕗 SOLVE: 0.2867560983
| 🇳  175 | ⭕️    7268800 | 🕗 SOLVE: 0.2931822538
| 🇳  176 | ⭕️    7393408 | 🕗 SOLVE: 0.2994064391
| 🇳  177 | ⭕️    7519432 | 🕗 SOLVE: 0.3066619337
| 🇳  178 | ⭕️    7646880 | 🕗 SOLVE: 0.3131026328
| 🇳  179 | ⭕️    7775760 | 🕗 SOLVE: 0.3201743662
| 🇳  180 | ⭕️    7906080 | 🕗 SOLVE: 0.3270924091
| 🇳  181 | ⭕️    8037848 | 🕗 SOLVE: 0.3341746330
| 🇳  182 | ⭕️    8171072 | 🕗 SOLVE: 0.3408803344
| 🇳  183 | ⭕️    8305760 | 🕗 SOLVE: 0.3488851488
| 🇳  184 | ⭕️    8441920 | 🕗 SOLVE: 0.3556617498
| 🇳  185 | ⭕️    8579560 | 🕗 SOLVE: 0.3633066416
| 🇳  186 | ⭕️    8718688 | 🕗 SOLVE: 0.3710851371
| 🇳  187 | ⭕️    8859312 | 🕗 SOLVE: 0.3787183166
| 🇳  188 | ⭕️    9001440 | 🕗 SOLVE: 0.3868706226
| 🇳  189 | ⭕️    9145080 | 🕗 SOLVE: 0.3945339620
| 🇳  190 | ⭕️    9290240 | 🕗 SOLVE: 0.4026339650
| 🇳  191 | ⭕️    9436928 | 🕗 SOLVE: 0.4110474288
| 🇳  192 | ⭕️    9585152 | 🕗 SOLVE: 0.4194793105
| 🇳  193 | ⭕️    9734920 | 🕗 SOLVE: 0.4284542203
| 🇳  194 | ⭕️    9886240 | 🕗 SOLVE: 0.4367388487
| 🇳  195 | ⭕️   10039120 | 🕗 SOLVE: 0.4452858865
| 🇳  196 | ⭕️   10193568 | 🕗 SOLVE: 0.4539607465
| 🇳  197 | ⭕️   10349592 | 🕗 SOLVE: 0.4631373584
| 🇳  198 | ⭕️   10507200 | 🕗 SOLVE: 0.4724363089
| 🇳  199 | ⭕️   10666400 | 🕗 SOLVE: 0.4817487895
| 🇳  200 | ⭕️   10827200 | 🕗 SOLVE: 0.4913620353
| 🇳  201 | ⭕️   10989608 | 🕗 SOLVE: 0.5013707876
| 🇳  202 | ⭕️   11153632 | 🕗 SOLVE: 0.5109505057
| 🇳  203 | ⭕️   11319280 | 🕗 SOLVE: 0.5206042528
| 🇳  204 | ⭕️   11486560 | 🕗 SOLVE: 0.5308341980
| 🇳  205 | ⭕️   11655480 | 🕗 SOLVE: 0.5409295559
| 🇳  206 | ⭕️   11826048 | 🕗 SOLVE: 0.5517295599
| 🇳  207 | ⭕️   11998272 | 🕗 SOLVE: 0.5626879334
| 🇳  208 | ⭕️   12172160 | 🕗 SOLVE: 0.5735034347
| 🇳  209 | ⭕️   12347720 | 🕗 SOLVE: 0.5844789743
| 🇳  210 | ⭕️   12524960 | 🕗 SOLVE: 0.5952639580
| 🇳  211 | ⭕️   12703888 | 🕗 SOLVE: 0.6072149873
| 🇳  212 | ⭕️   12884512 | 🕗 SOLVE: 0.6184394360
| 🇳  213 | ⭕️   13066840 | 🕗 SOLVE: 0.6297327280
| 🇳  214 | ⭕️   13250880 | 🕗 SOLVE: 0.6412607431
| 🇳  215 | ⭕️   13436640 | 🕗 SOLVE: 0.6535311341
| 🇳  216 | ⭕️   13624128 | 🕗 SOLVE: 0.6657137275
| 🇳  217 | ⭕️   13813352 | 🕗 SOLVE: 0.6777288914
| 🇳  218 | ⭕️   14004320 | 🕗 SOLVE: 0.6908183694
| 🇳  219 | ⭕️   14197040 | 🕗 SOLVE: 0.7023741603
| 🇳  220 | ⭕️   14391520 | 🕗 SOLVE: 0.7142126560
| 🇳  221 | ⭕️   14587768 | 🕗 SOLVE: 0.7279970646
| 🇳  222 | ⭕️   14785792 | 🕗 SOLVE: 0.7411111593
| 🇳  223 | ⭕️   14985600 | 🕗 SOLVE: 0.7544422746
| 🇳  224 | ⭕️   15187200 | 🕗 SOLVE: 0.7683255672
| 🇳  225 | ⭕️   15390600 | 🕗 SOLVE: 0.7816805840
| 🇳  226 | ⭕️   15595808 | 🕗 SOLVE: 0.7951261401
| 🇳  227 | ⭕️   15802832 | 🕗 SOLVE: 0.8092628717
| 🇳  228 | ⭕️   16011680 | 🕗 SOLVE: 0.8227874637
| 🇳  229 | ⭕️   16222360 | 🕗 SOLVE: 0.8370870948
| 🇳  230 | ⭕️   16434880 | 🕗 SOLVE: 0.8519513607
| 🇳  231 | ⭕️   16649248 | 🕗 SOLVE: 0.8665815592
| 🇳  232 | ⭕️   16865472 | 🕗 SOLVE: 0.8816897273
| 🇳  233 | ⭕️   17083560 | 🕗 SOLVE: 0.8971861601
| 🇳  234 | ⭕️   17303520 | 🕗 SOLVE: 0.9118126035
| 🇳  235 | ⭕️   17525360 | 🕗 SOLVE: 0.9273353815
| 🇳  236 | ⭕️   17749088 | 🕗 SOLVE: 0.9427413940
| 🇳  237 | ⭕️   17974712 | 🕗 SOLVE: 0.9595806599
| 🇳  238 | ⭕️   18202240 | 🕗 SOLVE: 0.9752129316
| 🇳  239 | ⭕️   18431680 | 🕗 SOLVE: 0.9915909767
| 🇳  240 | ⭕️   18663040 | 🕗 SOLVE: 1.0082355738
| 🇳  241 | ⭕️   18896328 | 🕗 SOLVE: 1.0252707005
| 🇳  242 | ⭕️   19131552 | 🕗 SOLVE: 1.0422573090
| 🇳  243 | ⭕️   19368720 | 🕗 SOLVE: 1.0594242811
| 🇳  244 | ⭕️   19607840 | 🕗 SOLVE: 1.0765631199
| 🇳  245 | ⭕️   19848920 | 🕗 SOLVE: 1.0940006971
| 🇳  246 | ⭕️   20091968 | 🕗 SOLVE: 1.1113030910
| 🇳  247 | ⭕️   20336992 | 🕗 SOLVE: 1.1294479370
| 🇳  248 | ⭕️   20584000 | 🕗 SOLVE: 1.1474530697
| 🇳  249 | ⭕️   20833000 | 🕗 SOLVE: 1.1652510166
| 🇳  250 | ⭕️   21084000 | 🕗 SOLVE: 1.1844950914
| 🇳  251 | ⭕️   21337008 | 🕗 SOLVE: 1.2034733295
| 🇳  252 | ⭕️   21592032 | 🕗 SOLVE: 1.2223532200
| 🇳  253 | ⭕️   21849080 | 🕗 SOLVE: 1.2411861420
| 🇳  254 | ⭕️   22108160 | 🕗 SOLVE: 1.2601187229
| 🇳  255 | ⭕️   22369280 | 🕗 SOLVE: 1.2808616161
| 🇳  256 | ⭕️   22632448 | 🕗 SOLVE: 1.3115926981
| 🇳  257 | ⭕️   22897672 | 🕗 SOLVE: 1.3322868347
| 🇳  258 | ⭕️   23164960 | 🕗 SOLVE: 1.3524920940
| 🇳  259 | ⭕️   23434320 | 🕗 SOLVE: 1.3738888502
| 🇳  260 | ⭕️   23705760 | 🕗 SOLVE: 1.3948390484
| 🇳  261 | ⭕️   23979288 | 🕗 SOLVE: 1.4165151119
| 🇳  262 | ⭕️   24254912 | 🕗 SOLVE: 1.4381496906
| 🇳  263 | ⭕️   24532640 | 🕗 SOLVE: 1.4598348141
| 🇳  264 | ⭕️   24812480 | 🕗 SOLVE: 1.4822442532
| 🇳  265 | ⭕️   25094440 | 🕗 SOLVE: 1.5055291653
| 🇳  266 | ⭕️   25378528 | 🕗 SOLVE: 1.5293437243
| 🇳  267 | ⭕️   25664752 | 🕗 SOLVE: 1.5519559383
| 🇳  268 | ⭕️   25953120 | 🕗 SOLVE: 1.5750749111
| 🇳  269 | ⭕️   26243640 | 🕗 SOLVE: 1.5995900631
| 🇳  270 | ⭕️   26536320 | 🕗 SOLVE: 1.6239156723
| 🇳  271 | ⭕️   26831168 | 🕗 SOLVE: 1.6461290121
| 🇳  272 | ⭕️   27128192 | 🕗 SOLVE: 1.6721776724
| 🇳  273 | ⭕️   27427400 | 🕗 SOLVE: 1.6959779263
| 🇳  274 | ⭕️   27728800 | 🕗 SOLVE: 1.7217242718
| 🇳  275 | ⭕️   28032400 | 🕗 SOLVE: 1.7458661795
| 🇳  276 | ⭕️   28338208 | 🕗 SOLVE: 1.7727648020
| 🇳  277 | ⭕️   28646232 | 🕗 SOLVE: 1.7985424995
| 🇳  278 | ⭕️   28956480 | 🕗 SOLVE: 1.8250669241
| 🇳  279 | ⭕️   29268960 | 🕗 SOLVE: 1.8509416580
| 🇳  280 | ⭕️   29583680 | 🕗 SOLVE: 1.8764822483
| 🇳  281 | ⭕️   29900648 | 🕗 SOLVE: 1.9029159546
| 🇳  282 | ⭕️   30219872 | 🕗 SOLVE: 1.9283620119
| 🇳  283 | ⭕️   30541360 | 🕗 SOLVE: 1.9569215775
| 🇳  284 | ⭕️   30865120 | 🕗 SOLVE: 1.9848431349
| 🇳  285 | ⭕️   31191160 | 🕗 SOLVE: 2.0117216110
| 🇳  286 | ⭕️   31519488 | 🕗 SOLVE: 2.0411350727
| 🇳  287 | ⭕️   31850112 | 🕗 SOLVE: 2.0681724548
| 🇳  288 | ⭕️   32183040 | 🕗 SOLVE: 2.0961732864
| 🇳  289 | ⭕️   32518280 | 🕗 SOLVE: 2.1271593571
| 🇳  290 | ⭕️   32855840 | 🕗 SOLVE: 2.1546497345
| 🇳  291 | ⭕️   33195728 | 🕗 SOLVE: 2.1842732430
| 🇳  292 | ⭕️   33537952 | 🕗 SOLVE: 2.2156043053
| 🇳  293 | ⭕️   33882520 | 🕗 SOLVE: 2.2446796894
| 🇳  294 | ⭕️   34229440 | 🕗 SOLVE: 2.2725489140
| 🇳  295 | ⭕️   34578720 | 🕗 SOLVE: 2.3059878349
| 🇳  296 | ⭕️   34930368 | 🕗 SOLVE: 2.3383712769
| 🇳  297 | ⭕️   35284392 | 🕗 SOLVE: 2.3659286499
| 🇳  298 | ⭕️   35640800 | 🕗 SOLVE: 2.3995804787
| 🇳  299 | ⭕️   35999600 | 🕗 SOLVE: 2.4293415546
| 🇳  300 | ⭕️   36360800 | 🕗 SOLVE: 2.4614112377
| 🇳  301 | ⭕️   36724408 | 🕗 SOLVE: 2.4947695732
| 🇳  302 | ⭕️   37090432 | 🕗 SOLVE: 2.5255017281
| 🇳  303 | ⭕️   37458880 | 🕗 SOLVE: 2.5646390915
| 🇳  304 | ⭕️   37829760 | 🕗 SOLVE: 2.5915014744
| 🇳  305 | ⭕️   38203080 | 🕗 SOLVE: 2.6287050247
| 🇳  306 | ⭕️   38578848 | 🕗 SOLVE: 2.6606783867
| 🇳  307 | ⭕️   38957072 | 🕗 SOLVE: 2.6951565742
| 🇳  308 | ⭕️   39337760 | 🕗 SOLVE: 2.7306163311
| 🇳  309 | ⭕️   39720920 | 🕗 SOLVE: 2.7673854828
| 🇳  310 | ⭕️   40106560 | 🕗 SOLVE: 2.8000831604
| 🇳  311 | ⭕️   40494688 | 🕗 SOLVE: 2.8358983994
| 🇳  312 | ⭕️   40885312 | 🕗 SOLVE: 2.8742041588
| 🇳  313 | ⭕️   41278440 | 🕗 SOLVE: 2.9088292122
| 🇳  314 | ⭕️   41674080 | 🕗 SOLVE: 2.9427518845
| 🇳  315 | ⭕️   42072240 | 🕗 SOLVE: 2.9799244404
| 🇳  316 | ⭕️   42472928 | 🕗 SOLVE: 3.0185284615
| 🇳  317 | ⭕️   42876152 | 🕗 SOLVE: 3.0520420074
| 🇳  318 | ⭕️   43281920 | 🕗 SOLVE: 3.0918805599
| 🇳  319 | ⭕️   43690240 | 🕗 SOLVE: 3.1293289661
| 🇳  320 | ⭕️   44101120 | 🕗 SOLVE: 3.1736469269
| 🇳  321 | ⭕️   44514568 | 🕗 SOLVE: 3.2103056908
| 🇳  322 | ⭕️   44930592 | 🕗 SOLVE: 3.2500233650
| 🇳  323 | ⭕️   45349200 | 🕗 SOLVE: 3.2854855061
| 🇳  324 | ⭕️   45770400 | 🕗 SOLVE: 3.3255665302
| 🇳  325 | ⭕️   46194200 | 🕗 SOLVE: 3.3673853874
| 🇳  326 | ⭕️   46620608 | 🕗 SOLVE: 3.4071111679
| 🇳  327 | ⭕️   47049632 | 🕗 SOLVE: 3.4492669106
| 🇳  328 | ⭕️   47481280 | 🕗 SOLVE: 3.4897911549
| 🇳  329 | ⭕️   47915560 | 🕗 SOLVE: 3.5297830105
| 🇳  330 | ⭕️   48352480 | 🕗 SOLVE: 3.5713343620
| 🇳  331 | ⭕️   48792048 | 🕗 SOLVE: 3.6174914837
| 🇳  332 | ⭕️   49234272 | 🕗 SOLVE: 3.6580235958
| 🇳  333 | ⭕️   49679160 | 🕗 SOLVE: 3.6974585056
| 🇳  334 | ⭕️   50126720 | 🕗 SOLVE: 3.7451782227
| 🇳  335 | ⭕️   50576960 | 🕗 SOLVE: 3.7907252312
| 🇳  336 | ⭕️   51029888 | 🕗 SOLVE: 3.8349757195
| 🇳  337 | ⭕️   51485512 | 🕗 SOLVE: 3.8807768822

NEW TIMES ABOVE UPDATE WHEN FINISHED.

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
<a name="section-e"></a>

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
