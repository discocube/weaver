<a name="toc"></a>

![Solution to a 79040 node graph](imgs/solution.png)
<sup><sub><em>Detail for a Hamiltonian cycle for a graph with 79,040 nodes.</em></sup></sub>


## Table of Contents

- [What happens if a graph object becomes an artist's muse?](#section-a)
- [Digital Discocubes](#section-b)
- [Command Line Usage](#section-c)
- [Plotting the Solution](#section-d)
- [Dependencies](#section-e)
- [Running Times up to 8 billion](#section-f)
- [Licensing](#section-g)
- [⇓](#section-a)
---
![Very first discocube in Berghain](imgs/ako.png)
<sup><sub><em>Me and Discocube in Berghain</em></sup></sub>
<br>
<br>
<br>
<a name="section-a"></a>
## [⇪](#toc) What happens if a graph object becomes an artist's muse?
<small><em>“A great discovery solves a great problem, but there is a grain of discovery in the solution of any problem. Your problem may be modest, but if it challenges your curiosity and brings into play your inventive faculties, and if you solve it by your own means, you may experience the tension and enjoy the triumph of discovery.”</em>

George Pólya: <em>How to Solve It: A New Aspect of Mathematical Method</em></small>

A deterministic and linear-time algorithm for constructing a Hamiltonian cycle on all instances of discocube graphs (tested for graphs with over 8 billion vertices, the world's population). Discocube graphs are
3-dimensional grid graphs derived from: a polycube of an octahedron | a Hauy construction of an octahedron with cubes as identical building blocks | the accretion of cubes around a central cube forming an octahedron at the limit |
the subgraph of the infinite 3-dimensional square grid graph consisting only of points contained within an octahedron | a 3d L1-norm unit ball. 

![Planar embedding of Cube and Discocubes](imgs/planar_emb.png?raw=true "Planar embedding of Cube and Discocubes")
<sup><sub><em>Planar embedding of a cube and a discocube. From the set of all graphs G, where the order of G is of the ***Uncentered octahedral numbers*** [A130809](https://oeis.org/A130809), only the first two instances shown above; n[0] and n[1] are planarly embeddable i.e., it can be represented on a two-dimensional surface without any of its edges crossing.</em></em></sup>

This algorithm is an artist's rendering of his muse, a graph object, using programming as a language (instead of painting flowers and apples or singing hymns about angels) and a means by which to describe his muse's body as an endless contour drawing or,  in graph theory terms, a Hamiltonian cycle.

![Alt text](imgs/elumina-inspiration-one-line-picasso-drawings-the-three-dancers-thumbnail.jpg?version%3D1680306692540)
<small>Pablo Picasso:  <em>Trois Danseuses (The Three Dancers)</em></small>

![Alt text](imgs/hambw.png)
<small><em>The graphs from platonic solids are all Hamiltonian.</em></small>

What links an endless contour line drawing and a Hamiltonian cycle? Both entail tracing a path without interruption, using a continuous line to describe the subject/object to be depicted. An endless contour line drawing is one in which the artist uses a single, uninterrupted line to describe a subject's form and shape.
Similarly, a Hamiltonian cycle describes a graph by tracing a path along edges of the graph, visiting vertex precisely once before returning to the beginning vertex. The path connects the vertices in a continuous line, describing the graph object.

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

https://user-images.githubusercontent.com/93198518/232765984-5b94c0e0-d788-4d1f-a1d3-457600b73daf.mp4

An artist manipulates their medium to create forms, using brush strokes to describe how the curve of the neck disappears behind the back, or playing with colors and contrasts to subtly bring the skin of a subject living 500 years ago back to life. For many, the medium is the end to itself, like painting a painting or photographing a photo, rather than the medium being wholly dependent on the concept or idea to be executed. In my artistic practice the medium used for any particular project is dependent on the first finding the right language and then developing it into a visual language.

In this project, I studied the discocube visually as a body, imagining each turn not as a discrete mathematical object but as a series of possible movements, as an endlessly iterated dance captured in a long camera exposure, resulting in less equating and more doodling, and wishing I knew more math. The result is a family of algorithms for finding Hamiltonian cycles with varying degrees of refinement (edge distribution), with the weave algorithm producing the least refinement. The other algorithms are concerned with finding an initial Hamiltonian cycle with a higher mutation rate and whose edges are more uniformly spread across the three axes x, y, and z, and through subsequent processes, the solution is polished like a diamond, i.e., the initial tour improved upon until the edges are evenly distributed across all three axes, culminating in an always-turning Hamiltonian cycle. Owing to the regularity and consistency of the solution produced by this determined and predictive algorithm, using weave() to obtain a diamond-grade discocube would take an inconceivable period of time, necessitating the development of other algorithms capable of accomplishing this reasonably.


https://user-images.githubusercontent.com/93198518/232767029-a3cc643b-c695-4def-8248-50d0e24ac708.mp4


Why weave? Finding the solution to the problem reminded me of macramé, of tying knots, weaving, and spinning yarn. I thought of how patterns in handwoven fabric are actually unwitting recordings of a knitter's hand movements, like how a piano roll is a recording of the pianist's finger hitting ebony, or how a seismograph records the motion of the earth, or how our skin is a type of recording of our life... I followed the thought further and asked myself: was there a pattern to expose and use to construct the discocube, level by level, similar to how one would knit a scarf, row by row, until the desired result is reached? To illustrate the intention of the code succinctly, I've structured the code to mimic the process of weaving a piece of tapestry, from spinning the yarn to incorporating the weft into the warps.

![First 11 discocubes and their order (number of nodes)](imgs/rect5857.png?raw=true "Discocubes orders")
*The first eleven discocubes and their respective orders (number of nodes)*

To paraphrase Hauy: 

*When solving problems that involve analyzing how nature progresses, we are led by very rapid methods to results that are not immediately obvious. These results may appear paradoxical and surprising. However, if we take the time to carefully examine the steps we took to reach these results, we will begin to understand the underlying principles that led to these outcomes. By going back over the process step by step, we can better understand the logic behind the final results.*

What started as a hack-your-own version of a depth-first search with shortcuts for the discocube graph (solving up to 960 vertices) metastasized into pages of overgrown mixin classes mysteriously coupled to each other like overgrown vines, pushing me deeper and deeper into the underbelly of its mutant tentacles. Although able to solve instances with over a million vertices, the algorithm had the clarity of primordial soup. So, as a sadistic gardener, I painstakingly pruned my own pubicity (my unescapable web of thorny vines) into presentable tiny bonsai trees. So what is a bonsai if not a tree in intimate scope?

https://user-images.githubusercontent.com/93198518/232765555-511aaf82-6276-45b7-96c4-3a8865eebe0d.mp4

The result of this creative process is a family of algorithms developed specifically to solve various graph problems on dodecahedron graphs, 3D grid graphs, and hexprism honeycomb diamond graphs.
The algorithm presented in this repository is the least complex, making it the fastest. It does the job, solving the Hamiltonian cycle problem for over millions of vertices in seconds and graphing with over a billion vertices in less than an hour and a graph with over 8 billion vertices in less than 5 hours, while other algorithms in the family take longer but also have other objectives, like forming an always-turning cycle with even edge distribution across all axes. But that's beyond the scope of this repository.

This algorithm has no while loops and will terminate after a definitive set of steps. The strength of this algorithm is knowing exactly when, where, and what is to happen, thereby reducing the amount of calculations needed (which is surprising as the creative process in creating this was anything but deterministic). It is a construction algorithm, constructing the path layer by layer until loops are produced, which are then joined using cycle merging. Further optimizations of the algorithm have also discarded the memory-heavy adjacency list, choosing instead to perform individual calculations where needed. Making and solving a graph with over a billion vertices, where n = 1000, won't require a distributed graph engine on the cloud anymore, and it takes a little over ten minutes.
<br>
<br>
<br>
### [⇪](#toc) Links
![NP-Completeness explained](https://youtu.be/ctwX--JEzSA)

![Discocubes](imgs/dcviews.png?raw=true "Discocubes")
*Discocubes 8 - 1760*

![Hexprism Honeycomb Diamond](imgs/hexhoneydiamond.png?raw=true "Hexprism Honeycomb Diamond")
*Hexprism Honeycomb Diamond*
<br>
<br>
<br>
<a name="section-b"></a>
## [⇪](#toc) digital discocubes  
[Play around and inspect a digital discocube here](https://playcanvas.com/model-viewer?load=https://raw.githubusercontent.com/discocube/weaver/main/imgs/snow3641.glb)

As each solution is as unique as a fingerprint, or a diamond it allows one to have their own digital version of a discocube, which is also an instruction for building your own.

![Discocube 3640 view](imgs/icy_cube.png?raw=true "icy cube") 
![Discocube 3640 view](imgs/icy_cube3.png?raw=true "confetti cube")
*Discocubes as glb, using different mirrored texture yields personalized results and unique reflections meaning each discocube has its own reflection/shadow fingerprint! With millions of combinations available (glass texture/image/color, mirror texture/image/color, edge texture/image/color), the possibilities are endless!*

The always turning hamiltonian cycle digital discocubes are not produced by the algorithm in this repository, but by another polynomial-time algorithm.
<br>
<br>
<br>
<a name="section-c"></a>

## [⇪](#toc) Command line usage
To use the package via the command line, navigate to the root directory of the project in your terminal and run the following command:
```
cargo run --release [Graph start instance] [Graph end instance] [steps] [repeats]
```
```
cargo run --release 1 100 1 100
```
<em>For each graph starting from 32 to 1.373 million vertices solve each graph order in steps of one and running each 100x to get the best time.</em>
<br>
<br>
<br>
<a name="section-d"></a>
## [⇪](#toc) Plotting the solution
The solution can be plotted using [this](https://github.com/discocube/plot_solution) python module to visualize and check the solution.
<br>
<br>
<br>
<a name="section-e"></a>
## [⇪](#toc) Dependencies

This repository uses the following crates (ordered by most to least used) for which it is grateful.

<em>For iterator traits, ndarrays, matrix operations on ndarrays, and parallelizing sequential computations:</em>
- [`itertools`](https://docs.rs/itertools/latest/itertools/)    Extra iterator adaptors, functions and macros.
- [`rayon`](https://docs.rs/rayon/latest/rayon/)    Data-parallelism library for parallelizing sequential computations whilst guaranteeing data-race freedom. 
- [`ndarray`](https://docs.rs/ndarray/latest/ndarray/)    Provides an n-dimensional container for general elements and for numerics.

<em>For serializing and writing to</em> `.csv` <em>file:</em>
- [`csv`](https://docs.rs/csv/latest/csv/)    Fast and flexible CSV reader and writer, with support for Serde.
- [`serde`](https://docs.rs/serde/latest/serde/)    Framework for serializing and deserializing Rust data structures efficiently and generically.

<em>For timestamping:</em>
- [`chrono`](https://docs.rs/chrono/latest/chrono/)    A feature-complete superset of the time library.
<br>
<br>
<a name="section-f"></a>

## [⇪](#toc) Running times

![Running times from 8 to over 8 billion vertices](https://user-images.githubusercontent.com/93198518/232534045-b193abdf-c32f-469f-9992-6ef0cfab3cfa.png)
<small>Running times from 8 to over 8 billion vertices</small> 
<br>
<br>
#### [⇪](#toc) Running times for graphs with 8 to over 8 billion vertices (solved in under 2 hours)
```
| 🇳    1 | ⭕️          8 | 🕗 0.0000097090
| 🇳    2 | ⭕️         32 | 🕗 0.0000107500
| 🇳    3 | ⭕️         80 | 🕗 0.0000213750
| 🇳    4 | ⭕️        160 | 🕗 0.0000417500
| 🇳    5 | ⭕️        280 | 🕗 0.0000715000
| 🇳    6 | ⭕️        448 | 🕗 0.0000695000
| 🇳    7 | ⭕️        672 | 🕗 0.0000869170
| 🇳    8 | ⭕️        960 | 🕗 0.0001065000
| 🇳    9 | ⭕️       1320 | 🕗 0.0001346250
| 🇳   10 | ⭕️       1760 | 🕗 0.0001462080
| 🇳   11 | ⭕️       2288 | 🕗 0.0002011250
| 🇳   12 | ⭕️       2912 | 🕗 0.0002020420
| 🇳   13 | ⭕️       3640 | 🕗 0.0002368750
| 🇳   14 | ⭕️       4480 | 🕗 0.0001658330
| 🇳   15 | ⭕️       5440 | 🕗 0.0002297080
| 🇳   16 | ⭕️       6528 | 🕗 0.0002664170
| 🇳   17 | ⭕️       7752 | 🕗 0.0003197080
| 🇳   18 | ⭕️       9120 | 🕗 0.0003539170
| 🇳   19 | ⭕️      10640 | 🕗 0.0004737500
| 🇳   20 | ⭕️      12320 | 🕗 0.0004786670
| 🇳   21 | ⭕️      14168 | 🕗 0.0005426670
| 🇳   22 | ⭕️      16192 | 🕗 0.0005195830
| 🇳   23 | ⭕️      18400 | 🕗 0.0006308750
| 🇳   24 | ⭕️      20800 | 🕗 0.0005582920
| 🇳   25 | ⭕️      23400 | 🕗 0.0007257920
| 🇳   26 | ⭕️      26208 | 🕗 0.0007504160
| 🇳   27 | ⭕️      29232 | 🕗 0.0008821250
| 🇳   28 | ⭕️      32480 | 🕗 0.0008918750
| 🇳   29 | ⭕️      35960 | 🕗 0.0010221250
| 🇳   30 | ⭕️      39680 | 🕗 0.0010365410
| 🇳   31 | ⭕️      43648 | 🕗 0.0011945000
| 🇳   32 | ⭕️      47872 | 🕗 0.0011780420
| 🇳   33 | ⭕️      52360 | 🕗 0.0012979590
| 🇳   34 | ⭕️      57120 | 🕗 0.0013822080
| 🇳   35 | ⭕️      62160 | 🕗 0.0014730840
| 🇳   36 | ⭕️      67488 | 🕗 0.0015290841
| 🇳   37 | ⭕️      73112 | 🕗 0.0017460830
| 🇳   38 | ⭕️      79040 | 🕗 0.0018225830
| 🇳   39 | ⭕️      85280 | 🕗 0.0019881660
| 🇳   40 | ⭕️      91840 | 🕗 0.0019867499
| 🇳   41 | ⭕️      98728 | 🕗 0.0021503749
| 🇳   42 | ⭕️     105952 | 🕗 0.0023369170
| 🇳   43 | ⭕️     113520 | 🕗 0.0025609999
| 🇳   44 | ⭕️     121440 | 🕗 0.0026205841
| 🇳   45 | ⭕️     129720 | 🕗 0.0029434580
| 🇳   46 | ⭕️     138368 | 🕗 0.0029239580
| 🇳   47 | ⭕️     147392 | 🕗 0.0031527090
| 🇳   48 | ⭕️     156800 | 🕗 0.0032564159
| 🇳   49 | ⭕️     166600 | 🕗 0.0035632080
| 🇳   50 | ⭕️     176800 | 🕗 0.0037121661
| 🇳   51 | ⭕️     187408 | 🕗 0.0039611249
| 🇳   52 | ⭕️     198432 | 🕗 0.0041897921
| 🇳   53 | ⭕️     209880 | 🕗 0.0044020829
| 🇳   54 | ⭕️     221760 | 🕗 0.0046391250
| 🇳   55 | ⭕️     234080 | 🕗 0.0049297502
| 🇳   56 | ⭕️     246848 | 🕗 0.0051606670
| 🇳   57 | ⭕️     260072 | 🕗 0.0054855002
| 🇳   58 | ⭕️     273760 | 🕗 0.0057385000
| 🇳   59 | ⭕️     287920 | 🕗 0.0060789580
| 🇳   60 | ⭕️     302560 | 🕗 0.0063904999
| 🇳   61 | ⭕️     317688 | 🕗 0.0067481659
| 🇳   62 | ⭕️     333312 | 🕗 0.0069979592
| 🇳   63 | ⭕️     349440 | 🕗 0.0073338752
| 🇳   64 | ⭕️     366080 | 🕗 0.0077237082
| 🇳   65 | ⭕️     383240 | 🕗 0.0082284156
| 🇳   66 | ⭕️     400928 | 🕗 0.0085117500
| 🇳   67 | ⭕️     419152 | 🕗 0.0089867497
| 🇳   68 | ⭕️     437920 | 🕗 0.0093887504
| 🇳   69 | ⭕️     457240 | 🕗 0.0099687502
| 🇳   70 | ⭕️     477120 | 🕗 0.0103635835
| 🇳   71 | ⭕️     497568 | 🕗 0.0108739575
| 🇳   72 | ⭕️     518592 | 🕗 0.0113045424
| 🇳   73 | ⭕️     540200 | 🕗 0.0118363751
| 🇳   74 | ⭕️     562400 | 🕗 0.0122363344
| 🇳   75 | ⭕️     585200 | 🕗 0.0129005844
| 🇳   76 | ⭕️     608608 | 🕗 0.0137065826
| 🇳   77 | ⭕️     632632 | 🕗 0.0141941672
| 🇳   78 | ⭕️     657280 | 🕗 0.0148375416
| 🇳   79 | ⭕️     682560 | 🕗 0.0156086674
| 🇳   80 | ⭕️     708480 | 🕗 0.0162032507
| 🇳   81 | ⭕️     735048 | 🕗 0.0171094593
| 🇳   82 | ⭕️     762272 | 🕗 0.0175603312
| 🇳   83 | ⭕️     790160 | 🕗 0.0183112919
| 🇳   84 | ⭕️     818720 | 🕗 0.018780376
| 🇳   85 | ⭕️     847960 | 🕗 0.0198742915
| 🇳   86 | ⭕️     877888 | 🕗 0.0207644999
| 🇳   87 | ⭕️     908512 | 🕗 0.021591292
| 🇳   88 | ⭕️     939840 | 🕗 0.0224560834
| 🇳   89 | ⭕️     971880 | 🕗 0.0234422926
| 🇳   90 | ⭕️    1004640 | 🕗 0.0244386680
| 🇳   91 | ⭕️    1038128 | 🕗 0.0255077500
| 🇳   92 | ⭕️    1072352 | 🕗 0.0262854602
| 🇳   93 | ⭕️    1107320 | 🕗 0.0271954164
| 🇳   94 | ⭕️    1143040 | 🕗 0.0281380415
| 🇳   95 | ⭕️    1179520 | 🕗 0.0292072073
| 🇳   96 | ⭕️    1216768 | 🕗 0.0306402072
| 🇳   97 | ⭕️    1254792 | 🕗 0.0317438766
| 🇳   98 | ⭕️    1293600 | 🕗 0.0324681662
| 🇳   99 | ⭕️    1333200 | 🕗 0.0340742096
| 🇳  100 | ⭕️    1373600 | 🕗 0.0350561664
| 🇳  101 | ⭕️    1414808 | 🕗 0.0368502922
| 🇳  102 | ⭕️    1456832 | 🕗 0.0381248333
| 🇳  103 | ⭕️    1499680 | 🕗 0.0394304171
| 🇳  104 | ⭕️    1543360 | 🕗 0.0410415009
| 🇳  105 | ⭕️    1587880 | 🕗 0.0424494557
| 🇳  106 | ⭕️    1633248 | 🕗 0.0435284562
| 🇳  107 | ⭕️    1679472 | 🕗 0.0457262471
| 🇳  108 | ⭕️    1726560 | 🕗 0.0473354161
| 🇳  109 | ⭕️    1774520 | 🕗 0.0491022095
| 🇳  110 | ⭕️    1823360 | 🕗 0.0503717065
| 🇳  111 | ⭕️    1873088 | 🕗 0.0522921681
| 🇳  112 | ⭕️    1923712 | 🕗 0.0539964996
| 🇳  113 | ⭕️    1975240 | 🕗 0.0559857935
| 🇳  114 | ⭕️    2027680 | 🕗 0.0575002469
| 🇳  115 | ⭕️    2081040 | 🕗 0.0596068762
| 🇳  116 | ⭕️    2135328 | 🕗 0.0615541674
| 🇳  117 | ⭕️    2190552 | 🕗 0.0639717057
| 🇳  118 | ⭕️    2246720 | 🕗 0.0659482107
| 🇳  119 | ⭕️    2303840 | 🕗 0.0679325461
| 🇳  120 | ⭕️    2361920 | 🕗 0.0702144951
| 🇳  121 | ⭕️    2420968 | 🕗 0.0717284977
| 🇳  122 | ⭕️    2480992 | 🕗 0.0745417550
| 🇳  123 | ⭕️    2542000 | 🕗 0.0767467916
| 🇳  124 | ⭕️    2604000 | 🕗 0.0792220384
| 🇳  125 | ⭕️    2667000 | 🕗 0.0817649588
| 🇳  126 | ⭕️    2731008 | 🕗 0.0842245817
| 🇳  127 | ⭕️    2796032 | 🕗 0.0868031681
| 🇳  128 | ⭕️    2862080 | 🕗 0.0892409608
| 🇳  129 | ⭕️    2929160 | 🕗 0.0920271650
| 🇳  130 | ⭕️    2997280 | 🕗 0.0947527513
| 🇳  131 | ⭕️    3066448 | 🕗 0.0977729186
| 🇳  132 | ⭕️    3136672 | 🕗 0.1008302048
| 🇳  133 | ⭕️    3207960 | 🕗 0.1031779572
| 🇳  134 | ⭕️    3280320 | 🕗 0.1059563756
| 🇳  135 | ⭕️    3353760 | 🕗 0.1093240380
| 🇳  136 | ⭕️    3428288 | 🕗 0.1125425845
| 🇳  137 | ⭕️    3503912 | 🕗 0.1158260778
| 🇳  138 | ⭕️    3580640 | 🕗 0.1189340428
| 🇳  139 | ⭕️    3658480 | 🕗 0.1223339587
| 🇳  140 | ⭕️    3737440 | 🕗 0.1251037866
| 🇳  141 | ⭕️    3817528 | 🕗 0.1289896667
| 🇳  142 | ⭕️    3898752 | 🕗 0.1321854591
| 🇳  143 | ⭕️    3981120 | 🕗 0.1358027905
| 🇳  144 | ⭕️    4064640 | 🕗 0.1388999522
| 🇳  145 | ⭕️    4149320 | 🕗 0.1436777413
| 🇳  146 | ⭕️    4235168 | 🕗 0.1470130831
| 🇳  147 | ⭕️    4322192 | 🕗 0.1511201560
| 🇳  148 | ⭕️    4410400 | 🕗 0.1546522528
| 🇳  149 | ⭕️    4499800 | 🕗 0.1593862921
| 🇳  150 | ⭕️    4590400 | 🕗 0.1629686654
| 🇳  151 | ⭕️    4682208 | 🕗 0.1674408019
| 🇳  152 | ⭕️    4775232 | 🕗 0.1710964888
| 🇳  153 | ⭕️    4869480 | 🕗 0.1752822846
| 🇳  154 | ⭕️    4964960 | 🕗 0.1798237413
| 🇳  155 | ⭕️    5061680 | 🕗 0.1846383363
| 🇳  156 | ⭕️    5159648 | 🕗 0.1887653768
| 🇳  157 | ⭕️    5258872 | 🕗 0.1937960535
| 🇳  158 | ⭕️    5359360 | 🕗 0.1982746720
| 🇳  159 | ⭕️    5461120 | 🕗 0.2037670016
| 🇳  160 | ⭕️    5564160 | 🕗 0.2079945803
| 🇳  161 | ⭕️    5668488 | 🕗 0.2133634984
| 🇳  162 | ⭕️    5774112 | 🕗 0.2184348255
| 🇳  163 | ⭕️    5881040 | 🕗 0.2234341353
| 🇳  164 | ⭕️    5989280 | 🕗 0.2281720042
| 🇳  165 | ⭕️    6098840 | 🕗 0.2344271243
| 🇳  166 | ⭕️    6209728 | 🕗 0.2397274971
| 🇳  167 | ⭕️    6321952 | 🕗 0.2456494272
| 🇳  168 | ⭕️    6435520 | 🕗 0.2508569956
| 🇳  169 | ⭕️    6550440 | 🕗 0.2570057809
| 🇳  170 | ⭕️    6666720 | 🕗 0.2625517249
| 🇳  171 | ⭕️    6784368 | 🕗 0.2684110105
| 🇳  172 | ⭕️    6903392 | 🕗 0.2748391926
| 🇳  173 | ⭕️    7023800 | 🕗 0.2808277905
| 🇳  174 | ⭕️    7145600 | 🕗 0.2867560983
| 🇳  175 | ⭕️    7268800 | 🕗 0.2931822538
| 🇳  176 | ⭕️    7393408 | 🕗 0.2994064391
| 🇳  177 | ⭕️    7519432 | 🕗 0.3066619337
| 🇳  178 | ⭕️    7646880 | 🕗 0.3131026328
| 🇳  179 | ⭕️    7775760 | 🕗 0.3201743662
| 🇳  180 | ⭕️    7906080 | 🕗 0.3270924091
| 🇳  181 | ⭕️    8037848 | 🕗 0.3341746330
| 🇳  182 | ⭕️    8171072 | 🕗 0.3408803344
| 🇳  183 | ⭕️    8305760 | 🕗 0.3488851488
| 🇳  184 | ⭕️    8441920 | 🕗 0.3556617498
| 🇳  185 | ⭕️    8579560 | 🕗 0.3633066416
| 🇳  186 | ⭕️    8718688 | 🕗 0.3710851371
| 🇳  187 | ⭕️    8859312 | 🕗 0.3787183166
| 🇳  188 | ⭕️    9001440 | 🕗 0.3868706226
| 🇳  189 | ⭕️    9145080 | 🕗 0.3945339620
| 🇳  190 | ⭕️    9290240 | 🕗 0.4026339650
| 🇳  191 | ⭕️    9436928 | 🕗 0.4110474288
| 🇳  192 | ⭕️    9585152 | 🕗 0.4194793105
| 🇳  193 | ⭕️    9734920 | 🕗 0.4284542203
| 🇳  194 | ⭕️    9886240 | 🕗 0.4367388487
| 🇳  195 | ⭕️   10039120 | 🕗 0.4452858865
| 🇳  196 | ⭕️   10193568 | 🕗 0.4539607465
| 🇳  197 | ⭕️   10349592 | 🕗 0.4631373584
| 🇳  198 | ⭕️   10507200 | 🕗 0.4724363089
| 🇳  199 | ⭕️   10666400 | 🕗 0.4817487895
| 🇳  200 | ⭕️   10827200 | 🕗 0.4913620353
| 🇳  201 | ⭕️   10989608 | 🕗 0.5013707876
| 🇳  202 | ⭕️   11153632 | 🕗 0.5109505057
| 🇳  203 | ⭕️   11319280 | 🕗 0.5206042528
| 🇳  204 | ⭕️   11486560 | 🕗 0.5308341980
| 🇳  205 | ⭕️   11655480 | 🕗 0.5409295559
| 🇳  206 | ⭕️   11826048 | 🕗 0.5517295599
| 🇳  207 | ⭕️   11998272 | 🕗 0.5626879334
| 🇳  208 | ⭕️   12172160 | 🕗 0.5735034347
| 🇳  209 | ⭕️   12347720 | 🕗 0.5844789743
| 🇳  210 | ⭕️   12524960 | 🕗 0.5952639580
| 🇳  211 | ⭕️   12703888 | 🕗 0.6072149873
| 🇳  212 | ⭕️   12884512 | 🕗 0.6184394360
| 🇳  213 | ⭕️   13066840 | 🕗 0.6297327280
| 🇳  214 | ⭕️   13250880 | 🕗 0.6412607431
| 🇳  215 | ⭕️   13436640 | 🕗 0.6535311341
| 🇳  216 | ⭕️   13624128 | 🕗 0.6657137275
| 🇳  217 | ⭕️   13813352 | 🕗 0.6777288914
| 🇳  218 | ⭕️   14004320 | 🕗 0.6908183694
| 🇳  219 | ⭕️   14197040 | 🕗 0.7023741603
| 🇳  220 | ⭕️   14391520 | 🕗 0.7142126560
| 🇳  221 | ⭕️   14587768 | 🕗 0.7279970646
| 🇳  222 | ⭕️   14785792 | 🕗 0.7411111593
| 🇳  223 | ⭕️   14985600 | 🕗 0.7544422746
| 🇳  224 | ⭕️   15187200 | 🕗 0.7683255672
| 🇳  225 | ⭕️   15390600 | 🕗 0.7816805840
| 🇳  226 | ⭕️   15595808 | 🕗 0.7951261401
| 🇳  227 | ⭕️   15802832 | 🕗 0.8092628717
| 🇳  228 | ⭕️   16011680 | 🕗 0.8227874637
| 🇳  229 | ⭕️   16222360 | 🕗 0.8370870948
| 🇳  230 | ⭕️   16434880 | 🕗 0.8519513607
| 🇳  231 | ⭕️   16649248 | 🕗 0.8665815592
| 🇳  232 | ⭕️   16865472 | 🕗 0.8816897273
| 🇳  233 | ⭕️   17083560 | 🕗 0.8971861601
| 🇳  234 | ⭕️   17303520 | 🕗 0.9118126035
| 🇳  235 | ⭕️   17525360 | 🕗 0.9273353815
| 🇳  236 | ⭕️   17749088 | 🕗 0.9427413940
| 🇳  237 | ⭕️   17974712 | 🕗 0.9595806599
| 🇳  238 | ⭕️   18202240 | 🕗 0.9752129316
| 🇳  239 | ⭕️   18431680 | 🕗 0.9915909767
| 🇳  240 | ⭕️   18663040 | 🕗 1.0082355738
| 🇳  241 | ⭕️   18896328 | 🕗 1.0252707005
| 🇳  242 | ⭕️   19131552 | 🕗 1.0422573090
| 🇳  243 | ⭕️   19368720 | 🕗 1.0594242811
| 🇳  244 | ⭕️   19607840 | 🕗 1.0765631199
| 🇳  245 | ⭕️   19848920 | 🕗 1.0940006971
| 🇳  246 | ⭕️   20091968 | 🕗 1.1113030910
| 🇳  247 | ⭕️   20336992 | 🕗 1.1294479370
| 🇳  248 | ⭕️   20584000 | 🕗 1.1474530697
| 🇳  249 | ⭕️   20833000 | 🕗 1.1652510166
| 🇳  250 | ⭕️   21084000 | 🕗 1.1844950914
| 🇳  251 | ⭕️   21337008 | 🕗 1.2034733295
| 🇳  252 | ⭕️   21592032 | 🕗 1.2223532200
| 🇳  253 | ⭕️   21849080 | 🕗 1.2411861420
| 🇳  254 | ⭕️   22108160 | 🕗 1.2601187229
| 🇳  255 | ⭕️   22369280 | 🕗 1.2808616161
| 🇳  256 | ⭕️   22632448 | 🕗 1.3115926981
| 🇳  257 | ⭕️   22897672 | 🕗 1.3322868347
| 🇳  258 | ⭕️   23164960 | 🕗 1.3524920940
| 🇳  259 | ⭕️   23434320 | 🕗 1.3738888502
| 🇳  260 | ⭕️   23705760 | 🕗 1.3948390484
| 🇳  261 | ⭕️   23979288 | 🕗 1.4165151119
| 🇳  262 | ⭕️   24254912 | 🕗 1.4381496906
| 🇳  263 | ⭕️   24532640 | 🕗 1.4598348141
| 🇳  264 | ⭕️   24812480 | 🕗 1.4822442532
| 🇳  265 | ⭕️   25094440 | 🕗 1.5055291653
| 🇳  266 | ⭕️   25378528 | 🕗 1.5293437243
| 🇳  267 | ⭕️   25664752 | 🕗 1.5519559383
| 🇳  268 | ⭕️   25953120 | 🕗 1.5750749111
| 🇳  269 | ⭕️   26243640 | 🕗 1.5995900631
| 🇳  270 | ⭕️   26536320 | 🕗 1.6239156723
| 🇳  271 | ⭕️   26831168 | 🕗 1.6461290121
| 🇳  272 | ⭕️   27128192 | 🕗 1.6721776724
| 🇳  273 | ⭕️   27427400 | 🕗 1.6959779263
| 🇳  274 | ⭕️   27728800 | 🕗 1.7217242718
| 🇳  275 | ⭕️   28032400 | 🕗 1.7458661795
| 🇳  276 | ⭕️   28338208 | 🕗 1.7727648020
| 🇳  277 | ⭕️   28646232 | 🕗 1.7985424995
| 🇳  278 | ⭕️   28956480 | 🕗 1.8250669241
| 🇳  279 | ⭕️   29268960 | 🕗 1.8509416580
| 🇳  280 | ⭕️   29583680 | 🕗 1.8764822483
| 🇳  281 | ⭕️   29900648 | 🕗 1.9029159546
| 🇳  282 | ⭕️   30219872 | 🕗 1.9283620119
| 🇳  283 | ⭕️   30541360 | 🕗 1.9569215775
| 🇳  284 | ⭕️   30865120 | 🕗 1.9848431349
| 🇳  285 | ⭕️   31191160 | 🕗 2.0117216110
| 🇳  286 | ⭕️   31519488 | 🕗 2.0411350727
| 🇳  287 | ⭕️   31850112 | 🕗 2.0681724548
| 🇳  288 | ⭕️   32183040 | 🕗 2.0961732864
| 🇳  289 | ⭕️   32518280 | 🕗 2.1271593571
| 🇳  290 | ⭕️   32855840 | 🕗 2.1546497345
| 🇳  291 | ⭕️   33195728 | 🕗 2.1842732430
| 🇳  292 | ⭕️   33537952 | 🕗 2.2156043053
| 🇳  293 | ⭕️   33882520 | 🕗 2.2446796894
| 🇳  294 | ⭕️   34229440 | 🕗 2.2725489140
| 🇳  295 | ⭕️   34578720 | 🕗 2.3059878349
| 🇳  296 | ⭕️   34930368 | 🕗 2.3383712769
| 🇳  297 | ⭕️   35284392 | 🕗 2.3659286499
| 🇳  298 | ⭕️   35640800 | 🕗 2.3995804787
| 🇳  299 | ⭕️   35999600 | 🕗 2.4293415546
| 🇳  300 | ⭕️   36360800 | 🕗 2.4614112377
| 🇳  301 | ⭕️   36724408 | 🕗 2.4947695732
| 🇳  302 | ⭕️   37090432 | 🕗 2.5255017281
| 🇳  303 | ⭕️   37458880 | 🕗 2.5646390915
| 🇳  304 | ⭕️   37829760 | 🕗 2.5915014744
| 🇳  305 | ⭕️   38203080 | 🕗 2.6287050247
| 🇳  306 | ⭕️   38578848 | 🕗 2.6606783867
| 🇳  307 | ⭕️   38957072 | 🕗 2.6951565742
| 🇳  308 | ⭕️   39337760 | 🕗 2.7306163311
| 🇳  309 | ⭕️   39720920 | 🕗 2.7673854828
| 🇳  310 | ⭕️   40106560 | 🕗 2.8000831604
| 🇳  311 | ⭕️   40494688 | 🕗 2.8358983994
| 🇳  312 | ⭕️   40885312 | 🕗 2.8742041588
| 🇳  313 | ⭕️   41278440 | 🕗 2.9088292122
| 🇳  314 | ⭕️   41674080 | 🕗 2.9427518845
| 🇳  315 | ⭕️   42072240 | 🕗 2.9799244404
| 🇳  316 | ⭕️   42472928 | 🕗 3.0185284615
| 🇳  317 | ⭕️   42876152 | 🕗 3.0520420074
| 🇳  318 | ⭕️   43281920 | 🕗 3.0918805599
| 🇳  319 | ⭕️   43690240 | 🕗 3.1293289661
| 🇳  320 | ⭕️   44101120 | 🕗 3.1736469269
| 🇳  321 | ⭕️   44514568 | 🕗 3.2103056908
| 🇳  322 | ⭕️   44930592 | 🕗 3.2500233650
| 🇳  323 | ⭕️   45349200 | 🕗 3.2854855061
| 🇳  324 | ⭕️   45770400 | 🕗 3.3255665302
| 🇳  325 | ⭕️   46194200 | 🕗 3.3673853874
| 🇳  326 | ⭕️   46620608 | 🕗 3.4071111679
| 🇳  327 | ⭕️   47049632 | 🕗 3.4492669106
| 🇳  328 | ⭕️   47481280 | 🕗 3.4897911549
| 🇳  329 | ⭕️   47915560 | 🕗 3.5297830105
| 🇳  330 | ⭕️   48352480 | 🕗 3.5713343620
| 🇳  331 | ⭕️   48792048 | 🕗 3.6174914837
| 🇳  332 | ⭕️   49234272 | 🕗 3.6580235958
| 🇳  333 | ⭕️   49679160 | 🕗 3.6974585056
| 🇳  334 | ⭕️   50126720 | 🕗 3.7451782227
| 🇳  335 | ⭕️   50576960 | 🕗 3.7907252312
| 🇳  336 | ⭕️   51029888 | 🕗 3.8349757195
| 🇳  337 | ⭕️   51485512 | 🕗 3.8807768822
| 🇳  338 | ⭕️   51943840 | 🕗 3.9244723320
| 🇳  339 | ⭕️   52404880 | 🕗 3.9686069489
| 🇳  340 | ⭕️   52868640 | 🕗 4.0174245834
| 🇳  341 | ⭕️   53335128 | 🕗 4.0668721199
| 🇳  342 | ⭕️   53804352 | 🕗 4.1059412956
| 🇳  343 | ⭕️   54276320 | 🕗 4.1581983566
| 🇳  344 | ⭕️   54751040 | 🕗 4.2020859718
| 🇳  345 | ⭕️   55228520 | 🕗 4.2532401085
| 🇳  346 | ⭕️   55708768 | 🕗 4.3003206253
| 🇳  347 | ⭕️   56191792 | 🕗 4.3467669487
| 🇳  348 | ⭕️   56677600 | 🕗 4.3979878426
| 🇳  349 | ⭕️   57166200 | 🕗 4.4503231049
| 🇳  350 | ⭕️   57657600 | 🕗 4.5007662773
| 🇳  351 | ⭕️   58151808 | 🕗 4.5566487312
| 🇳  352 | ⭕️   58648832 | 🕗 4.6036014557
| 🇳  353 | ⭕️   59148680 | 🕗 4.6662559509
| 🇳  354 | ⭕️   59651360 | 🕗 4.7120108604
| 🇳  355 | ⭕️   60156880 | 🕗 4.7623639107
| 🇳  356 | ⭕️   60665248 | 🕗 4.8190956116
| 🇳  357 | ⭕️   61176472 | 🕗 4.8735785484
| 🇳  358 | ⭕️   61690560 | 🕗 4.9198369980
| 🇳  359 | ⭕️   62207520 | 🕗 4.9798874855
| 🇳  360 | ⭕️   62727360 | 🕗 5.0303201675
| 🇳  361 | ⭕️   63250088 | 🕗 5.0865416527
| 🇳  362 | ⭕️   63775712 | 🕗 5.1404547691
| 🇳  363 | ⭕️   64304240 | 🕗 5.1950888634
| 🇳  364 | ⭕️   64835680 | 🕗 5.2499880791
| 🇳  365 | ⭕️   65370040 | 🕗 5.3112354279
| 🇳  366 | ⭕️   65907328 | 🕗 5.3658051491
| 🇳  367 | ⭕️   66447552 | 🕗 5.4261894226
| 🇳  368 | ⭕️   66990720 | 🕗 5.4808568954
| 🇳  369 | ⭕️   67536840 | 🕗 5.5431914330
| 🇳  370 | ⭕️   68085920 | 🕗 5.5955600739
| 🇳  371 | ⭕️   68637968 | 🕗 5.6552810669
| 🇳  372 | ⭕️   69192992 | 🕗 5.7188529968
| 🇳  373 | ⭕️   69751000 | 🕗 5.7767558098
| 🇳  374 | ⭕️   70312000 | 🕗 5.8372354507
| 🇳  375 | ⭕️   70876000 | 🕗 5.8972616196
| 🇳  376 | ⭕️   71443008 | 🕗 5.9613471031
| 🇳  377 | ⭕️   72013032 | 🕗 6.0249500275
| 🇳  378 | ⭕️   72586080 | 🕗 6.0874500275
| 🇳  379 | ⭕️   73162160 | 🕗 6.1527752876
| 🇳  380 | ⭕️   73741280 | 🕗 6.2089700699
| 🇳  381 | ⭕️   74323448 | 🕗 6.2738652229
| 🇳  382 | ⭕️   74908672 | 🕗 6.3433127403
| 🇳  383 | ⭕️   75496960 | 🕗 6.4100208282
| 🇳  384 | ⭕️   76088320 | 🕗 6.4728174210
| 🇳  385 | ⭕️   76682760 | 🕗 6.5371317863
| 🇳  386 | ⭕️   77280288 | 🕗 6.6019163132
| 🇳  387 | ⭕️   77880912 | 🕗 6.6738834381
| 🇳  388 | ⭕️   78484640 | 🕗 6.7449893951
| 🇳  389 | ⭕️   79091480 | 🕗 6.8079924583
| 🇳  390 | ⭕️   79701440 | 🕗 6.8811230659
| 🇳  391 | ⭕️   80314528 | 🕗 6.9510841370
| 🇳  392 | ⭕️   80930752 | 🕗 7.0138792992
| 🇳  393 | ⭕️   81550120 | 🕗 7.0812439919
| 🇳  394 | ⭕️   82172640 | 🕗 7.1532983780
| 🇳  395 | ⭕️   82798320 | 🕗 7.2307343483
| 🇳  396 | ⭕️   83427168 | 🕗 7.3018150330
| 🇳  397 | ⭕️   84059192 | 🕗 7.3712115288
| 🇳  398 | ⭕️   84694400 | 🕗 7.4464097023
| 🇳  399 | ⭕️   85332800 | 🕗 7.5190477371
| 🇳  400 | ⭕️   85974400 | 🕗 7.5956964493
| 🇳  401 | ⭕️   86619208 | 🕗 7.6649732590
| 🇳  402 | ⭕️   87267232 | 🕗 7.7422270775
| 🇳  403 | ⭕️   87918480 | 🕗 7.8194770813
| 🇳  404 | ⭕️   88572960 | 🕗 7.8944811821
| 🇳  405 | ⭕️   89230680 | 🕗 7.9699692726
| 🇳  406 | ⭕️   89891648 | 🕗 8.0515079498
| 🇳  407 | ⭕️   90555872 | 🕗 8.1267957687
| 🇳  408 | ⭕️   91223360 | 🕗 8.2132091522
| 🇳  409 | ⭕️   91894120 | 🕗 8.2896041870
| 🇳  410 | ⭕️   92568160 | 🕗 8.3729152679
| 🇳  411 | ⭕️   93245488 | 🕗 8.4482173920
| 🇳  412 | ⭕️   93926112 | 🕗 8.5502243042
| 🇳  413 | ⭕️   94610040 | 🕗 8.6289949417
| 🇳  414 | ⭕️   95297280 | 🕗 8.7363681793
| 🇳  415 | ⭕️   95987840 | 🕗 8.7960853577
| 🇳  416 | ⭕️   96681728 | 🕗 8.9173231125
| 🇳  417 | ⭕️   97378952 | 🕗 8.9862174988
| 🇳  418 | ⭕️   98079520 | 🕗 9.0546131134
| 🇳  419 | ⭕️   98783440 | 🕗 9.1591930389
| 🇳  420 | ⭕️   99490720 | 🕗 9.2285337448
| 🇳  421 | ⭕️  100201368 | 🕗 9.3371515274
| 🇳  422 | ⭕️  100915392 | 🕗 9.4183073044
| 🇳  423 | ⭕️  101632800 | 🕗 9.5342674255
| 🇳  424 | ⭕️  102353600 | 🕗 9.5900802612
| 🇳  425 | ⭕️  103077800 | 🕗 9.6829538345
| 🇳  426 | ⭕️  103805408 | 🕗 9.7635841370
| 🇳  427 | ⭕️  104536432 | 🕗 9.8882331848 |
| 🇳  428 | ⭕️  105270880 | 🕗 9.9908752441 |
| 🇳  429 | ⭕️  106008760 | 🕗 10.0747499466 |
| 🇳  430 | ⭕️  106750080 | 🕗 10.1107807159 |
| 🇳  431 | ⭕️  107494848 | 🕗 10.2599391937 |
| 🇳  432 | ⭕️  108243072 | 🕗 10.3516559601 |
| 🇳  433 | ⭕️  108994760 | 🕗 10.4076585770 |
| 🇳  434 | ⭕️  109749920 | 🕗 10.5567102432 |
| 🇳  435 | ⭕️  110508560 | 🕗 10.6500167847 |
| 🇳  436 | ⭕️  111270688 | 🕗 10.7025203705 |
| 🇳  437 | ⭕️  112036312 | 🕗 10.7929964066 |
| 🇳  438 | ⭕️  112805440 | 🕗 10.8963155746 |
| 🇳  439 | ⭕️  113578080 | 🕗 10.9870491028 |
| 🇳  440 | ⭕️  114354240 | 🕗 11.0835409164 |
| 🇳  441 | ⭕️  115133928 | 🕗 11.2131023407 |
| 🇳  442 | ⭕️  115917152 | 🕗 11.2627811432 |
| 🇳  443 | ⭕️  116703920 | 🕗 11.3999443054 |
| 🇳  444 | ⭕️  117494240 | 🕗 11.4872903824 |
| 🇳  445 | ⭕️  118288120 | 🕗 11.6312246323 |
| 🇳  446 | ⭕️  119085568 | 🕗 11.7240867615 |
| 🇳  447 | ⭕️  119886592 | 🕗 11.7775325775 |
| 🇳  448 | ⭕️  120691200 | 🕗 11.9043340683 |
| 🇳  449 | ⭕️  121499400 | 🕗 12.0621032715 |
| 🇳  450 | ⭕️  122311200 | 🕗 12.1336994171 |
| 🇳  451 | ⭕️  123126608 | 🕗 12.2854757309 |
| 🇳  452 | ⭕️  123945632 | 🕗 12.3238658905 |
| 🇳  453 | ⭕️  124768280 | 🕗 12.4075069427 |
| 🇳  454 | ⭕️  125594560 | 🕗 12.5710077286 |
| 🇳  455 | ⭕️  126424480 | 🕗 12.8137054443 |
| 🇳  456 | ⭕️  127258048 | 🕗 12.7437534332 |
| 🇳  457 | ⭕️  128095272 | 🕗 12.8861846924 |
| 🇳  458 | ⭕️  128936160 | 🕗 13.0877838135 |
| 🇳  459 | ⭕️  129780720 | 🕗 13.0910482407 |
| 🇳  460 | ⭕️  130628960 | 🕗 13.4251565933 |
| 🇳  461 | ⭕️  131480888 | 🕗 13.3878412247 |
| 🇳  462 | ⭕️  132336512 | 🕗 13.6326112747 |
| 🇳  463 | ⭕️  133195840 | 🕗 13.5999450684 |
| 🇳  464 | ⭕️  134058880 | 🕗 13.7784500122 |
| 🇳  465 | ⭕️  134925640 | 🕗 13.7059822083 |
| 🇳  466 | ⭕️  135796128 | 🕗 13.9022006989 |
| 🇳  467 | ⭕️  136670352 | 🕗 14.2412424088 |
| 🇳  468 | ⭕️  137548320 | 🕗 14.1911993027 |
| 🇳  469 | ⭕️  138430040 | 🕗 14.2329750061 |
| 🇳  470 | ⭕️  139315520 | 🕗 14.3600616455 |
| 🇳  471 | ⭕️  140204768 | 🕗 14.5454578400 |
| 🇳  472 | ⭕️  141097792 | 🕗 14.6230211258 |
| 🇳  473 | ⭕️  141994600 | 🕗 14.9150714874 |
| 🇳  474 | ⭕️  142895200 | 🕗 14.8227090836 |
| 🇳  475 | ⭕️  143799600 | 🕗 15.2033672333 |
| 🇳  476 | ⭕️  144707808 | 🕗 15.0604906082 |
| 🇳  477 | ⭕️  145619832 | 🕗 15.1800317764 |
| 🇳  478 | ⭕️  146535680 | 🕗 15.3825578690 |
| 🇳  479 | ⭕️  147455360 | 🕗 15.5832738876 |
| 🇳  480 | ⭕️  148378880 | 🕗 15.8113288879 |
| 🇳  481 | ⭕️  149306248 | 🕗 15.7239618301 |
| 🇳  482 | ⭕️  150237472 | 🕗 15.8736791611 |
| 🇳  483 | ⭕️  151172560 | 🕗 15.9729957581 |
| 🇳  484 | ⭕️  152111520 | 🕗 16.1180419922 |
| 🇳  485 | ⭕️  153054360 | 🕗 16.2621192932 |
| 🇳  486 | ⭕️  154001088 | 🕗 16.3419265747 |
| 🇳  487 | ⭕️  154951712 | 🕗 16.4736919403 |
| 🇳  488 | ⭕️  155906240 | 🕗 16.5876770020 |
| 🇳  489 | ⭕️  156864680 | 🕗 16.7882080078 |
| 🇳  490 | ⭕️  157827040 | 🕗 16.9397983551 |
| 🇳  491 | ⭕️  158793328 | 🕗 17.0677185059 |
| 🇳  492 | ⭕️  159763552 | 🕗 17.1988563538 |
| 🇳  493 | ⭕️  160737720 | 🕗 17.3506336212 |
| 🇳  494 | ⭕️  161715840 | 🕗 17.6069602966 |
| 🇳  495 | ⭕️  162697920 | 🕗 17.6740055084 |
| 🇳  496 | ⭕️  163683968 | 🕗 17.7093238831 |
| 🇳  497 | ⭕️  164673992 | 🕗 17.8067722321 |
| 🇳  498 | ⭕️  165668000 | 🕗 18.0331249237 |
| 🇳  499 | ⭕️  166666000 | 🕗 18.5309829712 |
| 🇳  500 | ⭕️  167668000 | 🕗 18.2926750183 |
| 🇳  450 | ⭕️  122311200 | 🕗 12.1336994171 |
| 🇳  451 | ⭕️  123126608 | 🕗 12.2854757309 |
| 🇳  452 | ⭕️  123945632 | 🕗 12.3238658905 |
| 🇳  453 | ⭕️  124768280 | 🕗 12.4075069427 |
| 🇳  454 | ⭕️  125594560 | 🕗 12.5710077286 |
| 🇳  455 | ⭕️  126424480 | 🕗 12.8137054443 |
| 🇳  456 | ⭕️  127258048 | 🕗 12.7437534332 |
| 🇳  457 | ⭕️  128095272 | 🕗 12.8861846924 |
| 🇳  458 | ⭕️  128936160 | 🕗 13.0877838135 |
| 🇳  459 | ⭕️  129780720 | 🕗 13.0910482407 |
| 🇳  460 | ⭕️  130628960 | 🕗 13.4251565933 |
| 🇳  461 | ⭕️  131480888 | 🕗 13.3878412247 |
| 🇳  462 | ⭕️  132336512 | 🕗 13.6326112747 |
| 🇳  463 | ⭕️  133195840 | 🕗 13.5999450684 |
| 🇳  464 | ⭕️  134058880 | 🕗 13.7784500122 |
| 🇳  465 | ⭕️  134925640 | 🕗 13.7059822083 |
| 🇳  466 | ⭕️  135796128 | 🕗 13.9022006989 |
| 🇳  467 | ⭕️  136670352 | 🕗 14.2412424088 |
| 🇳  468 | ⭕️  137548320 | 🕗 14.1911993027 |
| 🇳  469 | ⭕️  138430040 | 🕗 14.2329750061 |
| 🇳  470 | ⭕️  139315520 | 🕗 14.3600616455 |
| 🇳  471 | ⭕️  140204768 | 🕗 14.5454578400 |
| 🇳  472 | ⭕️  141097792 | 🕗 14.6230211258 |
| 🇳  473 | ⭕️  141994600 | 🕗 14.9150714874 |
| 🇳  474 | ⭕️  142895200 | 🕗 14.8227090836 |
| 🇳  475 | ⭕️  143799600 | 🕗 15.2033672333 |
| 🇳  476 | ⭕️  144707808 | 🕗 15.0604906082 |
| 🇳  477 | ⭕️  145619832 | 🕗 15.1800317764 |
| 🇳  478 | ⭕️  146535680 | 🕗 15.3825578690 |
| 🇳  479 | ⭕️  147455360 | 🕗 15.5832738876 |
| 🇳  480 | ⭕️  148378880 | 🕗 15.8113288879 |
| 🇳  481 | ⭕️  149306248 | 🕗 15.7239618301 |
| 🇳  482 | ⭕️  150237472 | 🕗 15.8736791611 |
| 🇳  483 | ⭕️  151172560 | 🕗 15.9729957581 |
| 🇳  484 | ⭕️  152111520 | 🕗 16.1180419922 |
| 🇳  485 | ⭕️  153054360 | 🕗 16.2621192932 |
| 🇳  486 | ⭕️  154001088 | 🕗 16.3419265747 |
| 🇳  487 | ⭕️  154951712 | 🕗 16.4736919403 |
| 🇳  488 | ⭕️  155906240 | 🕗 16.5876770020 |
| 🇳  489 | ⭕️  156864680 | 🕗 16.7882080078 |
| 🇳  490 | ⭕️  157827040 | 🕗 16.9397983551 |
| 🇳  491 | ⭕️  158793328 | 🕗 17.0677185059 |
| 🇳  492 | ⭕️  159763552 | 🕗 17.1988563538 |
| 🇳  493 | ⭕️  160737720 | 🕗 17.3506336212 |
| 🇳  494 | ⭕️  161715840 | 🕗 17.6069602966 |
| 🇳  495 | ⭕️  162697920 | 🕗 17.6740055084 |
| 🇳  496 | ⭕️  163683968 | 🕗 17.7093238831 |
| 🇳  497 | ⭕️  164673992 | 🕗 17.8067722321 |
| 🇳  498 | ⭕️  165668000 | 🕗 18.0331249237 |
| 🇳  499 | ⭕️  166666000 | 🕗 18.5309829712 |
| 🇳  500 | ⭕️  167668000 | 🕗 18.2926750183 |
| 🇳  450 | ⭕️  122311200 | 🕗 12.1336994171 |
| 🇳  451 | ⭕️  123126608 | 🕗 12.2854757309 |
| 🇳  452 | ⭕️  123945632 | 🕗 12.3238658905 |
| 🇳  453 | ⭕️  124768280 | 🕗 12.4075069427 |
| 🇳  454 | ⭕️  125594560 | 🕗 12.5710077286 |
| 🇳  455 | ⭕️  126424480 | 🕗 12.8137054443 |
| 🇳  456 | ⭕️  127258048 | 🕗 12.7437534332 |
| 🇳  457 | ⭕️  128095272 | 🕗 12.8861846924 |
| 🇳  458 | ⭕️  128936160 | 🕗 13.0877838135 |
| 🇳  459 | ⭕️  129780720 | 🕗 13.0910482407 |
| 🇳  460 | ⭕️  130628960 | 🕗 13.4251565933 |
| 🇳  461 | ⭕️  131480888 | 🕗 13.3878412247 |
| 🇳  462 | ⭕️  132336512 | 🕗 13.6326112747 |
| 🇳  463 | ⭕️  133195840 | 🕗 13.5999450684 |
| 🇳  464 | ⭕️  134058880 | 🕗 13.7784500122 |
| 🇳  465 | ⭕️  134925640 | 🕗 13.7059822083 |
| 🇳  466 | ⭕️  135796128 | 🕗 13.9022006989 |
| 🇳  467 | ⭕️  136670352 | 🕗 14.2412424088 |
| 🇳  468 | ⭕️  137548320 | 🕗 14.1911993027 |
| 🇳  469 | ⭕️  138430040 | 🕗 14.2329750061 |
| 🇳  470 | ⭕️  139315520 | 🕗 14.3600616455 |
| 🇳  471 | ⭕️  140204768 | 🕗 14.5454578400 |
| 🇳  472 | ⭕️  141097792 | 🕗 14.6230211258 |
| 🇳  473 | ⭕️  141994600 | 🕗 14.9150714874 |
| 🇳  474 | ⭕️  142895200 | 🕗 14.8227090836 |
| 🇳  475 | ⭕️  143799600 | 🕗 15.2033672333 |
| 🇳  476 | ⭕️  144707808 | 🕗 15.0604906082 |
| 🇳  477 | ⭕️  145619832 | 🕗 15.1800317764 |
| 🇳  478 | ⭕️  146535680 | 🕗 15.3825578690 |
| 🇳  479 | ⭕️  147455360 | 🕗 15.5832738876 |
| 🇳  480 | ⭕️  148378880 | 🕗 15.8113288879 |
| 🇳  481 | ⭕️  149306248 | 🕗 15.7239618301 |
| 🇳  482 | ⭕️  150237472 | 🕗 15.8736791611 |
| 🇳  483 | ⭕️  151172560 | 🕗 15.9729957581 |
| 🇳  484 | ⭕️  152111520 | 🕗 16.1180419922 |
| 🇳  485 | ⭕️  153054360 | 🕗 16.2621192932 |
| 🇳  486 | ⭕️  154001088 | 🕗 16.3419265747 |
| 🇳  487 | ⭕️  154951712 | 🕗 16.4736919403 |
| 🇳  488 | ⭕️  155906240 | 🕗 16.5876770020 |
| 🇳  489 | ⭕️  156864680 | 🕗 16.7882080078 |
| 🇳  490 | ⭕️  157827040 | 🕗 16.9397983551 |
| 🇳  491 | ⭕️  158793328 | 🕗 17.0677185059 |
| 🇳  492 | ⭕️  159763552 | 🕗 17.1988563538 |
| 🇳  493 | ⭕️  160737720 | 🕗 17.3506336212 |
| 🇳  494 | ⭕️  161715840 | 🕗 17.6069602966 |
| 🇳  495 | ⭕️  162697920 | 🕗 17.6740055084 |
| 🇳  496 | ⭕️  163683968 | 🕗 17.7093238831 |
| 🇳  497 | ⭕️  164673992 | 🕗 17.8067722321 |
| 🇳  498 | ⭕️  165668000 | 🕗 18.0331249237 |
| 🇳  499 | ⭕️  166666000 | 🕗 18.5309829712 |
| 🇳  500 | ⭕️  167668000 | 🕗 18.2926750183 |
| 🇳  450 | ⭕️  122311200 | 🕗 12.1336994171 |
| 🇳  451 | ⭕️  123126608 | 🕗 12.2854757309 |
| 🇳  452 | ⭕️  123945632 | 🕗 12.3238658905 |
| 🇳  453 | ⭕️  124768280 | 🕗 12.4075069427 |
| 🇳  454 | ⭕️  125594560 | 🕗 12.5710077286 |
| 🇳  455 | ⭕️  126424480 | 🕗 12.8137054443 |
| 🇳  456 | ⭕️  127258048 | 🕗 12.7437534332 |
| 🇳  457 | ⭕️  128095272 | 🕗 12.8861846924 |
| 🇳  458 | ⭕️  128936160 | 🕗 13.0877838135 |
| 🇳  459 | ⭕️  129780720 | 🕗 13.0910482407 |
| 🇳  460 | ⭕️  130628960 | 🕗 13.4251565933 |
| 🇳  461 | ⭕️  131480888 | 🕗 13.3878412247 |
| 🇳  462 | ⭕️  132336512 | 🕗 13.6326112747 |
| 🇳  463 | ⭕️  133195840 | 🕗 13.5999450684 |
| 🇳  464 | ⭕️  134058880 | 🕗 13.7784500122 |
| 🇳  465 | ⭕️  134925640 | 🕗 13.7059822083 |
| 🇳  466 | ⭕️  135796128 | 🕗 13.9022006989 |
| 🇳  467 | ⭕️  136670352 | 🕗 14.2412424088 |
| 🇳  468 | ⭕️  137548320 | 🕗 14.1911993027 |
| 🇳  469 | ⭕️  138430040 | 🕗 14.2329750061 |
| 🇳  470 | ⭕️  139315520 | 🕗 14.3600616455 |
| 🇳  471 | ⭕️  140204768 | 🕗 14.5454578400 |
| 🇳  472 | ⭕️  141097792 | 🕗 14.6230211258 |
| 🇳  473 | ⭕️  141994600 | 🕗 14.9150714874 |
| 🇳  474 | ⭕️  142895200 | 🕗 14.8227090836 |
| 🇳  475 | ⭕️  143799600 | 🕗 15.2033672333 |
| 🇳  476 | ⭕️  144707808 | 🕗 15.0604906082 |
| 🇳  477 | ⭕️  145619832 | 🕗 15.1800317764 |
| 🇳  478 | ⭕️  146535680 | 🕗 15.3825578690 |
| 🇳  479 | ⭕️  147455360 | 🕗 15.5832738876 |
| 🇳  480 | ⭕️  148378880 | 🕗 15.8113288879 |
| 🇳  481 | ⭕️  149306248 | 🕗 15.7239618301 |
| 🇳  482 | ⭕️  150237472 | 🕗 15.8736791611 |
| 🇳  483 | ⭕️  151172560 | 🕗 15.9729957581 |
| 🇳  484 | ⭕️  152111520 | 🕗 16.1180419922 |
| 🇳  485 | ⭕️  153054360 | 🕗 16.2621192932 |
| 🇳  486 | ⭕️  154001088 | 🕗 16.3419265747 |
| 🇳  487 | ⭕️  154951712 | 🕗 16.4736919403 |
| 🇳  488 | ⭕️  155906240 | 🕗 16.5876770020 |
| 🇳  489 | ⭕️  156864680 | 🕗 16.7882080078 |
| 🇳  490 | ⭕️  157827040 | 🕗 16.9397983551 |
| 🇳  491 | ⭕️  158793328 | 🕗 17.0677185059 |
| 🇳  492 | ⭕️  159763552 | 🕗 17.1988563538 |
| 🇳  493 | ⭕️  160737720 | 🕗 17.3506336212 |
| 🇳  494 | ⭕️  161715840 | 🕗 17.6069602966 |
| 🇳  495 | ⭕️  162697920 | 🕗 17.6740055084 |
| 🇳  496 | ⭕️  163683968 | 🕗 17.7093238831 |
| 🇳  497 | ⭕️  164673992 | 🕗 17.8067722321 |
| 🇳  498 | ⭕️  165668000 | 🕗 18.0331249237 |
| 🇳  499 | ⭕️  166666000 | 🕗 18.5309829712 |
| 🇳  500 | ⭕️  167668000 | 🕗 18.2926750183 |
| 🇳  450 | ⭕️  122311200 | 🕗 12.1336994171 |
| 🇳  451 | ⭕️  123126608 | 🕗 12.2854757309 |
| 🇳  452 | ⭕️  123945632 | 🕗 12.3238658905 |
| 🇳  453 | ⭕️  124768280 | 🕗 12.4075069427 |
| 🇳  454 | ⭕️  125594560 | 🕗 12.5710077286 |
| 🇳  455 | ⭕️  126424480 | 🕗 12.8137054443 |
| 🇳  456 | ⭕️  127258048 | 🕗 12.7437534332 |
| 🇳  457 | ⭕️  128095272 | 🕗 12.8861846924 |
| 🇳  458 | ⭕️  128936160 | 🕗 13.0877838135 |
| 🇳  459 | ⭕️  129780720 | 🕗 13.0910482407 |
| 🇳  460 | ⭕️  130628960 | 🕗 13.4251565933 |
| 🇳  461 | ⭕️  131480888 | 🕗 13.3878412247 |
| 🇳  462 | ⭕️  132336512 | 🕗 13.6326112747 |
| 🇳  463 | ⭕️  133195840 | 🕗 13.5999450684 |
| 🇳  464 | ⭕️  134058880 | 🕗 13.7784500122 |
| 🇳  465 | ⭕️  134925640 | 🕗 13.7059822083 |
| 🇳  466 | ⭕️  135796128 | 🕗 13.9022006989 |
| 🇳  467 | ⭕️  136670352 | 🕗 14.2412424088 |
| 🇳  468 | ⭕️  137548320 | 🕗 14.1911993027 |
| 🇳  469 | ⭕️  138430040 | 🕗 14.2329750061 |
| 🇳  470 | ⭕️  139315520 | 🕗 14.3600616455 |
| 🇳  471 | ⭕️  140204768 | 🕗 14.5454578400 |
| 🇳  472 | ⭕️  141097792 | 🕗 14.6230211258 |
| 🇳  473 | ⭕️  141994600 | 🕗 14.9150714874 |
| 🇳  474 | ⭕️  142895200 | 🕗 14.8227090836 |
| 🇳  475 | ⭕️  143799600 | 🕗 15.2033672333 |
| 🇳  476 | ⭕️  144707808 | 🕗 15.0604906082 |
| 🇳  477 | ⭕️  145619832 | 🕗 15.1800317764 |
| 🇳  478 | ⭕️  146535680 | 🕗 15.3825578690 |
| 🇳  479 | ⭕️  147455360 | 🕗 15.5832738876 |
| 🇳  480 | ⭕️  148378880 | 🕗 15.8113288879 |
| 🇳  481 | ⭕️  149306248 | 🕗 15.7239618301 |
| 🇳  482 | ⭕️  150237472 | 🕗 15.8736791611 |
| 🇳  483 | ⭕️  151172560 | 🕗 15.9729957581 |
| 🇳  484 | ⭕️  152111520 | 🕗 16.1180419922 |
| 🇳  485 | ⭕️  153054360 | 🕗 16.2621192932 |
| 🇳  486 | ⭕️  154001088 | 🕗 16.3419265747 |
| 🇳  487 | ⭕️  154951712 | 🕗 16.4736919403 |
| 🇳  488 | ⭕️  155906240 | 🕗 16.5876770020 |
| 🇳  489 | ⭕️  156864680 | 🕗 16.7882080078 |
| 🇳  490 | ⭕️  157827040 | 🕗 16.9397983551 |
| 🇳  491 | ⭕️  158793328 | 🕗 17.0677185059 |
| 🇳  492 | ⭕️  159763552 | 🕗 17.1988563538 |
| 🇳  493 | ⭕️  160737720 | 🕗 17.3506336212 |
| 🇳  494 | ⭕️  161715840 | 🕗 17.6069602966 |
| 🇳  495 | ⭕️  162697920 | 🕗 17.6740055084 |
| 🇳  496 | ⭕️  163683968 | 🕗 17.7093238831 |
| 🇳  497 | ⭕️  164673992 | 🕗 17.8067722321 |
| 🇳  498 | ⭕️  165668000 | 🕗 18.0331249237 |
| 🇳  499 | ⭕️  166666000 | 🕗 18.5309829712 |
| 🇳  500 | ⭕️  167668000 | 🕗 18.2926750183 |
| 🇳  450 | ⭕️  122311200 | 🕗 12.1336994171 |
| 🇳  451 | ⭕️  123126608 | 🕗 12.2854757309 |
| 🇳  452 | ⭕️  123945632 | 🕗 12.3238658905 |
| 🇳  453 | ⭕️  124768280 | 🕗 12.4075069427 |
| 🇳  454 | ⭕️  125594560 | 🕗 12.5710077286 |
| 🇳  455 | ⭕️  126424480 | 🕗 12.8137054443 |
| 🇳  456 | ⭕️  127258048 | 🕗 12.7437534332 |
| 🇳  457 | ⭕️  128095272 | 🕗 12.8861846924 |
| 🇳  458 | ⭕️  128936160 | 🕗 13.0877838135 |
| 🇳  459 | ⭕️  129780720 | 🕗 13.0910482407 |
| 🇳  460 | ⭕️  130628960 | 🕗 13.4251565933 |
| 🇳  461 | ⭕️  131480888 | 🕗 13.3878412247 |
| 🇳  462 | ⭕️  132336512 | 🕗 13.6326112747 |
| 🇳  463 | ⭕️  133195840 | 🕗 13.5999450684 |
| 🇳  464 | ⭕️  134058880 | 🕗 13.7784500122 |
| 🇳  465 | ⭕️  134925640 | 🕗 13.7059822083 |
| 🇳  466 | ⭕️  135796128 | 🕗 13.9022006989 |
| 🇳  467 | ⭕️  136670352 | 🕗 14.2412424088 |
| 🇳  468 | ⭕️  137548320 | 🕗 14.1911993027 |
| 🇳  469 | ⭕️  138430040 | 🕗 14.2329750061 |
| 🇳  470 | ⭕️  139315520 | 🕗 14.3600616455 |
| 🇳  471 | ⭕️  140204768 | 🕗 14.5454578400 |
| 🇳  472 | ⭕️  141097792 | 🕗 14.6230211258 |
| 🇳  473 | ⭕️  141994600 | 🕗 14.9150714874 |
| 🇳  474 | ⭕️  142895200 | 🕗 14.8227090836 |
| 🇳  475 | ⭕️  143799600 | 🕗 15.2033672333 |
| 🇳  476 | ⭕️  144707808 | 🕗 15.0604906082 |
| 🇳  477 | ⭕️  145619832 | 🕗 15.1800317764 |
| 🇳  478 | ⭕️  146535680 | 🕗 15.3825578690 |
| 🇳  479 | ⭕️  147455360 | 🕗 15.5832738876 |
| 🇳  480 | ⭕️  148378880 | 🕗 15.8113288879 |
| 🇳  481 | ⭕️  149306248 | 🕗 15.7239618301 |
| 🇳  482 | ⭕️  150237472 | 🕗 15.8736791611 |
| 🇳  483 | ⭕️  151172560 | 🕗 15.9729957581 |
| 🇳  484 | ⭕️  152111520 | 🕗 16.1180419922 |
| 🇳  485 | ⭕️  153054360 | 🕗 16.2621192932 |
| 🇳  486 | ⭕️  154001088 | 🕗 16.3419265747 |
| 🇳  487 | ⭕️  154951712 | 🕗 16.4736919403 |
| 🇳  488 | ⭕️  155906240 | 🕗 16.5876770020 |
| 🇳  489 | ⭕️  156864680 | 🕗 16.7882080078 |
| 🇳  490 | ⭕️  157827040 | 🕗 16.9397983551 |
| 🇳  491 | ⭕️  158793328 | 🕗 17.0677185059 |
| 🇳  492 | ⭕️  159763552 | 🕗 17.1988563538 |
| 🇳  493 | ⭕️  160737720 | 🕗 17.3506336212 |
| 🇳  494 | ⭕️  161715840 | 🕗 17.6069602966 |
| 🇳  495 | ⭕️  162697920 | 🕗 17.6740055084 |
| 🇳  496 | ⭕️  163683968 | 🕗 17.7093238831 |
| 🇳  497 | ⭕️  164673992 | 🕗 17.8067722321 |
| 🇳  498 | ⭕️  165668000 | 🕗 18.0331249237 |
| 🇳  499 | ⭕️  166666000 | 🕗 18.5309829712 |
| 🇳  500 | ⭕️  167668000 | 🕗 18.2926750183 |
| 🇳  450 | ⭕️  122311200 | 🕗 12.1336994171 |
| 🇳  451 | ⭕️  123126608 | 🕗 12.2854757309 |
| 🇳  452 | ⭕️  123945632 | 🕗 12.3238658905 |
| 🇳  453 | ⭕️  124768280 | 🕗 12.4075069427 |
| 🇳  454 | ⭕️  125594560 | 🕗 12.5710077286 |
| 🇳  455 | ⭕️  126424480 | 🕗 12.8137054443 |
| 🇳  456 | ⭕️  127258048 | 🕗 12.7437534332 |
| 🇳  457 | ⭕️  128095272 | 🕗 12.8861846924 |
| 🇳  458 | ⭕️  128936160 | 🕗 13.0877838135 |
| 🇳  459 | ⭕️  129780720 | 🕗 13.0910482407 |
| 🇳  460 | ⭕️  130628960 | 🕗 13.4251565933 |
| 🇳  461 | ⭕️  131480888 | 🕗 13.3878412247 |
| 🇳  462 | ⭕️  132336512 | 🕗 13.6326112747 |
| 🇳  463 | ⭕️  133195840 | 🕗 13.5999450684 |
| 🇳  464 | ⭕️  134058880 | 🕗 13.7784500122 |
| 🇳  465 | ⭕️  134925640 | 🕗 13.7059822083 |
| 🇳  466 | ⭕️  135796128 | 🕗 13.9022006989 |
| 🇳  467 | ⭕️  136670352 | 🕗 14.2412424088 |
| 🇳  468 | ⭕️  137548320 | 🕗 14.1911993027 |
| 🇳  469 | ⭕️  138430040 | 🕗 14.2329750061 |
| 🇳  470 | ⭕️  139315520 | 🕗 14.3600616455 |
| 🇳  471 | ⭕️  140204768 | 🕗 14.5454578400 |
| 🇳  472 | ⭕️  141097792 | 🕗 14.6230211258 |
| 🇳  473 | ⭕️  141994600 | 🕗 14.9150714874 |
| 🇳  474 | ⭕️  142895200 | 🕗 14.8227090836 |
| 🇳  475 | ⭕️  143799600 | 🕗 15.2033672333 |
| 🇳  476 | ⭕️  144707808 | 🕗 15.0604906082 |
| 🇳  477 | ⭕️  145619832 | 🕗 15.1800317764 |
| 🇳  478 | ⭕️  146535680 | 🕗 15.3825578690 |
| 🇳  479 | ⭕️  147455360 | 🕗 15.5832738876 |
| 🇳  480 | ⭕️  148378880 | 🕗 15.8113288879 |
| 🇳  481 | ⭕️  149306248 | 🕗 15.7239618301 |
| 🇳  482 | ⭕️  150237472 | 🕗 15.8736791611 |
| 🇳  483 | ⭕️  151172560 | 🕗 15.9729957581 |
| 🇳  484 | ⭕️  152111520 | 🕗 16.1180419922 |
| 🇳  485 | ⭕️  153054360 | 🕗 16.2621192932 |
| 🇳  486 | ⭕️  154001088 | 🕗 16.3419265747 |
| 🇳  487 | ⭕️  154951712 | 🕗 16.4736919403 |
| 🇳  488 | ⭕️  155906240 | 🕗 16.5876770020 |
| 🇳  489 | ⭕️  156864680 | 🕗 16.7882080078 |
| 🇳  490 | ⭕️  157827040 | 🕗 16.9397983551 |
| 🇳  491 | ⭕️  158793328 | 🕗 17.0677185059 |
| 🇳  492 | ⭕️  159763552 | 🕗 17.1988563538 |
| 🇳  493 | ⭕️  160737720 | 🕗 17.3506336212 |
| 🇳  494 | ⭕️  161715840 | 🕗 17.6069602966 |
| 🇳  495 | ⭕️  162697920 | 🕗 17.6740055084 |
| 🇳  496 | ⭕️  163683968 | 🕗 17.7093238831 |
| 🇳  497 | ⭕️  164673992 | 🕗 17.8067722321 |
| 🇳  498 | ⭕️  165668000 | 🕗 18.0331249237 |
| 🇳  499 | ⭕️  166666000 | 🕗 18.5309829712 |
| 🇳  500 | ⭕️  167668000 | 🕗 18.2926750183 |
| 🇳  450 | ⭕️  122311200 | 🕗 12.1336994171 |
| 🇳  451 | ⭕️  123126608 | 🕗 12.2854757309 |
| 🇳  452 | ⭕️  123945632 | 🕗 12.3238658905 |
| 🇳  453 | ⭕️  124768280 | 🕗 12.4075069427 |
| 🇳  454 | ⭕️  125594560 | 🕗 12.5710077286 |
| 🇳  455 | ⭕️  126424480 | 🕗 12.8137054443 |
| 🇳  456 | ⭕️  127258048 | 🕗 12.7437534332 |
| 🇳  457 | ⭕️  128095272 | 🕗 12.8861846924 |
| 🇳  458 | ⭕️  128936160 | 🕗 13.0877838135 |
| 🇳  459 | ⭕️  129780720 | 🕗 13.0910482407 |
| 🇳  460 | ⭕️  130628960 | 🕗 13.4251565933 |
| 🇳  461 | ⭕️  131480888 | 🕗 13.3878412247 |
| 🇳  462 | ⭕️  132336512 | 🕗 13.6326112747 |
| 🇳  463 | ⭕️  133195840 | 🕗 13.5999450684 |
| 🇳  464 | ⭕️  134058880 | 🕗 13.7784500122 |
| 🇳  465 | ⭕️  134925640 | 🕗 13.7059822083 |
| 🇳  466 | ⭕️  135796128 | 🕗 13.9022006989 |
| 🇳  467 | ⭕️  136670352 | 🕗 14.2412424088 |
| 🇳  468 | ⭕️  137548320 | 🕗 14.1911993027 |
| 🇳  469 | ⭕️  138430040 | 🕗 14.2329750061 |
| 🇳  470 | ⭕️  139315520 | 🕗 14.3600616455 |
| 🇳  471 | ⭕️  140204768 | 🕗 14.5454578400 |
| 🇳  472 | ⭕️  141097792 | 🕗 14.6230211258 |
| 🇳  473 | ⭕️  141994600 | 🕗 14.9150714874 |
| 🇳  474 | ⭕️  142895200 | 🕗 14.8227090836 |
| 🇳  475 | ⭕️  143799600 | 🕗 15.2033672333 |
| 🇳  476 | ⭕️  144707808 | 🕗 15.0604906082 |
| 🇳  477 | ⭕️  145619832 | 🕗 15.1800317764 |
| 🇳  478 | ⭕️  146535680 | 🕗 15.3825578690 |
| 🇳  479 | ⭕️  147455360 | 🕗 15.5832738876 |
| 🇳  480 | ⭕️  148378880 | 🕗 15.8113288879 |
| 🇳  481 | ⭕️  149306248 | 🕗 15.7239618301 |
| 🇳  482 | ⭕️  150237472 | 🕗 15.8736791611 |
| 🇳  483 | ⭕️  151172560 | 🕗 15.9729957581 |
| 🇳  484 | ⭕️  152111520 | 🕗 16.1180419922 |
| 🇳  485 | ⭕️  153054360 | 🕗 16.2621192932 |
| 🇳  486 | ⭕️  154001088 | 🕗 16.3419265747 |
| 🇳  487 | ⭕️  154951712 | 🕗 16.4736919403 |
| 🇳  488 | ⭕️  155906240 | 🕗 16.5876770020 |
| 🇳  489 | ⭕️  156864680 | 🕗 16.7882080078 |
| 🇳  490 | ⭕️  157827040 | 🕗 16.9397983551 |
| 🇳  491 | ⭕️  158793328 | 🕗 17.0677185059 |
| 🇳  492 | ⭕️  159763552 | 🕗 17.1988563538 |
| 🇳  493 | ⭕️  160737720 | 🕗 17.3506336212 |
| 🇳  494 | ⭕️  161715840 | 🕗 17.6069602966 |
| 🇳  495 | ⭕️  162697920 | 🕗 17.6740055084 |
| 🇳  496 | ⭕️  163683968 | 🕗 17.7093238831 |
| 🇳  497 | ⭕️  164673992 | 🕗 17.8067722321 |
| 🇳  498 | ⭕️  165668000 | 🕗 18.0331249237 |
| 🇳  499 | ⭕️  166666000 | 🕗 18.5309829712 |
| 🇳  500 | ⭕️  167668000 | 🕗 18.2926750183 |
| 🇳  450 | ⭕️  122311200 | 🕗 12.1336994171 |
| 🇳  451 | ⭕️  123126608 | 🕗 12.2854757309 |
| 🇳  452 | ⭕️  123945632 | 🕗 12.3238658905 |
| 🇳  453 | ⭕️  124768280 | 🕗 12.4075069427 |
| 🇳  454 | ⭕️  125594560 | 🕗 12.5710077286 |
| 🇳  455 | ⭕️  126424480 | 🕗 12.8137054443 |
| 🇳  456 | ⭕️  127258048 | 🕗 12.7437534332 |
| 🇳  457 | ⭕️  128095272 | 🕗 12.8861846924 |
| 🇳  458 | ⭕️  128936160 | 🕗 13.0877838135 |
| 🇳  459 | ⭕️  129780720 | 🕗 13.0910482407 |
| 🇳  460 | ⭕️  130628960 | 🕗 13.4251565933 |
| 🇳  461 | ⭕️  131480888 | 🕗 13.3878412247 |
| 🇳  462 | ⭕️  132336512 | 🕗 13.6326112747 |
| 🇳  463 | ⭕️  133195840 | 🕗 13.5999450684 |
| 🇳  464 | ⭕️  134058880 | 🕗 13.7784500122 |
| 🇳  465 | ⭕️  134925640 | 🕗 13.7059822083 |
| 🇳  466 | ⭕️  135796128 | 🕗 13.9022006989 |
| 🇳  467 | ⭕️  136670352 | 🕗 14.2412424088 |
| 🇳  468 | ⭕️  137548320 | 🕗 14.1911993027 |
| 🇳  469 | ⭕️  138430040 | 🕗 14.2329750061 |
| 🇳  470 | ⭕️  139315520 | 🕗 14.3600616455 |
| 🇳  471 | ⭕️  140204768 | 🕗 14.5454578400 |
| 🇳  472 | ⭕️  141097792 | 🕗 14.6230211258 |
| 🇳  473 | ⭕️  141994600 | 🕗 14.9150714874 |
| 🇳  474 | ⭕️  142895200 | 🕗 14.8227090836 |
| 🇳  475 | ⭕️  143799600 | 🕗 15.2033672333 |
| 🇳  476 | ⭕️  144707808 | 🕗 15.0604906082 |
| 🇳  477 | ⭕️  145619832 | 🕗 15.1800317764 |
| 🇳  478 | ⭕️  146535680 | 🕗 15.3825578690 |
| 🇳  479 | ⭕️  147455360 | 🕗 15.5832738876 |
| 🇳  480 | ⭕️  148378880 | 🕗 15.8113288879 |
| 🇳  481 | ⭕️  149306248 | 🕗 15.7239618301 |
| 🇳  482 | ⭕️  150237472 | 🕗 15.8736791611 |
| 🇳  483 | ⭕️  151172560 | 🕗 15.9729957581 |
| 🇳  484 | ⭕️  152111520 | 🕗 16.1180419922 |
| 🇳  485 | ⭕️  153054360 | 🕗 16.2621192932 |
| 🇳  486 | ⭕️  154001088 | 🕗 16.3419265747 |
| 🇳  487 | ⭕️  154951712 | 🕗 16.4736919403 |
| 🇳  488 | ⭕️  155906240 | 🕗 16.5876770020 |
| 🇳  489 | ⭕️  156864680 | 🕗 16.7882080078 |
| 🇳  490 | ⭕️  157827040 | 🕗 16.9397983551 |
| 🇳  491 | ⭕️  158793328 | 🕗 17.0677185059 |
| 🇳  492 | ⭕️  159763552 | 🕗 17.1988563538 |
| 🇳  493 | ⭕️  160737720 | 🕗 17.3506336212 |
| 🇳  494 | ⭕️  161715840 | 🕗 17.6069602966 |
| 🇳  495 | ⭕️  162697920 | 🕗 17.6740055084 |
| 🇳  496 | ⭕️  163683968 | 🕗 17.7093238831 |
| 🇳  497 | ⭕️  164673992 | 🕗 17.8067722321 |
| 🇳  498 | ⭕️  165668000 | 🕗 18.0331249237 |
| 🇳  499 | ⭕️  166666000 | 🕗 18.5309829712 |
| 🇳  500 | ⭕️  167668000 | 🕗 18.2926750183 |
| 🇳  600 | ⭕️  289441600 | 🕗 38.3527984619 |
| 🇳  700 | ⭕️  459295200 | 🕗 73.8402404785 |
| 🇳  800 | ⭕️  685228800 | 🕗 186.9543304443 |
M2 Mac Mini Above
Linux Below
| 🇳  900 | ⭕️  975242400 | 🕗 342.9434509277 |
| 🇳 1000 | ⭕️ 1337336000 | 🕗 521.8120117188 | 
| 🇳 1100 | ⭕️ 1779509600 | 🕗 708.8403320312 |
| 🇳 1200 | ⭕️ 2309763200 | 🕗 997.7574462891 |
| 🇳 1300 | ⭕️ 2936096800 | 🕗 1373.807495112 |
| 🇳 1400 | ⭕️ 3666510400 | 🕗 1870.465454101 |
| 🇳 1500 | ⭕️ 4509004000 | 🕗 2431.391845703 |
| 🇳 1600 | ⭕️ 5471577600 | 🕗 3118.060791015 |
| 🇳 1700 | ⭕️ 6562231200 | 🕗 3904.761718750 |
| 🇳 1800 | ⭕️ 7788964800 | 🕗 4924.760742187 |
```
<br>
<br>
<br>
<a name="section-g"></a>

## [⇪](#toc) Licensing

This package is licensed under the MIT license.
 
Thanks for making it this far!
