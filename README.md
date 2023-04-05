
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
![Running times from 8 to 68,085,920 vertices](imgs/8_to_212million.png?raw=true "Runtimes up to 212 million")
8_to_68085920.png

#### Running times for the first 600 instances: graphs with 8 to 289_441_600 vertices (to be continued until 1000th order (over 1 billion)):
```
```
| 🇳    1 | ⭕️          8 | 🕗 0.0013815890 |
| 🇳    2 | ⭕️         32 | 🕗 0.0000047190 |
| 🇳    3 | ⭕️         80 | 🕗 0.0004273100 |
| 🇳    4 | ⭕️        160 | 🕗 0.0003583790 |
| 🇳    5 | ⭕️        280 | 🕗 0.0003242040 |
| 🇳    6 | ⭕️        448 | 🕗 0.0005798590 |
| 🇳    7 | ⭕️        672 | 🕗 0.0005324490 |
| 🇳    8 | ⭕️        960 | 🕗 0.0006872120 |
| 🇳    9 | ⭕️       1320 | 🕗 0.0006456840 |
| 🇳   10 | ⭕️       1760 | 🕗 0.0007734560 |
| 🇳   11 | ⭕️       2288 | 🕗 0.0006803800 |
| 🇳   12 | ⭕️       2912 | 🕗 0.0005741980 |
| 🇳   13 | ⭕️       3640 | 🕗 0.0009652490 |
| 🇳   14 | ⭕️       4480 | 🕗 0.0011231290 |
| 🇳   15 | ⭕️       5440 | 🕗 0.0014634430 |
| 🇳   16 | ⭕️       6528 | 🕗 0.0017408280 |
| 🇳   17 | ⭕️       7752 | 🕗 0.0015450580 |
| 🇳   18 | ⭕️       9120 | 🕗 0.0016512200 |
| 🇳   19 | ⭕️      10640 | 🕗 0.0022574479 |
| 🇳   20 | ⭕️      12320 | 🕗 0.0022435819 |
| 🇳   21 | ⭕️      14168 | 🕗 0.0020163101 |
| 🇳   22 | ⭕️      16192 | 🕗 0.0024271200 |
| 🇳   23 | ⭕️      18400 | 🕗 0.0026344829 |
| 🇳   24 | ⭕️      20800 | 🕗 0.0036933280 |
| 🇳   25 | ⭕️      23400 | 🕗 0.0022895690 |
| 🇳   26 | ⭕️      26208 | 🕗 0.0024642900 |
| 🇳   27 | ⭕️      29232 | 🕗 0.0028946560 |
| 🇳   28 | ⭕️      32480 | 🕗 0.0030366839 |
| 🇳   29 | ⭕️      35960 | 🕗 0.0033189601 |
| 🇳   30 | ⭕️      39680 | 🕗 0.0038182761 |
| 🇳   31 | ⭕️      43648 | 🕗 0.0059783491 |
| 🇳   32 | ⭕️      47872 | 🕗 0.0048374678 |
| 🇳   33 | ⭕️      52360 | 🕗 0.0044528078 |
| 🇳   34 | ⭕️      57120 | 🕗 0.0053012762 |
| 🇳   35 | ⭕️      62160 | 🕗 0.0059690508 |
| 🇳   36 | ⭕️      67488 | 🕗 0.0065855798 |
| 🇳   37 | ⭕️      73112 | 🕗 0.0072165160 |
| 🇳   38 | ⭕️      79040 | 🕗 0.0090882033 |
| 🇳   39 | ⭕️      85280 | 🕗 0.0081540020 |
| 🇳   40 | ⭕️      91840 | 🕗 0.0094635142 |
| 🇳   41 | ⭕️      98728 | 🕗 0.0110895354 |
| 🇳   42 | ⭕️     105952 | 🕗 0.0087080318 |
| 🇳   43 | ⭕️     113520 | 🕗 0.0099840797 |
| 🇳   44 | ⭕️     121440 | 🕗 0.0102632986 |
| 🇳   45 | ⭕️     129720 | 🕗 0.0115072671 |
| 🇳   46 | ⭕️     138368 | 🕗 0.0116306208 |
| 🇳   47 | ⭕️     147392 | 🕗 0.0138647445 |
| 🇳   48 | ⭕️     156800 | 🕗 0.0167386103 |
| 🇳   49 | ⭕️     166600 | 🕗 0.0144543722 |
| 🇳   50 | ⭕️     176800 | 🕗 0.0198183563 |
| 🇳   51 | ⭕️     187408 | 🕗 0.0175391585 |
| 🇳   52 | ⭕️     198432 | 🕗 0.0195233971 |
| 🇳   53 | ⭕️     209880 | 🕗 0.0205503125 |
| 🇳   54 | ⭕️     221760 | 🕗 0.0214725528 |
| 🇳   55 | ⭕️     234080 | 🕗 0.0231290516 |
| 🇳   56 | ⭕️     246848 | 🕗 0.0235543475 |
| 🇳   57 | ⭕️     260072 | 🕗 0.0253237002 |
| 🇳   58 | ⭕️     273760 | 🕗 0.0276365522 |
| 🇳   59 | ⭕️     287920 | 🕗 0.0313731134 |
| 🇳   60 | ⭕️     302560 | 🕗 0.0319934897 |
| 🇳   61 | ⭕️     317688 | 🕗 0.0301596634 |
| 🇳   62 | ⭕️     333312 | 🕗 0.0429670550 |
| 🇳   63 | ⭕️     349440 | 🕗 0.0351847596 |
| 🇳   64 | ⭕️     366080 | 🕗 0.0374890454 |
| 🇳   65 | ⭕️     383240 | 🕗 0.0375859663 |
| 🇳   66 | ⭕️     400928 | 🕗 0.0422700830 |
| 🇳   67 | ⭕️     419152 | 🕗 0.0437570512 |
| 🇳   68 | ⭕️     437920 | 🕗 0.0563508272 |
| 🇳   69 | ⭕️     457240 | 🕗 0.0620375052 |
| 🇳   70 | ⭕️     477120 | 🕗 0.0645392239 |
| 🇳   71 | ⭕️     497568 | 🕗 0.0601396486 |
| 🇳   72 | ⭕️     518592 | 🕗 0.0594751611 |
| 🇳   73 | ⭕️     540200 | 🕗 0.0683005825 |
| 🇳   74 | ⭕️     562400 | 🕗 0.0792794898 |
| 🇳   75 | ⭕️     585200 | 🕗 0.0695413500 |
| 🇳   76 | ⭕️     608608 | 🕗 0.0746364072 |
| 🇳   77 | ⭕️     632632 | 🕗 0.0688159615 |
| 🇳   78 | ⭕️     657280 | 🕗 0.0758416504 |
| 🇳   79 | ⭕️     682560 | 🕗 0.0850964412 |
| 🇳   80 | ⭕️     708480 | 🕗 0.0953197330 |
| 🇳   81 | ⭕️     735048 | 🕗 0.0912824944 |
| 🇳   82 | ⭕️     762272 | 🕗 0.0947953090 |
| 🇳   83 | ⭕️     790160 | 🕗 0.0901399255 |
| 🇳   84 | ⭕️     818720 | 🕗 0.1004497111 |
| 🇳   85 | ⭕️     847960 | 🕗 0.0963472724 |
| 🇳   86 | ⭕️     877888 | 🕗 0.1060554460 |
| 🇳   87 | ⭕️     908512 | 🕗 0.1214653254 |
| 🇳   88 | ⭕️     939840 | 🕗 0.1256983131 |
| 🇳   89 | ⭕️     971880 | 🕗 0.1291872859 |
| 🇳   90 | ⭕️    1004640 | 🕗 0.1137835011 |
| 🇳   91 | ⭕️    1038128 | 🕗 0.1265137345 |
| 🇳   92 | ⭕️    1072352 | 🕗 0.1358175129 |
| 🇳   93 | ⭕️    1107320 | 🕗 0.1377061456 |
| 🇳   94 | ⭕️    1143040 | 🕗 0.1639890969 |
| 🇳   95 | ⭕️    1179520 | 🕗 0.1559798270 |
| 🇳   96 | ⭕️    1216768 | 🕗 0.1598809212 |
| 🇳   97 | ⭕️    1254792 | 🕗 0.1573577970 |
| 🇳   98 | ⭕️    1293600 | 🕗 0.1591622829 |
| 🇳   99 | ⭕️    1333200 | 🕗 0.2105180770 |
| 🇳  100 | ⭕️    1373600 | 🕗 0.2028523982 |
| 🇳  101 | ⭕️    1414808 | 🕗 0.1836721897 |
| 🇳  102 | ⭕️    1456832 | 🕗 0.2267440110 |
| 🇳  103 | ⭕️    1499680 | 🕗 0.1992289871 |
| 🇳  104 | ⭕️    1543360 | 🕗 0.2160534561 |
| 🇳  105 | ⭕️    1587880 | 🕗 0.2106008381 |
| 🇳  106 | ⭕️    1633248 | 🕗 0.2794747055 |
| 🇳  107 | ⭕️    1679472 | 🕗 0.2587340772 |
| 🇳  108 | ⭕️    1726560 | 🕗 0.2457456589 |
| 🇳  109 | ⭕️    1774520 | 🕗 0.2541573942 |
| 🇳  110 | ⭕️    1823360 | 🕗 0.2630845010 |
| 🇳  111 | ⭕️    1873088 | 🕗 0.2910682261 |
| 🇳  112 | ⭕️    1923712 | 🕗 0.2734142840 |
| 🇳  113 | ⭕️    1975240 | 🕗 0.2844256461 |
| 🇳  114 | ⭕️    2027680 | 🕗 0.2864318192 |
| 🇳  115 | ⭕️    2081040 | 🕗 0.2935339212 |
| 🇳  116 | ⭕️    2135328 | 🕗 0.2903203964 |
| 🇳  117 | ⭕️    2190552 | 🕗 0.3320993185 |
| 🇳  118 | ⭕️    2246720 | 🕗 0.3376709223 |
| 🇳  119 | ⭕️    2303840 | 🕗 0.3629279137 |
| 🇳  120 | ⭕️    2361920 | 🕗 0.3450326324 |
| 🇳  121 | ⭕️    2420968 | 🕗 0.3637551069 |
| 🇳  122 | ⭕️    2480992 | 🕗 0.3873646259 |
| 🇳  123 | ⭕️    2542000 | 🕗 0.4243555665 |
| 🇳  124 | ⭕️    2604000 | 🕗 0.4968822300 |
| 🇳  125 | ⭕️    2667000 | 🕗 0.4486365616 |
| 🇳  126 | ⭕️    2731008 | 🕗 0.4752750099 |
| 🇳  127 | ⭕️    2796032 | 🕗 0.5031830668 |
| 🇳  128 | ⭕️    2862080 | 🕗 0.5629584789 |
| 🇳  129 | ⭕️    2929160 | 🕗 0.5307385325 |
| 🇳  130 | ⭕️    2997280 | 🕗 0.6431633234 |
| 🇳  131 | ⭕️    3066448 | 🕗 0.5947707891 |
| 🇳  132 | ⭕️    3136672 | 🕗 0.6278721690 |
| 🇳  133 | ⭕️    3207960 | 🕗 0.6104190946 |
| 🇳  134 | ⭕️    3280320 | 🕗 0.5906074643 |
| 🇳  135 | ⭕️    3353760 | 🕗 0.5725651383 |
| 🇳  136 | ⭕️    3428288 | 🕗 0.5971797705 |
| 🇳  137 | ⭕️    3503912 | 🕗 0.6396141648 |
| 🇳  138 | ⭕️    3580640 | 🕗 0.6131388545 |
| 🇳  139 | ⭕️    3658480 | 🕗 0.6537435651 |
| 🇳  140 | ⭕️    3737440 | 🕗 0.6820777059 |
| 🇳  141 | ⭕️    3817528 | 🕗 0.6827752590 |
| 🇳  142 | ⭕️    3898752 | 🕗 0.7198037505 |
| 🇳  143 | ⭕️    3981120 | 🕗 0.7211180329 |
| 🇳  144 | ⭕️    4064640 | 🕗 0.7327196598 |
| 🇳  145 | ⭕️    4149320 | 🕗 0.7185481191 |
| 🇳  146 | ⭕️    4235168 | 🕗 0.7745695114 |
| 🇳  147 | ⭕️    4322192 | 🕗 0.8233369589 |
| 🇳  148 | ⭕️    4410400 | 🕗 1.0346111059 |
| 🇳  149 | ⭕️    4499800 | 🕗 1.0777467489 |
| 🇳  150 | ⭕️    4590400 | 🕗 1.0748878717 |
| 🇳  151 | ⭕️    4682208 | 🕗 0.9352809191 |
| 🇳  152 | ⭕️    4775232 | 🕗 0.9224188924 |
| 🇳  153 | ⭕️    4869480 | 🕗 0.9224092960 |
| 🇳  154 | ⭕️    4964960 | 🕗 0.9753056765 |
| 🇳  155 | ⭕️    5061680 | 🕗 0.9809852242 |
| 🇳  156 | ⭕️    5159648 | 🕗 1.0483398438 |
| 🇳  157 | ⭕️    5258872 | 🕗 1.0276982784 |
| 🇳  158 | ⭕️    5359360 | 🕗 1.1488850117 |
| 🇳  159 | ⭕️    5461120 | 🕗 1.1187808514 |
| 🇳  160 | ⭕️    5564160 | 🕗 1.2848486900 |
| 🇳  161 | ⭕️    5668488 | 🕗 1.3224966526 |
| 🇳  162 | ⭕️    5774112 | 🕗 1.3345160484 |
| 🇳  163 | ⭕️    5881040 | 🕗 1.2587999105 |
| 🇳  164 | ⭕️    5989280 | 🕗 1.2779357433 |
| 🇳  165 | ⭕️    6098840 | 🕗 1.2951819897 |
| 🇳  166 | ⭕️    6209728 | 🕗 1.3413314819 |
| 🇳  167 | ⭕️    6321952 | 🕗 1.3838578463 |
| 🇳  168 | ⭕️    6435520 | 🕗 1.3832068443 |
| 🇳  169 | ⭕️    6550440 | 🕗 1.4060752392 |
| 🇳  170 | ⭕️    6666720 | 🕗 1.5043284893 |
| 🇳  171 | ⭕️    6784368 | 🕗 1.4942501783 |
| 🇳  172 | ⭕️    6903392 | 🕗 1.5141285658 |
| 🇳  173 | ⭕️    7023800 | 🕗 1.5810283422 |
| 🇳  174 | ⭕️    7145600 | 🕗 1.7361406088 |
| 🇳  175 | ⭕️    7268800 | 🕗 1.9859375954 |
| 🇳  176 | ⭕️    7393408 | 🕗 2.6358499527 |
| 🇳  177 | ⭕️    7519432 | 🕗 2.0919463634 |
| 🇳  178 | ⭕️    7646880 | 🕗 1.7816535234 |
| 🇳  179 | ⭕️    7775760 | 🕗 1.8637257814 |
| 🇳  180 | ⭕️    7906080 | 🕗 1.8031185865 |
| 🇳  181 | ⭕️    8037848 | 🕗 1.8590803146 |
| 🇳  182 | ⭕️    8171072 | 🕗 1.8977694511 |
| 🇳  183 | ⭕️    8305760 | 🕗 1.9599635601 |
| 🇳  184 | ⭕️    8441920 | 🕗 2.1053178310 |
| 🇳  185 | ⭕️    8579560 | 🕗 2.2886521816 |
| 🇳  186 | ⭕️    8718688 | 🕗 2.8832197189 |
| 🇳  187 | ⭕️    8859312 | 🕗 2.6861686707 |
| 🇳  188 | ⭕️    9001440 | 🕗 3.1165597439 |
| 🇳  189 | ⭕️    9145080 | 🕗 2.5396134853 |
| 🇳  190 | ⭕️    9290240 | 🕗 2.4893870354 |
| 🇳  191 | ⭕️    9436928 | 🕗 2.5434436798 |
| 🇳  192 | ⭕️    9585152 | 🕗 2.4617903233 |
| 🇳  193 | ⭕️    9734920 | 🕗 2.6870172024 |
| 🇳  194 | ⭕️    9886240 | 🕗 2.5153274536 |
| 🇳  195 | ⭕️   10039120 | 🕗 2.4850392342 |
| 🇳  196 | ⭕️   10193568 | 🕗 2.6596279144 |
| 🇳  197 | ⭕️   10349592 | 🕗 2.6796290874 |
| 🇳  198 | ⭕️   10507200 | 🕗 2.7295646667 |
| 🇳  199 | ⭕️   10666400 | 🕗 2.8960919380 |
| 🇳  200 | ⭕️   10827200 | 🕗 3.1209578514 |
| 🇳  201 | ⭕️   10989608 | 🕗 2.9911346436 |
| 🇳  202 | ⭕️   11153632 | 🕗 2.9830625057 |
| 🇳  203 | ⭕️   11319280 | 🕗 3.7317581177 |
| 🇳  204 | ⭕️   11486560 | 🕗 3.3774478436 |
| 🇳  205 | ⭕️   11655480 | 🕗 3.1285121441 |
| 🇳  206 | ⭕️   11826048 | 🕗 3.2658522129 |
| 🇳  207 | ⭕️   11998272 | 🕗 3.8468670845 |
| 🇳  208 | ⭕️   12172160 | 🕗 3.7233178616 |
| 🇳  209 | ⭕️   12347720 | 🕗 3.3892743587 |
| 🇳  210 | ⭕️   12524960 | 🕗 3.5047724247 |
| 🇳  211 | ⭕️   12703888 | 🕗 4.1274061203 |
| 🇳  212 | ⭕️   12884512 | 🕗 3.5303590298 |
| 🇳  213 | ⭕️   13066840 | 🕗 3.5769639015 |
| 🇳  214 | ⭕️   13250880 | 🕗 4.5297980309 |
| 🇳  215 | ⭕️   13436640 | 🕗 3.7222216129 |
| 🇳  216 | ⭕️   13624128 | 🕗 3.8963844776 |
| 🇳  217 | ⭕️   13813352 | 🕗 4.3604688644 |
| 🇳  218 | ⭕️   14004320 | 🕗 4.0260019302 |
| 🇳  219 | ⭕️   14197040 | 🕗 4.0104618073 |
| 🇳  220 | ⭕️   14391520 | 🕗 4.2878665924 |
| 🇳  221 | ⭕️   14587768 | 🕗 4.2024683952 |
| 🇳  222 | ⭕️   14785792 | 🕗 4.9093794823 |
| 🇳  223 | ⭕️   14985600 | 🕗 4.8352146149 |
| 🇳  224 | ⭕️   15187200 | 🕗 4.5246958733 |
| 🇳  225 | ⭕️   15390600 | 🕗 4.8359103203 |
| 🇳  226 | ⭕️   15595808 | 🕗 4.8411083221 |
| 🇳  227 | ⭕️   15802832 | 🕗 4.6224822998 |
| 🇳  228 | ⭕️   16011680 | 🕗 4.8841900826 |
| 🇳  229 | ⭕️   16222360 | 🕗 5.1741619110 |
| 🇳  230 | ⭕️   16434880 | 🕗 5.8134331703 |
| 🇳  231 | ⭕️   16649248 | 🕗 4.9385428429 |
| 🇳  232 | ⭕️   16865472 | 🕗 5.5792074203 |
| 🇳  233 | ⭕️   17083560 | 🕗 5.4336562157 |
| 🇳  234 | ⭕️   17303520 | 🕗 5.3514013290 |
| 🇳  235 | ⭕️   17525360 | 🕗 5.9311618805 |
| 🇳  236 | ⭕️   17749088 | 🕗 5.3011960983 |
| 🇳  237 | ⭕️   17974712 | 🕗 5.5103974342 |
| 🇳  238 | ⭕️   18202240 | 🕗 6.1222605705 |
| 🇳  239 | ⭕️   18431680 | 🕗 6.0094895363 |
| 🇳  240 | ⭕️   18663040 | 🕗 5.6875782013 |
| 🇳  241 | ⭕️   18896328 | 🕗 6.4585180283 |
| 🇳  242 | ⭕️   19131552 | 🕗 5.8726687431 |
| 🇳  243 | ⭕️   19368720 | 🕗 5.9654445648 |
| 🇳  244 | ⭕️   19607840 | 🕗 6.9278464317 |
| 🇳  245 | ⭕️   19848920 | 🕗 6.4381299019 |
| 🇳  246 | ⭕️   20091968 | 🕗 6.4328861237 |
| 🇳  247 | ⭕️   20336992 | 🕗 6.7413506508 |
| 🇳  248 | ⭕️   20584000 | 🕗 7.5305848122 |
| 🇳  249 | ⭕️   20833000 | 🕗 6.5739173889 |
| 🇳  250 | ⭕️   21084000 | 🕗 7.4616861343 |
| 🇳  251 | ⭕️   21337008 | 🕗 7.1235918999 |
| 🇳  252 | ⭕️   21592032 | 🕗 6.7630677223 |
| 🇳  253 | ⭕️   21849080 | 🕗 8.1116666794 |
| 🇳  254 | ⭕️   22108160 | 🕗 7.3529558182 |
| 🇳  255 | ⭕️   22369280 | 🕗 7.4940605164 |
| 🇳  256 | ⭕️   22632448 | 🕗 8.9494037628 |
| 🇳  257 | ⭕️   22897672 | 🕗 7.8305282593 |
| 🇳  258 | ⭕️   23164960 | 🕗 8.2293262482 |
| 🇳  259 | ⭕️   23434320 | 🕗 8.1858673096 |
| 🇳  260 | ⭕️   23705760 | 🕗 8.0858602524 |
| 🇳  261 | ⭕️   23979288 | 🕗 9.3044519424 |
| 🇳  262 | ⭕️   24254912 | 🕗 8.2203483582 |
| 🇳  263 | ⭕️   24532640 | 🕗 9.4362258911 |
| 🇳  264 | ⭕️   24812480 | 🕗 9.4275264740 |
| 🇳  265 | ⭕️   25094440 | 🕗 9.2320680618 |
| 🇳  266 | ⭕️   25378528 | 🕗 9.0984687805 |
| 🇳  267 | ⭕️   25664752 | 🕗 9.1893873215 |
| 🇳  268 | ⭕️   25953120 | 🕗 9.2806978226 |
| 🇳  269 | ⭕️   26243640 | 🕗 9.5917491913 |
| 🇳  270 | ⭕️   26536320 | 🕗 9.1295166016 |
| 🇳  271 | ⭕️   26831168 | 🕗 9.9818038940 |
| 🇳  272 | ⭕️   27128192 | 🕗 9.9199790955 |
| 🇳  273 | ⭕️   27427400 | 🕗 10.2309617996 |
| 🇳  274 | ⭕️   27728800 | 🕗 10.1564455032 |
| 🇳  275 | ⭕️   28032400 | 🕗 10.9352989197 |
| 🇳  276 | ⭕️   28338208 | 🕗 10.8171520233 |
| 🇳  277 | ⭕️   28646232 | 🕗 10.2971982956 |
| 🇳  278 | ⭕️   28956480 | 🕗 11.7717514038 |
| 🇳  279 | ⭕️   29268960 | 🕗 11.1220169067 |
| 🇳  280 | ⭕️   29583680 | 🕗 11.4783992767 |
| 🇳  281 | ⭕️   29900648 | 🕗 11.0132217407 |
| 🇳  282 | ⭕️   30219872 | 🕗 11.6977148056 |
| 🇳  283 | ⭕️   30541360 | 🕗 11.5704269409 |
| 🇳  284 | ⭕️   30865120 | 🕗 12.3991012573 |
| 🇳  285 | ⭕️   31191160 | 🕗 12.0284099579 |
| 🇳  286 | ⭕️   31519488 | 🕗 11.9793939590 |
| 🇳  287 | ⭕️   31850112 | 🕗 12.4586954117 |
| 🇳  288 | ⭕️   32183040 | 🕗 13.1374807358 |
| 🇳  289 | ⭕️   32518280 | 🕗 12.9363136292 |
| 🇳  290 | ⭕️   32855840 | 🕗 13.3402795792 |
| 🇳  291 | ⭕️   33195728 | 🕗 13.1627445221 |
| 🇳  292 | ⭕️   33537952 | 🕗 13.3746948242 |
| 🇳  293 | ⭕️   33882520 | 🕗 13.6081752777 |
| 🇳  294 | ⭕️   34229440 | 🕗 13.7417860031 |
| 🇳  295 | ⭕️   34578720 | 🕗 14.0405597687 |
| 🇳  296 | ⭕️   34930368 | 🕗 14.6279935837 |
| 🇳  297 | ⭕️   35284392 | 🕗 14.0838956833 |
| 🇳  298 | ⭕️   35640800 | 🕗 14.5655708313 |
| 🇳  299 | ⭕️   35999600 | 🕗 14.9607343674 |
| 🇳  300 | ⭕️   36360800 | 🕗 15.0482511520 |
| 🇳  301 | ⭕️   36724408 | 🕗 14.0759820938 |
| 🇳  302 | ⭕️   37090432 | 🕗 14.4914464951 |
| 🇳  303 | ⭕️   37458880 | 🕗 15.6623172760 |
| 🇳  304 | ⭕️   37829760 | 🕗 16.2128677368 |
| 🇳  305 | ⭕️   38203080 | 🕗 15.7918472290 |
| 🇳  306 | ⭕️   38578848 | 🕗 16.2204990387 |
| 🇳  307 | ⭕️   38957072 | 🕗 16.1805191040 |
| 🇳  308 | ⭕️   39337760 | 🕗 16.7655181885 |
| 🇳  309 | ⭕️   39720920 | 🕗 16.1063842773 |
| 🇳  310 | ⭕️   40106560 | 🕗 17.0181922913 |
| 🇳  311 | ⭕️   40494688 | 🕗 16.9932193756 |
| 🇳  312 | ⭕️   40885312 | 🕗 17.4907760620 |
| 🇳  313 | ⭕️   41278440 | 🕗 18.0010566711 |
| 🇳  314 | ⭕️   41674080 | 🕗 17.1483039856 |
| 🇳  315 | ⭕️   42072240 | 🕗 17.6603832245 |
| 🇳  316 | ⭕️   42472928 | 🕗 17.6451873779 |
| 🇳  317 | ⭕️   42876152 | 🕗 18.7257156372 |
| 🇳  318 | ⭕️   43281920 | 🕗 20.5910301208 |
| 🇳  319 | ⭕️   43690240 | 🕗 18.4504089355 |
| 🇳  320 | ⭕️   44101120 | 🕗 20.2118263245 |
| 🇳  321 | ⭕️   44514568 | 🕗 20.4877414703 |
| 🇳  322 | ⭕️   44930592 | 🕗 20.8660869598 |
| 🇳  323 | ⭕️   45349200 | 🕗 21.6280918121 |
| 🇳  324 | ⭕️   45770400 | 🕗 19.9407539368 |
| 🇳  325 | ⭕️   46194200 | 🕗 21.6425800323 |
| 🇳  326 | ⭕️   46620608 | 🕗 21.0318336487 |
| 🇳  327 | ⭕️   47049632 | 🕗 21.3481063843 |
| 🇳  328 | ⭕️   47481280 | 🕗 20.4672451019 |
| 🇳  329 | ⭕️   47915560 | 🕗 21.1295375824 |
| 🇳  330 | ⭕️   48352480 | 🕗 22.0267238617 |
| 🇳  331 | ⭕️   48792048 | 🕗 20.4863185883 |
| 🇳  332 | ⭕️   49234272 | 🕗 21.7202587128 |
| 🇳  333 | ⭕️   49679160 | 🕗 21.4830627441 |
| 🇳  334 | ⭕️   50126720 | 🕗 21.8682289124 |
| 🇳  335 | ⭕️   50576960 | 🕗 21.9534950256 |
| 🇳  336 | ⭕️   51029888 | 🕗 22.3893661499 |
| 🇳  337 | ⭕️   51485512 | 🕗 22.7789802551 |
| 🇳  338 | ⭕️   51943840 | 🕗 24.2893199921 |
| 🇳  339 | ⭕️   52404880 | 🕗 23.8776817322 |
| 🇳  340 | ⭕️   52868640 | 🕗 25.1674728394 |
| 🇳  341 | ⭕️   53335128 | 🕗 24.3233795166 |
| 🇳  342 | ⭕️   53804352 | 🕗 24.6200008392 |
| 🇳  343 | ⭕️   54276320 | 🕗 24.9462871552 |
| 🇳  344 | ⭕️   54751040 | 🕗 24.7288360596 |
| 🇳  345 | ⭕️   55228520 | 🕗 26.2907333374 |
| 🇳  346 | ⭕️   55708768 | 🕗 26.2195720673 |
| 🇳  347 | ⭕️   56191792 | 🕗 25.5412483215 |
| 🇳  348 | ⭕️   56677600 | 🕗 27.0444698334 |
| 🇳  349 | ⭕️   57166200 | 🕗 26.8865680695 |
| 🇳  350 | ⭕️   57657600 | 🕗 26.6446723938 |
| 🇳  351 | ⭕️   58151808 | 🕗 26.8374538422 |
| 🇳  352 | ⭕️   58648832 | 🕗 27.7784824371 |
| 🇳  353 | ⭕️   59148680 | 🕗 27.2609729767 |
| 🇳  354 | ⭕️   59651360 | 🕗 29.5786533356 |
| 🇳  355 | ⭕️   60156880 | 🕗 29.3769245148 |
| 🇳  356 | ⭕️   60665248 | 🕗 30.3442916870 |
| 🇳  357 | ⭕️   61176472 | 🕗 29.3335876465 |
| 🇳  358 | ⭕️   61690560 | 🕗 29.1410408020 |
| 🇳  359 | ⭕️   62207520 | 🕗 30.4601860046 |
| 🇳  360 | ⭕️   62727360 | 🕗 29.6040382385 |
| 🇳  361 | ⭕️   63250088 | 🕗 30.6567668915 |
| 🇳  362 | ⭕️   63775712 | 🕗 30.6122932434 |
| 🇳  363 | ⭕️   64304240 | 🕗 31.0433120728 |
| 🇳  364 | ⭕️   64835680 | 🕗 31.2733287811 |
| 🇳  365 | ⭕️   65370040 | 🕗 30.8189163208 |
| 🇳  366 | ⭕️   65907328 | 🕗 33.0198173523 |
| 🇳  367 | ⭕️   66447552 | 🕗 33.5019264221 |
| 🇳  368 | ⭕️   66990720 | 🕗 33.6000595093 |
| 🇳  369 | ⭕️   67536840 | 🕗 33.1757698059 |
| 🇳  370 | ⭕️   68085920 | 🕗 34.4215621948 |
| 🇳  371 | ⭕️   68637968 | 🕗 34.9661064148 |
| 🇳  372 | ⭕️   69192992 | 🕗 36.3701744080 |
| 🇳  373 | ⭕️   69751000 | 🕗 34.4412193298 |
| 🇳  374 | ⭕️   70312000 | 🕗 36.2738380432 |
| 🇳  375 | ⭕️   70876000 | 🕗 35.3459968567 |
| 🇳  376 | ⭕️   71443008 | 🕗 35.4334678650 |
| 🇳  377 | ⭕️   72013032 | 🕗 36.6368865967 |
| 🇳  378 | ⭕️   72586080 | 🕗 36.2929458618 |
| 🇳  379 | ⭕️   73162160 | 🕗 36.6776351929 |
| 🇳  380 | ⭕️   73741280 | 🕗 37.4549636841 |
| 🇳  381 | ⭕️   74323448 | 🕗 37.9030914307 |
| 🇳  382 | ⭕️   74908672 | 🕗 38.3143386841 |
| 🇳  383 | ⭕️   75496960 | 🕗 37.7345504761 |
| 🇳  384 | ⭕️   76088320 | 🕗 38.7556114197 |
| 🇳  385 | ⭕️   76682760 | 🕗 36.4428558350 |
| 🇳  386 | ⭕️   77280288 | 🕗 39.9526329041 |
| 🇳  387 | ⭕️   77880912 | 🕗 40.6358413696 |
| 🇳  388 | ⭕️   78484640 | 🕗 40.6154251099 |
| 🇳  389 | ⭕️   79091480 | 🕗 41.2153434753 |
| 🇳  390 | ⭕️   79701440 | 🕗 40.5099067688 |
| 🇳  391 | ⭕️   80314528 | 🕗 41.3164024353 |
| 🇳  392 | ⭕️   80930752 | 🕗 41.9001388550 |
| 🇳  393 | ⭕️   81550120 | 🕗 40.5671806335 |
| 🇳  394 | ⭕️   82172640 | 🕗 42.1694183350 |
| 🇳  395 | ⭕️   82798320 | 🕗 43.0273056030 |
| 🇳  396 | ⭕️   83427168 | 🕗 44.0190849304 |
| 🇳  397 | ⭕️   84059192 | 🕗 41.3468856812 |
| 🇳  398 | ⭕️   84694400 | 🕗 43.5366668701 |
| 🇳  399 | ⭕️   85332800 | 🕗 45.5460395813 |
| 🇳  400 | ⭕️   85974400 | 🕗 44.5925559998 |
| 🇳  401 | ⭕️   86619208 | 🕗 43.8678588867 |
| 🇳  402 | ⭕️   87267232 | 🕗 43.3589668274 |
| 🇳  403 | ⭕️   87918480 | 🕗 47.1697235107 |
| 🇳  404 | ⭕️   88572960 | 🕗 47.7294960022 |
| 🇳  405 | ⭕️   89230680 | 🕗 46.9262619019 |
| 🇳  406 | ⭕️   89891648 | 🕗 46.5659446716 |
| 🇳  407 | ⭕️   90555872 | 🕗 48.5598030090 |
| 🇳  408 | ⭕️   91223360 | 🕗 49.2453002930 |
| 🇳  409 | ⭕️   91894120 | 🕗 50.2071037292 |
| 🇳  410 | ⭕️   92568160 | 🕗 49.0292129517 |
| 🇳  411 | ⭕️   93245488 | 🕗 48.4471931458 |
| 🇳  412 | ⭕️   93926112 | 🕗 50.8128547668 |
| 🇳  413 | ⭕️   94610040 | 🕗 49.7855606079 |
| 🇳  414 | ⭕️   95297280 | 🕗 52.6560859680 |
| 🇳  415 | ⭕️   95987840 | 🕗 52.1597785950 |
| 🇳  416 | ⭕️   96681728 | 🕗 51.0061836243 |
| 🇳  417 | ⭕️   97378952 | 🕗 54.3934898376 |
| 🇳  418 | ⭕️   98079520 | 🕗 54.1354484558 |
| 🇳  419 | ⭕️   98783440 | 🕗 55.0203552246 |
| 🇳  420 | ⭕️   99490720 | 🕗 55.2032051086 |
| 🇳  421 | ⭕️  100201368 | 🕗 56.0544548035 |
| 🇳  422 | ⭕️  100915392 | 🕗 56.8572006226 |
| 🇳  423 | ⭕️  101632800 | 🕗 54.9829940796 |
| 🇳  424 | ⭕️  102353600 | 🕗 56.5563888550 |
| 🇳  425 | ⭕️  103077800 | 🕗 59.8030967712 |
| 🇳  426 | ⭕️  103805408 | 🕗 61.4458618164 |
| 🇳  427 | ⭕️  104536432 | 🕗 61.9797286987 |
| 🇳  428 | ⭕️  105270880 | 🕗 61.5555953979 |
| 🇳  429 | ⭕️  106008760 | 🕗 63.1129264832 |
| 🇳  430 | ⭕️  106750080 | 🕗 63.0090217590 |
| 🇳  431 | ⭕️  107494848 | 🕗 61.7442588806 |
| 🇳  432 | ⭕️  108243072 | 🕗 64.9224548340 |
| 🇳  433 | ⭕️  108994760 | 🕗 66.8498687744 |
| 🇳  434 | ⭕️  109749920 | 🕗 64.5297393799 |
| 🇳  435 | ⭕️  110508560 | 🕗 65.0109863281 |
| 🇳  436 | ⭕️  111270688 | 🕗 68.2749481201 |
| 🇳  437 | ⭕️  112036312 | 🕗 66.1378402710 |
| 🇳  438 | ⭕️  112805440 | 🕗 67.7823257446 |
| 🇳  439 | ⭕️  113578080 | 🕗 67.4901504517 |
| 🇳  440 | ⭕️  114354240 | 🕗 68.9601135254 |
| 🇳  441 | ⭕️  115133928 | 🕗 69.2714080811 |
| 🇳  442 | ⭕️  115917152 | 🕗 72.2484817505 |
| 🇳  443 | ⭕️  116703920 | 🕗 71.1025390625 |
| 🇳  444 | ⭕️  117494240 | 🕗 73.8542175293 |
| 🇳  445 | ⭕️  118288120 | 🕗 71.3532943726 |
| 🇳  446 | ⭕️  119085568 | 🕗 72.5653762817 |
| 🇳  447 | ⭕️  119886592 | 🕗 72.5122299194 |
| 🇳  448 | ⭕️  120691200 | 🕗 74.0412216187 |
| 🇳  449 | ⭕️  121499400 | 🕗 72.4873657227 |
| 🇳  450 | ⭕️  122311200 | 🕗 76.7459411621 |
| 🇳  451 | ⭕️  123126608 | 🕗 75.6177291870 |
| 🇳  452 | ⭕️  123945632 | 🕗 77.9286422729 |
| 🇳  453 | ⭕️  124768280 | 🕗 74.8584289551 |
| 🇳  454 | ⭕️  125594560 | 🕗 77.5533218384 |
| 🇳  455 | ⭕️  126424480 | 🕗 78.9352340698 |
| 🇳  456 | ⭕️  127258048 | 🕗 79.2335281372 |
| 🇳  457 | ⭕️  128095272 | 🕗 75.8029098511 |
| 🇳  458 | ⭕️  128936160 | 🕗 81.7742996216 |
| 🇳  459 | ⭕️  129780720 | 🕗 81.3206481934 |
| 🇳  460 | ⭕️  130628960 | 🕗 83.2125625610 |
| 🇳  461 | ⭕️  131480888 | 🕗 81.2322845459 |
| 🇳  462 | ⭕️  132336512 | 🕗 84.6842956543 |
| 🇳  463 | ⭕️  133195840 | 🕗 84.7790756226 |
| 🇳  464 | ⭕️  134058880 | 🕗 89.0183563232 |
| 🇳  465 | ⭕️  134925640 | 🕗 89.4054489136 |
| 🇳  466 | ⭕️  135796128 | 🕗 86.0402603149 |
| 🇳  467 | ⭕️  136670352 | 🕗 88.8748245239 |
| 🇳  468 | ⭕️  137548320 | 🕗 89.6920700073 |
| 🇳  469 | ⭕️  138430040 | 🕗 88.8107528687 |
| 🇳  470 | ⭕️  139315520 | 🕗 89.2064590454 |
| 🇳  471 | ⭕️  140204768 | 🕗 90.1898956299 |
| 🇳  472 | ⭕️  141097792 | 🕗 91.8132476807 |
| 🇳  473 | ⭕️  141994600 | 🕗 92.8922424316 |
| 🇳  474 | ⭕️  142895200 | 🕗 89.8424911499 |
| 🇳  475 | ⭕️  143799600 | 🕗 94.0037612915 |
| 🇳  476 | ⭕️  144707808 | 🕗 93.2743072510 |
| 🇳  477 | ⭕️  145619832 | 🕗 93.6512832642 |
| 🇳  478 | ⭕️  146535680 | 🕗 98.2833786011 |
| 🇳  479 | ⭕️  147455360 | 🕗 94.7246780396 |
| 🇳  480 | ⭕️  148378880 | 🕗 99.7933349609 |
| 🇳  481 | ⭕️  149306248 | 🕗 95.1066207886 |
| 🇳  482 | ⭕️  150237472 | 🕗 101.6403656006 |
| 🇳  483 | ⭕️  151172560 | 🕗 100.1924743652 |
| 🇳  484 | ⭕️  152111520 | 🕗 100.5447006226 |
| 🇳  485 | ⭕️  153054360 | 🕗 103.5607299805 |
| 🇳  486 | ⭕️  154001088 | 🕗 103.2894973755 |
| 🇳  487 | ⭕️  154951712 | 🕗 105.7483749390 |
| 🇳  488 | ⭕️  155906240 | 🕗 104.8427276611 |
| 🇳  489 | ⭕️  156864680 | 🕗 101.3816528320 |
| 🇳  490 | ⭕️  157827040 | 🕗 106.9203872681 |
| 🇳  491 | ⭕️  158793328 | 🕗 110.4528808594 |
| 🇳  492 | ⭕️  159763552 | 🕗 111.5629730225 |
| 🇳  493 | ⭕️  160737720 | 🕗 108.3564453125 |
| 🇳  494 | ⭕️  161715840 | 🕗 110.9057159424 |
| 🇳  495 | ⭕️  162697920 | 🕗 110.7699203491 |
| 🇳  496 | ⭕️  163683968 | 🕗 109.5451049805 |
| 🇳  497 | ⭕️  164673992 | 🕗 112.1879043579 |
| 🇳  498 | ⭕️  165668000 | 🕗 112.0320816040 |
| 🇳  499 | ⭕️  166666000 | 🕗 113.5061264038 |
| 🇳  500 | ⭕️  167668000 | 🕗 119.1129989624 |
| 🇳  501 | ⭕️  168674008 | 🕗 116.0884628296 |
| 🇳  502 | ⭕️  169684032 | 🕗 115.6823348999 |
| 🇳  503 | ⭕️  170698080 | 🕗 116.1075439453 |
| 🇳  504 | ⭕️  171716160 | 🕗 120.8041458130 |
| 🇳  505 | ⭕️  172738280 | 🕗 120.0512008667 |
| 🇳  506 | ⭕️  173764448 | 🕗 125.8657989502 |
| 🇳  507 | ⭕️  174794672 | 🕗 119.8752441406 |
| 🇳  508 | ⭕️  175828960 | 🕗 119.9067306519 |
| 🇳  509 | ⭕️  176867320 | 🕗 124.3516845703 |
| 🇳  510 | ⭕️  177909760 | 🕗 118.1283416748 |
| 🇳  511 | ⭕️  178956288 | 🕗 121.8117523193 |
| 🇳  512 | ⭕️  180006912 | 🕗 121.9041137695 |
| 🇳  513 | ⭕️  181061640 | 🕗 121.2979202271 |
| 🇳  514 | ⭕️  182120480 | 🕗 126.4198760986 |
| 🇳  515 | ⭕️  183183440 | 🕗 126.4720382690 |
| 🇳  516 | ⭕️  184250528 | 🕗 124.2561721802 |
| 🇳  517 | ⭕️  185321752 | 🕗 127.2892761230 |
| 🇳  518 | ⭕️  186397120 | 🕗 129.3020782471 |
| 🇳  519 | ⭕️  187476640 | 🕗 127.0842208862 |
| 🇳  520 | ⭕️  188560320 | 🕗 126.7461013794 |
| 🇳  521 | ⭕️  189648168 | 🕗 127.0287170410 |
| 🇳  522 | ⭕️  190740192 | 🕗 129.0426940918 |
| 🇳  523 | ⭕️  191836400 | 🕗 132.0360107422 |
| 🇳  524 | ⭕️  192936800 | 🕗 131.4754486084 |
| 🇳  525 | ⭕️  194041400 | 🕗 134.3373565674 |
| 🇳  526 | ⭕️  195150208 | 🕗 139.4683837891 |
| 🇳  527 | ⭕️  196263232 | 🕗 132.9397125244 |
| 🇳  528 | ⭕️  197380480 | 🕗 135.4264984131 |
| 🇳  529 | ⭕️  198501960 | 🕗 137.1791076660 |
| 🇳  530 | ⭕️  199627680 | 🕗 137.0079956055 |
| 🇳  531 | ⭕️  200757648 | 🕗 139.4329681396 |
| 🇳  532 | ⭕️  201891872 | 🕗 142.4179687500 |
| 🇳  533 | ⭕️  203030360 | 🕗 139.7931823730 |
| 🇳  534 | ⭕️  204173120 | 🕗 140.5159149170 |
| 🇳  535 | ⭕️  205320160 | 🕗 143.7351379395 |
| 🇳  536 | ⭕️  206471488 | 🕗 144.6264801025 |
| 🇳  537 | ⭕️  207627112 | 🕗 137.9014892578 |
| 🇳  538 | ⭕️  208787040 | 🕗 135.5072174072 |
| 🇳  539 | ⭕️  209951280 | 🕗 140.5518493652 |
| 🇳  540 | ⭕️  211119840 | 🕗 142.6713256836 |
| 🇳  541 | ⭕️  212292728 | 🕗 148.1614837646 |
| 🇳  542 | ⭕️  213469952 | 🕗 146.0531311035 |
| 🇳  543 | ⭕️  214651520 | 🕗 145.3648376465 |
| 🇳  544 | ⭕️  215837440 | 🕗 150.2340393066 |
| 🇳  545 | ⭕️  217027720 | 🕗 148.3772735596 |
| 🇳  546 | ⭕️  218222368 | 🕗 151.3356628418 |
| 🇳  547 | ⭕️  219421392 | 🕗 155.7022094727 |
| 🇳  548 | ⭕️  220624800 | 🕗 159.3164672852 |
| 🇳  549 | ⭕️  221832600 | 🕗 166.2347717285 |
| 🇳  550 | ⭕️  223044800 | 🕗 166.2517242432 |
| 🇳  551 | ⭕️  224261408 | 🕗 168.4310455322 |
| 🇳  552 | ⭕️  225482432 | 🕗 173.3970947266 |
| 🇳  553 | ⭕️  226707880 | 🕗 167.3548583984 |
| 🇳  554 | ⭕️  227937760 | 🕗 170.3714752197 |
| 🇳  555 | ⭕️  229172080 | 🕗 173.6964111328 |
| 🇳  556 | ⭕️  230410848 | 🕗 177.2917022705 |
| 🇳  557 | ⭕️  231654072 | 🕗 168.6299743652 |
| 🇳  558 | ⭕️  232901760 | 🕗 172.6243133545 |
| 🇳  559 | ⭕️  234153920 | 🕗 173.8056640625 |
| 🇳  560 | ⭕️  235410560 | 🕗 177.3089294434 |
| 🇳  561 | ⭕️  236671688 | 🕗 176.2440338135 |
| 🇳  562 | ⭕️  237937312 | 🕗 179.4583435059 |
| 🇳  563 | ⭕️  239207440 | 🕗 185.1515502930 |
| 🇳  564 | ⭕️  240482080 | 🕗 181.7184448242 |
| 🇳  565 | ⭕️  241761240 | 🕗 180.7025451660 |
| 🇳  566 | ⭕️  243044928 | 🕗 184.4105529785 |
| 🇳  567 | ⭕️  244333152 | 🕗 184.4000091553 |
| 🇳  568 | ⭕️  245625920 | 🕗 189.2600402832 |
| 🇳  569 | ⭕️  246923240 | 🕗 185.1887664795 |
| 🇳  570 | ⭕️  248225120 | 🕗 179.78815      |
| 🇳  571 | ⭕️  249531568 | 🕗 180.31573      |
| 🇳  572 | ⭕️  250842592 | 🕗 182.19945      |
| 🇳  573 | ⭕️  252158200 | 🕗 182.62872      |
| 🇳  574 | ⭕️  253478400 | 🕗 184.66798      |
| 🇳  575 | ⭕️  254803200 | 🕗 187.61467      |
| 🇳  576 | ⭕️  256132608 | 🕗 186.98619      |
| 🇳  577 | ⭕️  257466632 | 🕗 184.87135      |
| 🇳  578 | ⭕️  258805280 | 🕗 189.56717      |
| 🇳  579 | ⭕️  260148560 | 🕗 187.35043      |
| 🇳  580 | ⭕️  261496480 | 🕗 192.12587      |
| 🇳  581 | ⭕️  262849048 | 🕗 193.16795      |
| 🇳  582 | ⭕️  264206272 | 🕗 194.6902       |
| 🇳  583 | ⭕️  265568160 | 🕗 197.04663      |
| 🇳  584 | ⭕️  266934720 | 🕗 194.37743      |
| 🇳  585 | ⭕️  268305960 | 🕗 193.86542      |
| 🇳  586 | ⭕️  269681888 | 🕗 195.97664      |
| 🇳  587 | ⭕️  271062512 | 🕗 197.89943      |
| 🇳  588 | ⭕️  272447840 | 🕗 200.29701      |
| 🇳  589 | ⭕️  273837880 | 🕗 202.6525       |
| 🇳  590 | ⭕️  275232640 | 🕗 200.66794      |
| 🇳  591 | ⭕️  276632128 | 🕗 202.40009      |
| 🇳  592 | ⭕️  278036352 | 🕗 203.74187      |
| 🇳  593 | ⭕️  279445320 | 🕗 207.82635      |
| 🇳  594 | ⭕️  280859040 | 🕗 208.36325      |
| 🇳  595 | ⭕️  282277520 | 🕗 210.08517      |
| 🇳  596 | ⭕️  283700768 | 🕗 212.49861      |
| 🇳  597 | ⭕️  285128792 | 🕗 215.53134      |
| 🇳  598 | ⭕️  286561600 | 🕗 216.51059      |
| 🇳  599 | ⭕️  287999200 | 🕗 220.99907      |
| 🇳  600 | ⭕️  289441600 | 🕗 219.10083      |
| 🇳  601 | ⭕️  290888808 | 🕗 217.03249      |
| 🇳  602 | ⭕️  292340832 | 🕗 221.26782      |
| 🇳  603 | ⭕️  293797680 | 🕗 220.79         |
| 🇳  604 | ⭕️  295259360 | 🕗 227.8595       |
| 🇳  605 | ⭕️  296725880 | 🕗 227.5769       |
| 🇳  606 | ⭕️  298197248 | 🕗 227.61673      |
| 🇳  607 | ⭕️  299673472 | 🕗 230.5792       |
| 🇳  608 | ⭕️  301154560 | 🕗 237.21375      |
| 🇳  609 | ⭕️  302640520 | 🕗 228.3568       |
| 🇳  610 | ⭕️  304131360 | 🕗 233.98697      |
| 🇳  611 | ⭕️  305627088 | 🕗 235.43521      |
| 🇳  612 | ⭕️  307127712 | 🕗 234.63312      |
| 🇳  613 | ⭕️  308633240 | 🕗 235.42027      |
| 🇳  614 | ⭕️  310143680 | 🕗 238.36655      |
| 🇳  615 | ⭕️  311659040 | 🕗 236.21582      |
| 🇳  616 | ⭕️  313179328 | 🕗 244.35         |
| 🇳  617 | ⭕️  314704552 | 🕗 245.1835       |
| 🇳  618 | ⭕️  316234720 | 🕗 248.50839      |
| 🇳  619 | ⭕️  317769840 | 🕗 244.38019      |
| 🇳  620 | ⭕️  319309920 | 🕗 249.75476      |
| 🇳  621 | ⭕️  320854968 | 🕗 251.6605       |
| 🇳  622 | ⭕️  322404992 | 🕗 252.70122      |
| 🇳  623 | ⭕️  323960000 | 🕗 255.81602      |
| 🇳  624 | ⭕️  325520000 | 🕗 254.25148      |
| 🇳  625 | ⭕️  327085000 | 🕗 256.72485      |
| 🇳  626 | ⭕️  328655008 | 🕗 259.71835      |
| 🇳  627 | ⭕️  330230032 | 🕗 259.02118      |
| 🇳  628 | ⭕️  331810080 | 🕗 264.54694      |
| 🇳  629 | ⭕️  333395160 | 🕗 263.34445      |
| 🇳  630 | ⭕️  334985280 | 🕗 266.01642      |
| 🇳  631 | ⭕️  336580448 | 🕗 265.517        |
| 🇳  632 | ⭕️  338180672 | 🕗 270.39685      |
| 🇳  633 | ⭕️  339785960 | 🕗 269.1739       |
| 🇳  634 | ⭕️  341396320 | 🕗 270.71725      |
| 🇳  635 | ⭕️  343011760 | 🕗 275.8484       |
| 🇳  636 | ⭕️  344632288 | 🕗 275.3204       |
| 🇳  637 | ⭕️  346257912 | 🕗 276.99954      |
| 🇳  638 | ⭕️  347888640 | 🕗 281.7454       |
| 🇳  639 | ⭕️  349524480 | 🕗 281.16776      |
| 🇳  640 | ⭕️  351165440 | 🕗 282.38452      |
| 🇳  641 | ⭕️  352811528 | 🕗 281.90735      |
| 🇳  642 | ⭕️  354462752 | 🕗 291.44666      |
| 🇳  643 | ⭕️  356119120 | 🕗 283.36206      |
| 🇳  644 | ⭕️  357780640 | 🕗 294.34402      |
| 🇳  645 | ⭕️  359447320 | 🕗 290.5282       |
| 🇳  646 | ⭕️  361119168 | 🕗 292.79742      |
| 🇳  647 | ⭕️  362796192 | 🕗 294.5445       |
| 🇳  648 | ⭕️  364478400 | 🕗 301.7287       |
| 🇳  649 | ⭕️  366165800 | 🕗 298.8732       |
| 🇳  650 | ⭕️  367858400 | 🕗 296.63193      |
| 🇳  651 | ⭕️  369556208 | 🕗 299.59885      |
| 🇳  652 | ⭕️  371259232 | 🕗 304.9361       |
| 🇳  653 | ⭕️  372967480 | 🕗 305.51074      |
| 🇳  654 | ⭕️  374680960 | 🕗 310.15192      |
| 🇳  655 | ⭕️  376399680 | 🕗 306.63828      |
| 🇳  656 | ⭕️  378123648 | 🕗 316.4287       |
| 🇳  657 | ⭕️  379852872 | 🕗 316.29248      |
| 🇳  658 | ⭕️  381587360 | 🕗 318.9253       |
| 🇳  659 | ⭕️  383327120 | 🕗 317.72104      |
| 🇳  660 | ⭕️  385072160 | 🕗 324.91394      |
| 🇳  661 | ⭕️  386822488 | 🕗 320.90378      |
| 🇳  662 | ⭕️  388578112 | 🕗 332.783        |
| 🇳  663 | ⭕️  390339040 | 🕗 326.09805      |
| 🇳  664 | ⭕️  392105280 | 🕗 325.52686      |
| 🇳  665 | ⭕️  393876840 | 🕗 332.3045       |
| 🇳  666 | ⭕️  395653728 | 🕗 332.49396      |
| 🇳  667 | ⭕️  397435952 | 🕗 333.56726      |
| 🇳  668 | ⭕️  399223520 | 🕗 333.41656      |
| 🇳  669 | ⭕️  401016440 | 🕗 335.96536      |
| 🇳  670 | ⭕️  402814720 | 🕗 352.47244      |
| 🇳  671 | ⭕️  404618368 | 🕗 342.31882      |
| 🇳  672 | ⭕️  406427392 | 🕗 349.36813      |
| 🇳  673 | ⭕️  408241800 | 🕗 343.68176      |
| 🇳  674 | ⭕️  410061600 | 🕗 347.03033      |
| 🇳  675 | ⭕️  411886800 | 🕗 343.7539       |
| 🇳  676 | ⭕️  413717408 | 🕗 351.1874       |
| 🇳  677 | ⭕️  415553432 | 🕗 350.2762       |
| 🇳  678 | ⭕️  417394880 | 🕗 352.42383      |
| 🇳  679 | ⭕️  419241760 | 🕗 376.27448      |
| 🇳  680 | ⭕️  421094080 | 🕗 375.88297      |
| 🇳  681 | ⭕️  422951848 | 🕗 376.91907      |
| 🇳  682 | ⭕️  424815072 | 🕗 381.3379       |
| 🇳  683 | ⭕️  426683760 | 🕗 373.8281       |
| 🇳  684 | ⭕️  428557920 | 🕗 382.82214      |
| 🇳  685 | ⭕️  430437560 | 🕗 382.72775      |
| 🇳  686 | ⭕️  432322688 | 🕗 374.317        |
| 🇳  687 | ⭕️  434213312 | 🕗 384.2565       |
| 🇳  688 | ⭕️  436109440 | 🕗 381.04147      |
| 🇳  689 | ⭕️  438011080 | 🕗 388.53992      |
| 🇳  690 | ⭕️  439918240 | 🕗 389.09537      |
| 🇳  691 | ⭕️  441830928 | 🕗 389.1716       |
| 🇳  692 | ⭕️  443749152 | 🕗 400.79697      |
| 🇳  693 | ⭕️  445672920 | 🕗 405.38696      |
| 🇳  694 | ⭕️  447602240 | 🕗 408.5462       |
| 🇳  695 | ⭕️  449537120 | 🕗 404.8601       |
| 🇳  696 | ⭕️  451477568 | 🕗 410.6329       |
| 🇳  697 | ⭕️  453423592 | 🕗 414.41913      |
| 🇳  698 | ⭕️  455375200 | 🕗 419.08527      |
| 🇳  699 | ⭕️  457332400 | 🕗 415.31454      |
| 🇳  700 | ⭕️  459295200 | 🕗 422.31543      |
| 🇳  701 | ⭕️  461263608 | 🕗 409.09766      |
| 🇳  702 | ⭕️  463237632 | 🕗 420.29843      |
| 🇳  703 | ⭕️  465217280 | 🕗 426.2619       |
| 🇳  704 | ⭕️  467202560 | 🕗 439.5127       |
| 🇳  705 | ⭕️  469193480 | 🕗 442.2176       |
| 🇳  706 | ⭕️  471190048 | 🕗 448.0588       |
| 🇳  707 | ⭕️  473192272 | 🕗 445.55316      |
| 🇳  708 | ⭕️  475200160 | 🕗 440.87         |
| 🇳  709 | ⭕️  477213720 | 🕗 440.41907      |
| 🇳  710 | ⭕️  479232960 | 🕗 443.05048      |
| 🇳  711 | ⭕️  481257888 | 🕗 455.3008       |
| 🇳  712 | ⭕️  483288512 | 🕗 442.98337      |
| 🇳  713 | ⭕️  485324840 | 🕗 452.03366      |
| 🇳  714 | ⭕️  487366880 | 🕗 448.5766       |
| 🇳  715 | ⭕️  489414640 | 🕗 447.6833       |
| 🇳  716 | ⭕️  491468128 | 🕗 460.47015      |
| 🇳  717 | ⭕️  493527352 | 🕗 454.91162      |
| 🇳  718 | ⭕️  495592320 | 🕗 468.20422      |
| 🇳  719 | ⭕️  497663040 | 🕗 467.21396      |
| 🇳  720 | ⭕️  499739520 | 🕗 463.67163      |
| 🇳  721 | ⭕️  501821768 | 🕗 469.57587      |
| 🇳  722 | ⭕️  503909792 | 🕗 474.38907      |
| 🇳  723 | ⭕️  506003600 | 🕗 482.1608       |
| 🇳  724 | ⭕️  508103200 | 🕗 478.16135      |
| 🇳  725 | ⭕️  510208600 | 🕗 477.60892      |
| 🇳  726 | ⭕️  512319808 | 🕗 487.3817       |
| 🇳  727 | ⭕️  514436832 | 🕗 494.51654      |
| 🇳  728 | ⭕️  516559680 | 🕗 487.7568       |
| 🇳  729 | ⭕️  518688360 | 🕗 485.13315      |
| 🇳  730 | ⭕️  520822880 | 🕗 498.55057      |
| 🇳  731 | ⭕️  522963248 | 🕗 498.00424      |
| 🇳  732 | ⭕️  525109472 | 🕗 502.94525      |
| 🇳  733 | ⭕️  527261560 | 🕗 502.40942      |
| 🇳  734 | ⭕️  529419520 | 🕗 511.55048      |
| 🇳  735 | ⭕️  531583360 | 🕗 512.0618       |
| 🇳  736 | ⭕️  533753088 | 🕗 519.5763       |
| 🇳  737 | ⭕️  535928712 | 🕗 513.1648       |
| 🇳  738 | ⭕️  538110240 | 🕗 522.27026      |
| 🇳  739 | ⭕️  540297680 | 🕗 525.7643       |
| 🇳  740 | ⭕️  542491040 | 🕗 522.33984      |
| 🇳  741 | ⭕️  544690328 | 🕗 530.2288       |
| 🇳  742 | ⭕️  546895552 | 🕗 534.44507      |
| 🇳  743 | ⭕️  549106720 | 🕗 537.5375       |
| 🇳  744 | ⭕️  551323840 | 🕗 542.68414      |
| 🇳  745 | ⭕️  553546920 | 🕗 549.7611       |
| 🇳  746 | ⭕️  555775968 | 🕗 559.8681       |
| 🇳  747 | ⭕️  558010992 | 🕗 557.90625      |
| 🇳  748 | ⭕️  560252000 | 🕗 563.9526       |
| 🇳  749 | ⭕️  562499000 | 🕗 567.5977       |
| 🇳  750 | ⭕️  564752000 | 🕗 570.25946      |
| 🇳  751 | ⭕️  567011008 | 🕗 580.4319       |
| 🇳  752 | ⭕️  569276032 | 🕗 564.4526       |
| 🇳  753 | ⭕️  571547080 | 🕗 584.68445      |
| 🇳  754 | ⭕️  573824160 | 🕗 580.8851       |
| 🇳  755 | ⭕️  576107280 | 🕗 582.40955      |
| 🇳  756 | ⭕️  578396448 | 🕗 593.1553       |
| 🇳  757 | ⭕️  580691672 | 🕗 596.1205       |
| 🇳  758 | ⭕️  582992960 | 🕗 592.6622       |
| 🇳  759 | ⭕️  585300320 | 🕗 607.0393       |
| 🇳  760 | ⭕️  587613760 | 🕗 609.11786      |
| 🇳  761 | ⭕️  589933288 | 🕗 613.04626      |
| 🇳  762 | ⭕️  592258912 | 🕗 621.4857       |
| 🇳  763 | ⭕️  594590640 | 🕗 615.5292       |
| 🇳  764 | ⭕️  596928480 | 🕗 626.5498       |
| 🇳  765 | ⭕️  599272440 | 🕗 627.74664      |
| 🇳  766 | ⭕️  601622528 | 🕗 635.35016      |
| 🇳  767 | ⭕️  603978752 | 🕗 632.1883       |
| 🇳  768 | ⭕️  606341120 | 🕗 638.151        |
| 🇳  769 | ⭕️  608709640 | 🕗 639.95465      |
| 🇳  770 | ⭕️  611084320 | 🕗 639.476        |
| 🇳  771 | ⭕️  613465168 | 🕗 640.6278       |
| 🇳  772 | ⭕️  615852192 | 🕗 654.24304      |
| 🇳  773 | ⭕️  618245400 | 🕗 653.71533      |
| 🇳  774 | ⭕️  620644800 | 🕗 672.1014       |
| 🇳  775 | ⭕️  623050400 | 🕗 665.65643      |
| 🇳  776 | ⭕️  625462208 | 🕗 669.41693      |
| 🇳  777 | ⭕️  627880232 | 🕗 676.18365      |
| 🇳  778 | ⭕️  630304480 | 🕗 677.1852       |
| 🇳  779 | ⭕️  632734960 | 🕗 691.56335      |
| 🇳  780 | ⭕️  635171680 | 🕗 697.5961       |
| 🇳  781 | ⭕️  637614648 | 🕗 694.1226       |
| 🇳  782 | ⭕️  640063872 | 🕗 698.0464       |
| 🇳  783 | ⭕️  642519360 | 🕗 692.53595      |
| 🇳  784 | ⭕️  644981120 | 🕗 713.0903       |
| 🇳  785 | ⭕️  647449160 | 🕗 707.44183      |
| 🇳  786 | ⭕️  649923488 | 🕗 711.2089       |
| 🇳  787 | ⭕️  652404112 | 🕗 707.148        |
| 🇳  788 | ⭕️  654891040 | 🕗 716.5655       |
| 🇳  789 | ⭕️  657384280 | 🕗 717.7836       |
| 🇳  790 | ⭕️  659883840 | 🕗 727.2548       |
| 🇳  791 | ⭕️  662389728 | 🕗 711.4368       |
| 🇳  792 | ⭕️  664901952 | 🕗 749.35803      |
| 🇳  793 | ⭕️  667420520 | 🕗 740.723        |
| 🇳  794 | ⭕️  669945440 | 🕗 739.453        |
| 🇳  795 | ⭕️  672476720 | 🕗 739.6702       |
| 🇳  796 | ⭕️  675014368 | 🕗 754.49194      |
| 🇳  797 | ⭕️  677558392 | 🕗 757.54785      |
| 🇳  798 | ⭕️  680108800 | 🕗 753.9655       |
| 🇳  799 | ⭕️  682665600 | 🕗 767.94226      |
| 🇳  800 | ⭕️  685228800 | 🕗 776.0574       |
| 🇳  801 | ⭕️  687798408 | 🕗 767.04785      |
| 🇳  802 | ⭕️  690374432 | 🕗 795.5465       |
| 🇳  803 | ⭕️  692956880 | 🕗 776.9618       |
| 🇳  804 | ⭕️  695545760 | 🕗 800.3405       |
| 🇳  805 | ⭕️  698141080 | 🕗 795.2684       |
| 🇳  806 | ⭕️  700742848 | 🕗 806.8855       |
| 🇳  807 | ⭕️  703351072 | 🕗 797.0185       |
| 🇳  808 | ⭕️  705965760 | 🕗 815.8328       |
| 🇳  809 | ⭕️  708586920 | 🕗 821.943        |
| 🇳  810 | ⭕️  711214560 | 🕗 817.94806      |
| 🇳  811 | ⭕️  713848688 | 🕗 831.2155       |
| 🇳  812 | ⭕️  716489312 | 🕗 832.433        |
| 🇳  813 | ⭕️  719136440 | 🕗 836.33704      |
| 🇳  814 | ⭕️  721790080 | 🕗 837.7355       |
| 🇳  815 | ⭕️  724450240 | 🕗 854.2903       |
| 🇳  816 | ⭕️  727116928 | 🕗 832.3699       |
| 🇳  817 | ⭕️  729790152 | 🕗 862.3605       |
| 🇳  818 | ⭕️  732469920 | 🕗 864.9721       |
| 🇳  819 | ⭕️  735156240 | 🕗 887.0709       |
| 🇳  820 | ⭕️  737849120 | 🕗 896.02313      |
| 🇳  821 | ⭕️  740548568 | 🕗 903.262        |
| 🇳  822 | ⭕️  743254592 | 🕗 918.7063       |
| 🇳  823 | ⭕️  745967200 | 🕗 905.7513       |
| 🇳  824 | ⭕️  748686400 | 🕗 878.9788       |
| 🇳  825 | ⭕️  751412200 | 🕗 901.5448       |
| 🇳  826 | ⭕️  754144608 | 🕗 916.91876      |
| 🇳  827 | ⭕️  756883632 | 🕗 932.1164       |
| 🇳  828 | ⭕️  759629280 | 🕗 933.8062       |
| 🇳  829 | ⭕️  762381560 | 🕗 927.74115      |
| 🇳  830 | ⭕️  765140480 | 🕗 947.50653      |
| 🇳  831 | ⭕️  767906048 | 🕗 959.41345      |
| 🇳  832 | ⭕️  770678272 | 🕗 933.34033      |
| 🇳  833 | ⭕️  773457160 | 🕗 956.98254      |
| 🇳  834 | ⭕️  776242720 | 🕗 949.05835      |
| 🇳  835 | ⭕️  779034960 | 🕗 1100.8534      |
| 🇳  835 | ⭕️  779034960 | 🕗1326.0327       |
| 🇳  836 | ⭕️  781833888 | 🕗1326.1456       |
| 🇳  837 | ⭕️  784639512 | 🕗1316.1548       |
| 🇳  838 | ⭕️  787451840 | 🕗1309.3453       |
| 🇳  839 | ⭕️  790270880 | 🕗1335.1016       |
| 🇳  840 | ⭕️  793096640 | 🕗1324.4258       |
| 🇳  841 | ⭕️  795929128 | 🕗 996.884        |
| 🇳  842 | ⭕️  798768352 | 🕗 1009.86005     |
| 🇳  843 | ⭕️  801614320 | 🕗 1008.44684     |
| 🇳  844 | ⭕️  804467040 | 🕗 1023.4822      |
| 🇳  845 | ⭕️  807326520 | 🕗 1036.7554      |
| 🇳  846 | ⭕️  810192768 | 🕗 1048.3129      |
| 🇳  847 | ⭕️  813065792 | 🕗 1039.1687      |
| 🇳  848 | ⭕️  815945600 | 🕗 1032.5049      |
| 🇳  849 | ⭕️  818832200 | 🕗 1034.432       |
| 🇳  850 | ⭕️  821725600 | 🕗 1070.8883      |
| 🇳  851 | ⭕️  824625808 | 🕗 1044.2975      |
| 🇳  852 | ⭕️  827532832 | 🕗 1081.0212      |
| 🇳  853 | ⭕️  830446680 | 🕗 1062.7942      |
| 🇳  854 | ⭕️  833367360 | 🕗 1081.022       |
| 🇳  855 | ⭕️  836294880 | 🕗 1103.6678      |
| 🇳  856 | ⭕️  839229248 | 🕗 1094.1357      |
| 🇳  857 | ⭕️  842170472 | 🕗 1088.5359      |
| 🇳  858 | ⭕️  845118560 | 🕗 1102.7394      |
| 🇳  859 | ⭕️  848073520 | 🕗 1086.4933      |
| 🇳  860 | ⭕️  851035360 | 🕗 1098.6343      |
| 🇳  861 | ⭕️  854004088 | 🕗 1099.377       |
| 🇳  862 | ⭕️  856979712 | 🕗 1126.6124      |
| 🇳  863 | ⭕️  859962240 | 🕗 1141.824       |
| 🇳  864 | ⭕️  862951680 | 🕗 1125.0447      |
| 🇳  865 | ⭕️  865948040 | 🕗 1123.4713      |
| 🇳  866 | ⭕️  868951328 | 🕗 1124.2178      |
| 🇳  867 | ⭕️  871961552 | 🕗 1150.2515      |
| 🇳  868 | ⭕️  874978720 | 🕗 1186.849       |
| 🇳  869 | ⭕️  878002840 | 🕗 1215.1089      |
| 🇳  870 | ⭕️  881033920 | 🕗 1184.2728      |
| 🇳  871 | ⭕️  884071968 | 🕗 1176.9257      |
| 🇳  872 | ⭕️  887116992 | 🕗 1191.0714      |
| 🇳  873 | ⭕️  890169000 | 🕗 1151.8418      |
| 🇳  874 | ⭕️  893228000 | 🕗 1219.4382      |
| 🇳  875 | ⭕️  896294000 | 🕗 1230.8821      |
| 🇳  876 | ⭕️  899367008 | 🕗 1198.1079      |
| 🇳  877 | ⭕️  902447032 | 🕗 1204.568       |
| 🇳  878 | ⭕️  905534080 | 🕗 1226.0151      |
| 🇳  879 | ⭕️  908628160 | 🕗 1235.4539      |
| 🇳  880 | ⭕️  911729280 | 🕗 1247.8569      |
| 🇳  881 | ⭕️  914837448 | 🕗 1249.0696      |
| 🇳  882 | ⭕️  917952672 | 🕗 1253.1187      |
| 🇳  883 | ⭕️  921074960 | 🕗 1241.7137      |
| 🇳  884 | ⭕️  924204320 | 🕗 1281.2852      |
| 🇳  885 | ⭕️  927340760 | 🕗 1242.4163      |
| 🇳  886 | ⭕️  930484288 | 🕗 1336.2025      |
| 🇳  887 | ⭕️  933634912 | 🕗 1272.7885      |
| 🇳  888 | ⭕️  936792640 | 🕗 1312.6239      |
| 🇳  889 | ⭕️  939957480 | 🕗 1268.2065      |
| 🇳  890 | ⭕️  943129440 | 🕗 1286.0504      |
| 🇳  891 | ⭕️  946308528 | 🕗 1374.1068      |
| 🇳  892 | ⭕️  949494752 | 🕗 1384.2745      |
| 🇳  893 | ⭕️  952688120 | 🕗 1309.6842      |
| 🇳  894 | ⭕️  955888640 | 🕗 1349.9381      |
| 🇳  895 | ⭕️  959096320 | 🕗 1364.9525      |
| 🇳  896 | ⭕️  962311168 | 🕗 1400.4164      |
| 🇳  897 | ⭕️  965533192 | 🕗 1384.9238      |
| 🇳  898 | ⭕️  968762400 | 🕗 1387.0442      |
| 🇳  899 | ⭕️  971998800 | 🕗 1431.7327      |
| 🇳  900 | ⭕️  975242400 | 🕗 1134.092       | 

| 1-8 BILLION: 
| 🇳  950 | ⭕️ 1146779200 | 🕗 1416.025       | 
| 🇳 1000 | ⭕️ 1337336000 | 🕗 1779.502       | 
| 🇳 1010 | ⭕️ 1377817760 | 🕗 2561.4006      |  
| 🇳 1100 | ⭕️ 1779509600 | 🕗 3562.2673      |  
| 🇳 1200 | ⭕️ 2309763200 | 🕗 8420.2519      | 
| 🇳 1300 | ⭕️ 2936096800 | 🕗 11424.484      | 
| 🇳 1310 | ⭕️ 3004322560 | 🕗 11881.937      | 
| 🇳 1320 | ⭕️ 3073597120 | 🕗 12736.059      | 
| 🇳 1330 | ⭕️ 3143928480 | 🕗 12246.044      | 
| 🇳 1350 | ⭕️ 3287793600 | 🕗 5761.2993      | 
| 🇳 1400 | ⭕️ 3666510400 | 🕗 6919.091       | 
| 🇳 1450 | ⭕️ 4073247200 | 🕗 7296.6455      | 
| 🇳 1500 | ⭕️ 4509004000 | 🕗 8243.446       | 
| 🇳 1550 | ⭕️ 4974780800 | 🕗 10231.715      | 
| 🇳 1600 | ⭕️ 5471577600 | 🕗 11542.147      |
| 🇳 1650 | ⭕️ 6000394400 | 🕗 13756.121      |
| 🇳 1700 | ⭕️ 6562231200 | 🕗 15356.612      |
| 🇳 1817 | ⭕️ 8011618152 | 🕗 19884.557      | 

```
┌──────────────────────────────┐
│ ESTIMATED SIZE OF SOLUTION   │
|------------------------------|
| ORDER (Billions) | SIZE (GB) |
|------------------|-----------|
| 1                | 5.88      |
| 1.5              | 8.82      |
| 2                | 11.76     |
| 2.5              | 14.70     |
| 3                | 17.64     |
| 3.5              | 20.58     |
| 4                | 23.53     | Usage is about twice.
| 4.5              | 26.47     |
| 5                | 29.41     |
| 5.5              | 32.35     |
| 6                | 35.29     |
| 6.5              | 38.23     |
| 7                | 41.18     |
| 7.5              | 44.12     |
| 8                | 47.06     |
| 8.5              | 50.00     | <------------ GOAL OF 8 BILLION
| 9                | 52.94     |
| 9.5              | 55.88     |
| 10               | 58.82     |
└──────────────────────────────┘
## Licensing:

This package is licensed under the MIT license.
 
Thanks for making it this far!


![Very first discocube in Berghain](imgs/ako.png)
*Me and Discocube in Berghain*
