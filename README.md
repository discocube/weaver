
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

#### Running times for graphs with 8 to over 8 billion vertices:
```
| 🇳    1 | ⭕️          8 | 🕗 SOLVE: 0.0000032500 | 📌 HamCycle | 🕗 CERTIFY: 0.0000030000
| 🇳    2 | ⭕️         32 | 🕗 SOLVE: 0.0000052080 | 📌 HamCycle | 🕗 CERTIFY: 0.0000074170
| 🇳    3 | ⭕️         80 | 🕗 SOLVE: 0.0000235830 | 📌 HamCycle | 🕗 CERTIFY: 0.0000058750
| 🇳    4 | ⭕️        160 | 🕗 SOLVE: 0.0000172500 | 📌 HamCycle | 🕗 CERTIFY: 0.0000096250
| 🇳    5 | ⭕️        280 | 🕗 SOLVE: 0.0000320420 | 📌 HamCycle | 🕗 CERTIFY: 0.0000227920
| 🇳    6 | ⭕️        448 | 🕗 SOLVE: 0.0000341660 | 📌 HamCycle | 🕗 CERTIFY: 0.0000205000
| 🇳    7 | ⭕️        672 | 🕗 SOLVE: 0.0000497920 | 📌 HamCycle | 🕗 CERTIFY: 0.0000278340
| 🇳    8 | ⭕️        960 | 🕗 SOLVE: 0.0000617500 | 📌 HamCycle | 🕗 CERTIFY: 0.0000587090
| 🇳    9 | ⭕️       1320 | 🕗 SOLVE: 0.0000841250 | 📌 HamCycle | 🕗 CERTIFY: 0.0000720420
| 🇳   10 | ⭕️       1760 | 🕗 SOLVE: 0.0001036250 | 📌 HamCycle | 🕗 CERTIFY: 0.0000737080
| 🇳   11 | ⭕️       2288 | 🕗 SOLVE: 0.0001224170 | 📌 HamCycle | 🕗 CERTIFY: 0.0001104580
| 🇳   12 | ⭕️       2912 | 🕗 SOLVE: 0.0001251660 | 📌 HamCycle | 🕗 CERTIFY: 0.0001145410
| 🇳   13 | ⭕️       3640 | 🕗 SOLVE: 0.0001680000 | 📌 HamCycle | 🕗 CERTIFY: 0.0001706250
| 🇳   14 | ⭕️       4480 | 🕗 SOLVE: 0.0001842920 | 📌 HamCycle | 🕗 CERTIFY: 0.0002605000
| 🇳   15 | ⭕️       5440 | 🕗 SOLVE: 0.0002058340 | 📌 HamCycle | 🕗 CERTIFY: 0.0002512920
| 🇳   16 | ⭕️       6528 | 🕗 SOLVE: 0.0002532920 | 📌 HamCycle | 🕗 CERTIFY: 0.0002444580
| 🇳   17 | ⭕️       7752 | 🕗 SOLVE: 0.0002825840 | 📌 HamCycle | 🕗 CERTIFY: 0.0003543750
| 🇳   18 | ⭕️       9120 | 🕗 SOLVE: 0.0003360420 | 📌 HamCycle | 🕗 CERTIFY: 0.0003684170
| 🇳   19 | ⭕️      10640 | 🕗 SOLVE: 0.0003747910 | 📌 HamCycle | 🕗 CERTIFY: 0.0003840420
| 🇳   20 | ⭕️      12320 | 🕗 SOLVE: 0.0004447500 | 📌 HamCycle | 🕗 CERTIFY: 0.0004862080
| 🇳   21 | ⭕️      14168 | 🕗 SOLVE: 0.0005017920 | 📌 HamCycle | 🕗 CERTIFY: 0.0005682500
| 🇳   22 | ⭕️      16192 | 🕗 SOLVE: 0.0005813340 | 📌 HamCycle | 🕗 CERTIFY: 0.0007138330
| 🇳   23 | ⭕️      18400 | 🕗 SOLVE: 0.0006646660 | 📌 HamCycle | 🕗 CERTIFY: 0.0007405840
| 🇳   24 | ⭕️      20800 | 🕗 SOLVE: 0.0007150830 | 📌 HamCycle | 🕗 CERTIFY: 0.0008022500
| 🇳   25 | ⭕️      23400 | 🕗 SOLVE: 0.0008433750 | 📌 HamCycle | 🕗 CERTIFY: 0.0007905000
| 🇳   26 | ⭕️      26208 | 🕗 SOLVE: 0.0009349160 | 📌 HamCycle | 🕗 CERTIFY: 0.0009167500
| 🇳   27 | ⭕️      29232 | 🕗 SOLVE: 0.0009802910 | 📌 HamCycle | 🕗 CERTIFY: 0.0013890000
| 🇳   28 | ⭕️      32480 | 🕗 SOLVE: 0.0011420000 | 📌 HamCycle | 🕗 CERTIFY: 0.0014355830
| 🇳   29 | ⭕️      35960 | 🕗 SOLVE: 0.0012345830 | 📌 HamCycle | 🕗 CERTIFY: 0.0014287090
| 🇳   30 | ⭕️      39680 | 🕗 SOLVE: 0.0013672910 | 📌 HamCycle | 🕗 CERTIFY: 0.0015529169
| 🇳   31 | ⭕️      43648 | 🕗 SOLVE: 0.0014664161 | 📌 HamCycle | 🕗 CERTIFY: 0.0015741671
| 🇳   32 | ⭕️      47872 | 🕗 SOLVE: 0.0016479160 | 📌 HamCycle | 🕗 CERTIFY: 0.0016750830
| 🇳   33 | ⭕️      52360 | 🕗 SOLVE: 0.0018100420 | 📌 HamCycle | 🕗 CERTIFY: 0.0017924170
| 🇳   34 | ⭕️      57120 | 🕗 SOLVE: 0.0019897500 | 📌 HamCycle | 🕗 CERTIFY: 0.0021717080
| 🇳   35 | ⭕️      62160 | 🕗 SOLVE: 0.0022630829 | 📌 HamCycle | 🕗 CERTIFY: 0.0028140000
| 🇳   36 | ⭕️      67488 | 🕗 SOLVE: 0.0024215421 | 📌 HamCycle | 🕗 CERTIFY: 0.0029153749
| 🇳   37 | ⭕️      73112 | 🕗 SOLVE: 0.0025602500 | 📌 HamCycle | 🕗 CERTIFY: 0.0029888330
| 🇳   38 | ⭕️      79040 | 🕗 SOLVE: 0.0028506671 | 📌 HamCycle | 🕗 CERTIFY: 0.0029771670
| 🇳   39 | ⭕️      85280 | 🕗 SOLVE: 0.0031580001 | 📌 HamCycle | 🕗 CERTIFY: 0.0031922499
| 🇳   40 | ⭕️      91840 | 🕗 SOLVE: 0.0033443330 | 📌 HamCycle | 🕗 CERTIFY: 0.0032883750
| 🇳   41 | ⭕️      98728 | 🕗 SOLVE: 0.0036359581 | 📌 HamCycle | 🕗 CERTIFY: 0.0034247921
| 🇳   42 | ⭕️     105952 | 🕗 SOLVE: 0.0040548341 | 📌 HamCycle | 🕗 CERTIFY: 0.0038860410
| 🇳   43 | ⭕️     113520 | 🕗 SOLVE: 0.0042377082 | 📌 HamCycle | 🕗 CERTIFY: 0.0040679998
| 🇳   44 | ⭕️     121440 | 🕗 SOLVE: 0.0046635000 | 📌 HamCycle | 🕗 CERTIFY: 0.0055944999
| 🇳   45 | ⭕️     129720 | 🕗 SOLVE: 0.0048082909 | 📌 HamCycle | 🕗 CERTIFY: 0.0057002092
| 🇳   46 | ⭕️     138368 | 🕗 SOLVE: 0.0054362500 | 📌 HamCycle | 🕗 CERTIFY: 0.0061487500
| 🇳   47 | ⭕️     147392 | 🕗 SOLVE: 0.0055478751 | 📌 HamCycle | 🕗 CERTIFY: 0.0059315409
| 🇳   48 | ⭕️     156800 | 🕗 SOLVE: 0.0058117500 | 📌 HamCycle | 🕗 CERTIFY: 0.0062373751
| 🇳   49 | ⭕️     166600 | 🕗 SOLVE: 0.0064380420 | 📌 HamCycle | 🕗 CERTIFY: 0.0062152082
| 🇳   50 | ⭕️     176800 | 🕗 SOLVE: 0.0070682908 | 📌 HamCycle | 🕗 CERTIFY: 0.0065730000
| 🇳   51 | ⭕️     187408 | 🕗 SOLVE: 0.0074742502 | 📌 HamCycle | 🕗 CERTIFY: 0.0067877499
| 🇳   52 | ⭕️     198432 | 🕗 SOLVE: 0.0079222918 | 📌 HamCycle | 🕗 CERTIFY: 0.0070886249
| 🇳   53 | ⭕️     209880 | 🕗 SOLVE: 0.0084123332 | 📌 HamCycle | 🕗 CERTIFY: 0.0073621250
| 🇳   54 | ⭕️     221760 | 🕗 SOLVE: 0.0090563335 | 📌 HamCycle | 🕗 CERTIFY: 0.0076199998
| 🇳   55 | ⭕️     234080 | 🕗 SOLVE: 0.0097064581 | 📌 HamCycle | 🕗 CERTIFY: 0.0111032920
| 🇳   56 | ⭕️     246848 | 🕗 SOLVE: 0.0105915423 | 📌 HamCycle | 🕗 CERTIFY: 0.0109774591
| 🇳   57 | ⭕️     260072 | 🕗 SOLVE: 0.0112127503 | 📌 HamCycle | 🕗 CERTIFY: 0.0113850003
| 🇳   58 | ⭕️     273760 | 🕗 SOLVE: 0.0115482500 | 📌 HamCycle | 🕗 CERTIFY: 0.0117070414
| 🇳   59 | ⭕️     287920 | 🕗 SOLVE: 0.0119887497 | 📌 HamCycle | 🕗 CERTIFY: 0.0118852081
| 🇳   60 | ⭕️     302560 | 🕗 SOLVE: 0.0132045830 | 📌 HamCycle | 🕗 CERTIFY: 0.0121636661
| 🇳   61 | ⭕️     317688 | 🕗 SOLVE: 0.0144898752 | 📌 HamCycle | 🕗 CERTIFY: 0.0126226246
| 🇳   62 | ⭕️     333312 | 🕗 SOLVE: 0.0154511249 | 📌 HamCycle | 🕗 CERTIFY: 0.0125555405
| 🇳   63 | ⭕️     349440 | 🕗 SOLVE: 0.0158527922 | 📌 HamCycle | 🕗 CERTIFY: 0.0131693333
| 🇳   64 | ⭕️     366080 | 🕗 SOLVE: 0.0169200841 | 📌 HamCycle | 🕗 CERTIFY: 0.0133274589
| 🇳   65 | ⭕️     383240 | 🕗 SOLVE: 0.0175776239 | 📌 HamCycle | 🕗 CERTIFY: 0.0139895836
| 🇳   66 | ⭕️     400928 | 🕗 SOLVE: 0.0192298759 | 📌 HamCycle | 🕗 CERTIFY: 0.0141697917
| 🇳   67 | ⭕️     419152 | 🕗 SOLVE: 0.0187792089 | 📌 HamCycle | 🕗 CERTIFY: 0.0149302082
| 🇳   68 | ⭕️     437920 | 🕗 SOLVE: 0.0208730847 | 📌 HamCycle | 🕗 CERTIFY: 0.0154599585
| 🇳   69 | ⭕️     457240 | 🕗 SOLVE: 0.0220509581 | 📌 HamCycle | 🕗 CERTIFY: 0.0163380839
| 🇳   70 | ⭕️     477120 | 🕗 SOLVE: 0.0227906685 | 📌 HamCycle | 🕗 CERTIFY: 0.0229610838
| 🇳   71 | ⭕️     497568 | 🕗 SOLVE: 0.0243494157 | 📌 HamCycle | 🕗 CERTIFY: 0.0227371231
| 🇳   72 | ⭕️     518592 | 🕗 SOLVE: 0.0265869591 | 📌 HamCycle | 🕗 CERTIFY: 0.0230245423
| 🇳   73 | ⭕️     540200 | 🕗 SOLVE: 0.0274206679 | 📌 HamCycle | 🕗 CERTIFY: 0.0236101244
| 🇳   74 | ⭕️     562400 | 🕗 SOLVE: 0.0285371244 | 📌 HamCycle | 🕗 CERTIFY: 0.0239614155
| 🇳   75 | ⭕️     585200 | 🕗 SOLVE: 0.0295322072 | 📌 HamCycle | 🕗 CERTIFY: 0.0257434584
| 🇳   76 | ⭕️     608608 | 🕗 SOLVE: 0.0328147076 | 📌 HamCycle | 🕗 CERTIFY: 0.0253448766
| 🇳   77 | ⭕️     632632 | 🕗 SOLVE: 0.0326855853 | 📌 HamCycle | 🕗 CERTIFY: 0.0274521243
| 🇳   78 | ⭕️     657280 | 🕗 SOLVE: 0.0386042483 | 📌 HamCycle | 🕗 CERTIFY: 0.0277401675
| 🇳   79 | ⭕️     682560 | 🕗 SOLVE: 0.0377084576 | 📌 HamCycle | 🕗 CERTIFY: 0.0266082082
| 🇳   80 | ⭕️     708480 | 🕗 SOLVE: 0.0382352918 | 📌 HamCycle | 🕗 CERTIFY: 0.0269487072
| 🇳   81 | ⭕️     735048 | 🕗 SOLVE: 0.0398047082 | 📌 HamCycle | 🕗 CERTIFY: 0.0381569155
| 🇳   82 | ⭕️     762272 | 🕗 SOLVE: 0.0452369601 | 📌 HamCycle | 🕗 CERTIFY: 0.0323198326
| 🇳   83 | ⭕️     790160 | 🕗 SOLVE: 0.0457271673 | 📌 HamCycle | 🕗 CERTIFY: 0.0292230416
| 🇳   84 | ⭕️     818720 | 🕗 SOLVE: 0.0463402085 | 📌 HamCycle | 🕗 CERTIFY: 0.0305257086
| 🇳   85 | ⭕️     847960 | 🕗 SOLVE: 0.0482897088 | 📌 HamCycle | 🕗 CERTIFY: 0.0311278757
| 🇳   86 | ⭕️     877888 | 🕗 SOLVE: 0.0513685383 | 📌 HamCycle | 🕗 CERTIFY: 0.0325507075
| 🇳   87 | ⭕️     908512 | 🕗 SOLVE: 0.0544733778 | 📌 HamCycle | 🕗 CERTIFY: 0.0340006687
| 🇳   88 | ⭕️     939840 | 🕗 SOLVE: 0.0571270846 | 📌 HamCycle | 🕗 CERTIFY: 0.0481280833
| 🇳   89 | ⭕️     971880 | 🕗 SOLVE: 0.0565938316 | 📌 HamCycle | 🕗 CERTIFY: 0.0463047065
| 🇳   90 | ⭕️    1004640 | 🕗 SOLVE: 0.0606471673 | 📌 HamCycle | 🕗 CERTIFY: 0.0480904542
| 🇳   91 | ⭕️    1038128 | 🕗 SOLVE: 0.0628498793 | 📌 HamCycle | 🕗 CERTIFY: 0.0487485006
| 🇳   92 | ⭕️    1072352 | 🕗 SOLVE: 0.0671452060 | 📌 HamCycle | 🕗 CERTIFY: 0.0532167070
| 🇳   93 | ⭕️    1107320 | 🕗 SOLVE: 0.0691804588 | 📌 HamCycle | 🕗 CERTIFY: 0.0510776676
| 🇳   94 | ⭕️    1143040 | 🕗 SOLVE: 0.0729944557 | 📌 HamCycle | 🕗 CERTIFY: 0.0621716231
| 🇳   95 | ⭕️    1179520 | 🕗 SOLVE: 0.0745130777 | 📌 HamCycle | 🕗 CERTIFY: 0.0515405424
| 🇳   96 | ⭕️    1216768 | 🕗 SOLVE: 0.0792695805 | 📌 HamCycle | 🕗 CERTIFY: 0.0538300835
| 🇳   97 | ⭕️    1254792 | 🕗 SOLVE: 0.0825179219 | 📌 HamCycle | 🕗 CERTIFY: 0.0624022931
| 🇳   98 | ⭕️    1293600 | 🕗 SOLVE: 0.0834023729 | 📌 HamCycle | 🕗 CERTIFY: 0.0548647530
| 🇳   99 | ⭕️    1333200 | 🕗 SOLVE: 0.0863782465 | 📌 HamCycle | 🕗 CERTIFY: 0.0559503771
| 🇳  100 | ⭕️    1373600 | 🕗 SOLVE: 0.0907897502 | 📌 HamCycle | 🕗 CERTIFY: 0.0571479984
| 🇳  101 | ⭕️    1414808 | 🕗 SOLVE: 0.0958669633 | 📌 HamCycle | 🕗 CERTIFY: 0.0607737936
| 🇳  102 | ⭕️    1456832 | 🕗 SOLVE: 0.0980693772 | 📌 HamCycle | 🕗 CERTIFY: 0.0588104166
| 🇳  103 | ⭕️    1499680 | 🕗 SOLVE: 0.1009650379 | 📌 HamCycle | 🕗 CERTIFY: 0.0594538338
| 🇳  104 | ⭕️    1543360 | 🕗 SOLVE: 0.1084494591 | 📌 HamCycle | 🕗 CERTIFY: 0.0609899163
| 🇳  105 | ⭕️    1587880 | 🕗 SOLVE: 0.1095474586 | 📌 HamCycle | 🕗 CERTIFY: 0.0619926639
| 🇳  106 | ⭕️    1633248 | 🕗 SOLVE: 0.1138958335 | 📌 HamCycle | 🕗 CERTIFY: 0.0639771223
| 🇳  107 | ⭕️    1679472 | 🕗 SOLVE: 0.1151447147 | 📌 HamCycle | 🕗 CERTIFY: 0.0653836653
| 🇳  108 | ⭕️    1726560 | 🕗 SOLVE: 0.1250615418 | 📌 HamCycle | 🕗 CERTIFY: 0.0670739189
| 🇳  109 | ⭕️    1774520 | 🕗 SOLVE: 0.1277494580 | 📌 HamCycle | 🕗 CERTIFY: 0.0687517524
| 🇳  110 | ⭕️    1823360 | 🕗 SOLVE: 0.1333761215 | 📌 HamCycle | 🕗 CERTIFY: 0.0713072494
| 🇳  111 | ⭕️    1873088 | 🕗 SOLVE: 0.1350522488 | 📌 HamCycle | 🕗 CERTIFY: 0.0993924588
| 🇳  112 | ⭕️    1923712 | 🕗 SOLVE: 0.1431567818 | 📌 HamCycle | 🕗 CERTIFY: 0.1014569998
| 🇳  113 | ⭕️    1975240 | 🕗 SOLVE: 0.1433984190 | 📌 HamCycle | 🕗 CERTIFY: 0.1013584957
| 🇳  114 | ⭕️    2027680 | 🕗 SOLVE: 0.1511938721 | 📌 HamCycle | 🕗 CERTIFY: 0.1025762856
| 🇳  115 | ⭕️    2081040 | 🕗 SOLVE: 0.1526802033 | 📌 HamCycle | 🕗 CERTIFY: 0.1051908731
| 🇳  116 | ⭕️    2135328 | 🕗 SOLVE: 0.1631644219 | 📌 HamCycle | 🕗 CERTIFY: 0.1058824956
| 🇳  117 | ⭕️    2190552 | 🕗 SOLVE: 0.1640782952 | 📌 HamCycle | 🕗 CERTIFY: 0.1075495854
| 🇳  118 | ⭕️    2246720 | 🕗 SOLVE: 0.1730021536 | 📌 HamCycle | 🕗 CERTIFY: 0.1086438298
| 🇳  119 | ⭕️    2303840 | 🕗 SOLVE: 0.1788444221 | 📌 HamCycle | 🕗 CERTIFY: 0.1085794568
| 🇳  120 | ⭕️    2361920 | 🕗 SOLVE: 0.1813175827 | 📌 HamCycle | 🕗 CERTIFY: 0.1110610813
| 🇳  121 | ⭕️    2420968 | 🕗 SOLVE: 0.1922602057 | 📌 HamCycle | 🕗 CERTIFY: 0.1123071685
| 🇳  122 | ⭕️    2480992 | 🕗 SOLVE: 0.2007973343 | 📌 HamCycle | 🕗 CERTIFY: 0.1143695861
| 🇳  123 | ⭕️    2542000 | 🕗 SOLVE: 0.2024549544 | 📌 HamCycle | 🕗 CERTIFY: 0.1166447550
| 🇳  124 | ⭕️    2604000 | 🕗 SOLVE: 0.2132250816 | 📌 HamCycle | 🕗 CERTIFY: 0.1178896651
| 🇳  125 | ⭕️    2667000 | 🕗 SOLVE: 0.2167387456 | 📌 HamCycle | 🕗 CERTIFY: 0.1201791689
| 🇳  126 | ⭕️    2731008 | 🕗 SOLVE: 0.2255000472 | 📌 HamCycle | 🕗 CERTIFY: 0.1213071644
| 🇳  127 | ⭕️    2796032 | 🕗 SOLVE: 0.2362187505 | 📌 HamCycle | 🕗 CERTIFY: 0.1243598312
| 🇳  128 | ⭕️    2862080 | 🕗 SOLVE: 0.2410653830 | 📌 HamCycle | 🕗 CERTIFY: 0.1240152940
| 🇳  129 | ⭕️    2929160 | 🕗 SOLVE: 0.2460841537 | 📌 HamCycle | 🕗 CERTIFY: 0.1269655824
| 🇳  130 | ⭕️    2997280 | 🕗 SOLVE: 0.2532179058 | 📌 HamCycle | 🕗 CERTIFY: 0.1299516261
| 🇳  131 | ⭕️    3066448 | 🕗 SOLVE: 0.2628486156 | 📌 HamCycle | 🕗 CERTIFY: 0.1321863383
| 🇳  132 | ⭕️    3136672 | 🕗 SOLVE: 0.2649004757 | 📌 HamCycle | 🕗 CERTIFY: 0.1346864104
| 🇳  133 | ⭕️    3207960 | 🕗 SOLVE: 0.2778384387 | 📌 HamCycle | 🕗 CERTIFY: 0.1367300004
| 🇳  134 | ⭕️    3280320 | 🕗 SOLVE: 0.2833045125 | 📌 HamCycle | 🕗 CERTIFY: 0.1402141303
| 🇳  135 | ⭕️    3353760 | 🕗 SOLVE: 0.2946422398 | 📌 HamCycle | 🕗 CERTIFY: 0.1436815113
| 🇳  136 | ⭕️    3428288 | 🕗 SOLVE: 0.2991968393 | 📌 HamCycle | 🕗 CERTIFY: 0.1467610896
| 🇳  137 | ⭕️    3503912 | 🕗 SOLVE: 0.3069812655 | 📌 HamCycle | 🕗 CERTIFY: 0.1489024162
| 🇳  138 | ⭕️    3580640 | 🕗 SOLVE: 0.3207840919 | 📌 HamCycle | 🕗 CERTIFY: 0.1535527110
| 🇳  139 | ⭕️    3658480 | 🕗 SOLVE: 0.3352535665 | 📌 HamCycle | 🕗 CERTIFY: 0.1575685740
| 🇳  140 | ⭕️    3737440 | 🕗 SOLVE: 0.3307761550 | 📌 HamCycle | 🕗 CERTIFY: 0.2192643434
| 🇳  141 | ⭕️    3817528 | 🕗 SOLVE: 0.3490007222 | 📌 HamCycle | 🕗 CERTIFY: 0.2219216377
| 🇳  142 | ⭕️    3898752 | 🕗 SOLVE: 0.3602769673 | 📌 HamCycle | 🕗 CERTIFY: 0.2263452411
| 🇳  143 | ⭕️    3981120 | 🕗 SOLVE: 0.3776981831 | 📌 HamCycle | 🕗 CERTIFY: 0.2272515297
| 🇳  144 | ⭕️    4064640 | 🕗 SOLVE: 0.3799947798 | 📌 HamCycle | 🕗 CERTIFY: 0.2284243405
| 🇳  145 | ⭕️    4149320 | 🕗 SOLVE: 0.3879981637 | 📌 HamCycle | 🕗 CERTIFY: 0.2318513691
| 🇳  146 | ⭕️    4235168 | 🕗 SOLVE: 0.4014013410 | 📌 HamCycle | 🕗 CERTIFY: 0.2391704619
| 🇳  147 | ⭕️    4322192 | 🕗 SOLVE: 0.4134836197 | 📌 HamCycle | 🕗 CERTIFY: 0.2364137918
| 🇳  148 | ⭕️    4410400 | 🕗 SOLVE: 0.4204864204 | 📌 HamCycle | 🕗 CERTIFY: 0.2379920781
| 🇳  149 | ⭕️    4499800 | 🕗 SOLVE: 0.4251140654 | 📌 HamCycle | 🕗 CERTIFY: 0.2416902035
| 🇳  150 | ⭕️    4590400 | 🕗 SOLVE: 0.4496076107 | 📌 HamCycle | 🕗 CERTIFY: 0.2455630451
| 🇳  151 | ⭕️    4682208 | 🕗 SOLVE: 0.4511633515 | 📌 HamCycle | 🕗 CERTIFY: 0.2464859933
| 🇳  152 | ⭕️    4775232 | 🕗 SOLVE: 0.4632550776 | 📌 HamCycle | 🕗 CERTIFY: 0.2495052218
| 🇳  153 | ⭕️    4869480 | 🕗 SOLVE: 0.4739755392 | 📌 HamCycle | 🕗 CERTIFY: 0.2500919104
| 🇳  154 | ⭕️    4964960 | 🕗 SOLVE: 0.4721223414 | 📌 HamCycle | 🕗 CERTIFY: 0.2554323375
| 🇳  155 | ⭕️    5061680 | 🕗 SOLVE: 0.5112373829 | 📌 HamCycle | 🕗 CERTIFY: 0.2604016364
| 🇳  156 | ⭕️    5159648 | 🕗 SOLVE: 0.5268186331 | 📌 HamCycle | 🕗 CERTIFY: 0.2619829476
| 🇳  157 | ⭕️    5258872 | 🕗 SOLVE: 0.5287690759 | 📌 HamCycle | 🕗 CERTIFY: 0.2648456991
| 🇳  158 | ⭕️    5359360 | 🕗 SOLVE: 0.5346980691 | 📌 HamCycle | 🕗 CERTIFY: 0.2659545541
| 🇳  159 | ⭕️    5461120 | 🕗 SOLVE: 0.5520305634 | 📌 HamCycle | 🕗 CERTIFY: 0.2713535428
| 🇳  160 | ⭕️    5564160 | 🕗 SOLVE: 0.5735202432 | 📌 HamCycle | 🕗 CERTIFY: 0.2718769312
| 🇳  161 | ⭕️    5668488 | 🕗 SOLVE: 0.5838571787 | 📌 HamCycle | 🕗 CERTIFY: 0.3109790087
| 🇳  162 | ⭕️    5774112 | 🕗 SOLVE: 0.6009947062 | 📌 HamCycle | 🕗 CERTIFY: 0.3170869946
| 🇳  163 | ⭕️    5881040 | 🕗 SOLVE: 0.6238725781 | 📌 HamCycle | 🕗 CERTIFY: 0.2843618095
| 🇳  164 | ⭕️    5989280 | 🕗 SOLVE: 0.6478753686 | 📌 HamCycle | 🕗 CERTIFY: 0.3228500783
| 🇳  165 | ⭕️    6098840 | 🕗 SOLVE: 0.6504259109 | 📌 HamCycle | 🕗 CERTIFY: 0.2913619280
| 🇳  166 | ⭕️    6209728 | 🕗 SOLVE: 0.6625688672 | 📌 HamCycle | 🕗 CERTIFY: 0.3298715651
| 🇳  167 | ⭕️    6321952 | 🕗 SOLVE: 0.6739610434 | 📌 HamCycle | 🕗 CERTIFY: 0.3013093770
| 🇳  168 | ⭕️    6435520 | 🕗 SOLVE: 0.7048268318 | 📌 HamCycle | 🕗 CERTIFY: 0.3039506376
| 🇳  169 | ⭕️    6550440 | 🕗 SOLVE: 0.7104204893 | 📌 HamCycle | 🕗 CERTIFY: 0.3472827971
| 🇳  170 | ⭕️    6666720 | 🕗 SOLVE: 0.7373938560 | 📌 HamCycle | 🕗 CERTIFY: 0.3158228397
| 🇳  171 | ⭕️    6784368 | 🕗 SOLVE: 0.7458135486 | 📌 HamCycle | 🕗 CERTIFY: 0.3525474370
| 🇳  172 | ⭕️    6903392 | 🕗 SOLVE: 0.7618639469 | 📌 HamCycle | 🕗 CERTIFY: 0.3256621659
| 🇳  173 | ⭕️    7023800 | 🕗 SOLVE: 0.7898070812 | 📌 HamCycle | 🕗 CERTIFY: 0.3736437559
| 🇳  174 | ⭕️    7145600 | 🕗 SOLVE: 0.8101043105 | 📌 HamCycle | 🕗 CERTIFY: 0.3732929826
| 🇳  175 | ⭕️    7268800 | 🕗 SOLVE: 0.8271037936 | 📌 HamCycle | 🕗 CERTIFY: 0.3466070592
| 🇳  176 | ⭕️    7393408 | 🕗 SOLVE: 0.8450680971 | 📌 HamCycle | 🕗 CERTIFY: 0.5331867337
| 🇳  177 | ⭕️    7519432 | 🕗 SOLVE: 0.8592913747 | 📌 HamCycle | 🕗 CERTIFY: 0.6314012408
| 🇳  178 | ⭕️    7646880 | 🕗 SOLVE: 0.8963866830 | 📌 HamCycle | 🕗 CERTIFY: 0.5319378972
| 🇳  179 | ⭕️    7775760 | 🕗 SOLVE: 0.9083073139 | 📌 HamCycle | 🕗 CERTIFY: 0.5346336365
| 🇳  180 | ⭕️    7906080 | 🕗 SOLVE: 0.9260295630 | 📌 HamCycle | 🕗 CERTIFY: 0.5349258780
| 🇳  181 | ⭕️    8037848 | 🕗 SOLVE: 0.9352477789 | 📌 HamCycle | 🕗 CERTIFY: 0.6859945059
| 🇳  182 | ⭕️    8171072 | 🕗 SOLVE: 0.9673479199 | 📌 HamCycle | 🕗 CERTIFY: 0.6562106013
| 🇳  183 | ⭕️    8305760 | 🕗 SOLVE: 0.9852146506 | 📌 HamCycle | 🕗 CERTIFY: 0.6593267322
| 🇳  184 | ⭕️    8441920 | 🕗 SOLVE: 1.0090435743 | 📌 HamCycle | 🕗 CERTIFY: 0.6680057645
| 🇳  185 | ⭕️    8579560 | 🕗 SOLVE: 1.0231652260 | 📌 HamCycle | 🕗 CERTIFY: 0.6725590825
| 🇳  186 | ⭕️    8718688 | 🕗 SOLVE: 1.0425723791 | 📌 HamCycle | 🕗 CERTIFY: 0.5642858744
| 🇳  187 | ⭕️    8859312 | 🕗 SOLVE: 1.0798088312 | 📌 HamCycle | 🕗 CERTIFY: 0.5689097047
| 🇳  188 | ⭕️    9001440 | 🕗 SOLVE: 1.0968896151 | 📌 HamCycle | 🕗 CERTIFY: 0.5746728778
| 🇳  189 | ⭕️    9145080 | 🕗 SOLVE: 1.1234657764 | 📌 HamCycle | 🕗 CERTIFY: 0.5791327357
| 🇳  190 | ⭕️    9290240 | 🕗 SOLVE: 1.1419420242 | 📌 HamCycle | 🕗 CERTIFY: 0.6962649822
| 🇳  191 | ⭕️    9436928 | 🕗 SOLVE: 1.1405230761 | 📌 HamCycle | 🕗 CERTIFY: 0.5903390646
| 🇳  192 | ⭕️    9585152 | 🕗 SOLVE: 1.1948571205 | 📌 HamCycle | 🕗 CERTIFY: 0.7087152600
| 🇳  193 | ⭕️    9734920 | 🕗 SOLVE: 1.1910440922 | 📌 HamCycle | 🕗 CERTIFY: 0.6030912995
| 🇳  194 | ⭕️    9886240 | 🕗 SOLVE: 1.2376817465 | 📌 HamCycle | 🕗 CERTIFY: 0.6026688218
| 🇳  195 | ⭕️   10039120 | 🕗 SOLVE: 1.2475427389 | 📌 HamCycle | 🕗 CERTIFY: 0.6125047207
| 🇳  196 | ⭕️   10193568 | 🕗 SOLVE: 1.2770527601 | 📌 HamCycle | 🕗 CERTIFY: 0.6152799129
| 🇳  197 | ⭕️   10349592 | 🕗 SOLVE: 1.3150676489 | 📌 HamCycle | 🕗 CERTIFY: 0.6209831238
| 🇳  198 | ⭕️   10507200 | 🕗 SOLVE: 1.3332883120 | 📌 HamCycle | 🕗 CERTIFY: 0.6276137233
| 🇳  199 | ⭕️   10666400 | 🕗 SOLVE: 1.3530586958 | 📌 HamCycle | 🕗 CERTIFY: 0.6293233633
| 🇳  200 | ⭕️   10827200 | 🕗 SOLVE: 1.4353644848 | 📌 HamCycle | 🕗 CERTIFY: 0.6387774348
| 🇳  201 | ⭕️   10989608 | 🕗 SOLVE: 1.4160395861 | 📌 HamCycle | 🕗 CERTIFY: 0.6412240267
| 🇳  202 | ⭕️   11153632 | 🕗 SOLVE: 1.4769572020 | 📌 HamCycle | 🕗 CERTIFY: 0.6507937312
| 🇳  203 | ⭕️   11319280 | 🕗 SOLVE: 1.4897103310 | 📌 HamCycle | 🕗 CERTIFY: 0.6656789780
| 🇳  204 | ⭕️   11486560 | 🕗 SOLVE: 1.5059686899 | 📌 HamCycle | 🕗 CERTIFY: 0.6679863334
| 🇳  205 | ⭕️   11655480 | 🕗 SOLVE: 1.5308849812 | 📌 HamCycle | 🕗 CERTIFY: 0.8200314045
| 🇳  206 | ⭕️   11826048 | 🕗 SOLVE: 1.6000189781 | 📌 HamCycle | 🕗 CERTIFY: 0.6429114938
| 🇳  207 | ⭕️   11998272 | 🕗 SOLVE: 1.6053988934 | 📌 HamCycle | 🕗 CERTIFY: 0.7964468598
| 🇳  208 | ⭕️   12172160 | 🕗 SOLVE: 1.6357505322 | 📌 HamCycle | 🕗 CERTIFY: 0.7194095850
| 🇳  209 | ⭕️   12347720 | 🕗 3.158135    |
| 🇳  210 | ⭕️   12524960 | 🕗 3.40144     |
| 🇳  211 | ⭕️   12703888 | 🕗 3.3752189   |
| 🇳  212 | ⭕️   12884512 | 🕗 3.4984384   |
| 🇳  213 | ⭕️   13066840 | 🕗 3.5713162   |
| 🇳  214 | ⭕️   13250880 | 🕗 3.6841693   |
| 🇳  215 | ⭕️   13436640 | 🕗 3.6591806   |
| 🇳  216 | ⭕️   13624128 | 🕗 3.7989397   |
| 🇳  217 | ⭕️   13813352 | 🕗 3.8821692   |
| 🇳  218 | ⭕️   14004320 | 🕗 3.9267392   |
| 🇳  219 | ⭕️   14197040 | 🕗 3.9715004   |
| 🇳  220 | ⭕️   14391520 | 🕗 3.94625     |
| 🇳  221 | ⭕️   14587768 | 🕗 4.1092734   |
| 🇳  222 | ⭕️   14785792 | 🕗 4.169661    |
| 🇳  223 | ⭕️   14985600 | 🕗 4.322804    |
| 🇳  224 | ⭕️   15187200 | 🕗 4.3841324   |
| 🇳  225 | ⭕️   15390600 | 🕗 4.396573    |
| 🇳  226 | ⭕️   15595808 | 🕗 4.5972123   |
| 🇳  227 | ⭕️   15802832 | 🕗 4.5757213   |
| 🇳  228 | ⭕️   16011680 | 🕗 4.633295    |
| 🇳  229 | ⭕️   16222360 | 🕗 4.573985    |
| 🇳  230 | ⭕️   16434880 | 🕗 4.7796025   |
| 🇳  231 | ⭕️   16649248 | 🕗 4.9244943   |
| 🇳  232 | ⭕️   16865472 | 🕗 5.0315943   |
| 🇳  233 | ⭕️   17083560 | 🕗 4.957198    |
| 🇳  234 | ⭕️   17303520 | 🕗 5.175337    |
| 🇳  235 | ⭕️   17525360 | 🕗 5.284099    |
| 🇳  236 | ⭕️   17749088 | 🕗 5.1999283   |
| 🇳  237 | ⭕️   17974712 | 🕗 5.392593    |
| 🇳  238 | ⭕️   18202240 | 🕗 5.394155    |
| 🇳  239 | ⭕️   18431680 | 🕗 5.656842    |
| 🇳  240 | ⭕️   18663040 | 🕗 5.646858    |
| 🇳  241 | ⭕️   18896328 | 🕗 5.714145    |
| 🇳  242 | ⭕️   19131552 | 🕗 5.948969    |
| 🇳  243 | ⭕️   19368720 | 🕗 6.099502    |
| 🇳  244 | ⭕️   19607840 | 🕗 6.0746346   |
| 🇳  245 | ⭕️   19848920 | 🕗 6.0399275   |
| 🇳  246 | ⭕️   20091968 | 🕗 6.2939715   |
| 🇳  247 | ⭕️   20336992 | 🕗 6.397452    |
| 🇳  248 | ⭕️   20584000 | 🕗 6.4761877   |
| 🇳  249 | ⭕️   20833000 | 🕗 6.526003    |
| 🇳  250 | ⭕️   21084000 | 🕗 6.659648    |
| 🇳  251 | ⭕️   21337008 | 🕗 6.829479    |
| 🇳  252 | ⭕️   21592032 | 🕗 6.9698257   |
| 🇳  253 | ⭕️   21849080 | 🕗 7.212076    |
| 🇳  254 | ⭕️   22108160 | 🕗 7.242034    |
| 🇳  255 | ⭕️   22369280 | 🕗 7.038605    |
| 🇳  256 | ⭕️   22632448 | 🕗 7.2706337   |
| 🇳  257 | ⭕️   22897672 | 🕗 7.405891    |
| 🇳  258 | ⭕️   23164960 | 🕗 7.665243    |
| 🇳  259 | ⭕️   23434320 | 🕗 7.7834406   |
| 🇳  260 | ⭕️   23705760 | 🕗 7.8104124   |
| 🇳  261 | ⭕️   23979288 | 🕗 7.8971953   |
| 🇳  262 | ⭕️   24254912 | 🕗 7.911546    |
| 🇳  263 | ⭕️   24532640 | 🕗 8.096141    |
| 🇳  264 | ⭕️   24812480 | 🕗 8.263163    |
| 🇳  265 | ⭕️   25094440 | 🕗 8.247822    |
| 🇳  266 | ⭕️   25378528 | 🕗 8.536981    |
| 🇳  267 | ⭕️   25664752 | 🕗 8.425643    |
| 🇳  268 | ⭕️   25953120 | 🕗 8.804618    |
| 🇳  269 | ⭕️   26243640 | 🕗 8.737803    |
| 🇳  270 | ⭕️   26536320 | 🕗 9.081698    |
| 🇳  271 | ⭕️   26831168 | 🕗 8.961112    |
| 🇳  272 | ⭕️   27128192 | 🕗 9.387175    |
| 🇳  273 | ⭕️   27427400 | 🕗 9.264832    |
| 🇳  274 | ⭕️   27728800 | 🕗 9.84508     |
| 🇳  275 | ⭕️   28032400 | 🕗 9.617822    |
| 🇳  276 | ⭕️   28338208 | 🕗 9.948429    |
| 🇳  277 | ⭕️   28646232 | 🕗 10.012301   |
| 🇳  278 | ⭕️   28956480 | 🕗 10.162512   |
| 🇳  279 | ⭕️   29268960 | 🕗 10.246682   |
| 🇳  280 | ⭕️   29583680 | 🕗 10.729552   |
| 🇳  281 | ⭕️   29900648 | 🕗 10.638324   |
| 🇳  282 | ⭕️   30219872 | 🕗 10.831721   |
| 🇳  283 | ⭕️   30541360 | 🕗 10.593552   |
| 🇳  284 | ⭕️   30865120 | 🕗 11.029773   |
| 🇳  285 | ⭕️   31191160 | 🕗 10.914741   |
| 🇳  286 | ⭕️   31519488 | 🕗 11.613157   |
| 🇳  287 | ⭕️   31850112 | 🕗 11.576298   |
| 🇳  288 | ⭕️   32183040 | 🕗 11.737955   |
| 🇳  289 | ⭕️   32518280 | 🕗 11.839574   |
| 🇳  290 | ⭕️   32855840 | 🕗 11.878326   |
| 🇳  291 | ⭕️   33195728 | 🕗 11.968829   |
| 🇳  292 | ⭕️   33537952 | 🕗 12.379219   |
| 🇳  293 | ⭕️   33882520 | 🕗 12.531543   |
| 🇳  294 | ⭕️   34229440 | 🕗 12.753095   |
| 🇳  295 | ⭕️   34578720 | 🕗 12.61166    |
| 🇳  296 | ⭕️   34930368 | 🕗 12.874686   |
| 🇳  297 | ⭕️   35284392 | 🕗 13.268115   |
| 🇳  298 | ⭕️   35640800 | 🕗 13.757408   |
| 🇳  299 | ⭕️   35999600 | 🕗 13.78506    |
| 🇳  300 | ⭕️   36360800 | 🕗 13.678138   |
| 🇳  301 | ⭕️   36724408 | 🕗 13.980104   |
| 🇳  302 | ⭕️   37090432 | 🕗 14.037015   |
| 🇳  303 | ⭕️   37458880 | 🕗 14.315289   |
| 🇳  304 | ⭕️   37829760 | 🕗 14.424659   |
| 🇳  305 | ⭕️   38203080 | 🕗 14.710381   |
| 🇳  306 | ⭕️   38578848 | 🕗 14.761967   |
| 🇳  307 | ⭕️   38957072 | 🕗 14.691561   |
| 🇳  308 | ⭕️   39337760 | 🕗 15.450688   |
| 🇳  309 | ⭕️   39720920 | 🕗 15.573951   |
| 🇳  310 | ⭕️   40106560 | 🕗 15.581749   |
| 🇳  311 | ⭕️   40494688 | 🕗 15.875947   |
| 🇳  312 | ⭕️   40885312 | 🕗 16.263403   |
| 🇳  313 | ⭕️   41278440 | 🕗 16.04307    |
| 🇳  314 | ⭕️   41674080 | 🕗 16.455729   |
| 🇳  315 | ⭕️   42072240 | 🕗 16.6889     |
| 🇳  316 | ⭕️   42472928 | 🕗 16.784016   |
| 🇳  317 | ⭕️   42876152 | 🕗 17.418867   |
| 🇳  318 | ⭕️   43281920 | 🕗 17.406      |
| 🇳  319 | ⭕️   43690240 | 🕗 17.667955   |
| 🇳  320 | ⭕️   44101120 | 🕗 17.942705   |
| 🇳  321 | ⭕️   44514568 | 🕗 17.884752   |
| 🇳  322 | ⭕️   44930592 | 🕗 18.308977   |
| 🇳  323 | ⭕️   45349200 | 🕗 18.488873   |
| 🇳  324 | ⭕️   45770400 | 🕗 18.35628    |
| 🇳  325 | ⭕️   46194200 | 🕗 18.71288    |
| 🇳  326 | ⭕️   46620608 | 🕗 18.776562   |
| 🇳  327 | ⭕️   47049632 | 🕗 19.564642   |
| 🇳  328 | ⭕️   47481280 | 🕗 19.43866    |
| 🇳  329 | ⭕️   47915560 | 🕗 19.573055   |
| 🇳  330 | ⭕️   48352480 | 🕗 20.014952   |
| 🇳  331 | ⭕️   48792048 | 🕗 20.129084   |
| 🇳  332 | ⭕️   49234272 | 🕗 20.332262   |
| 🇳  333 | ⭕️   49679160 | 🕗 21.20825    | 
| 🇳  334 | ⭕️   50126720 | 🕗 21.457092   |
| 🇳  335 | ⭕️   50576960 | 🕗 21.080397   |
| 🇳  336 | ⭕️   51029888 | 🕗 21.408003   |
| 🇳  337 | ⭕️   51485512 | 🕗 21.73982    |
| 🇳  338 | ⭕️   51943840 | 🕗 22.051832   |
| 🇳  339 | ⭕️   52404880 | 🕗 22.409481   |
| 🇳  340 | ⭕️   52868640 | 🕗 22.715778   |
| 🇳  341 | ⭕️   53335128 | 🕗 23.081837   |
| 🇳  342 | ⭕️   53804352 | 🕗 23.221256   |
| 🇳  343 | ⭕️   54276320 | 🕗 23.67122    |
| 🇳  344 | ⭕️   54751040 | 🕗 24.011658   |
| 🇳  345 | ⭕️   55228520 | 🕗 23.92422    |
| 🇳  346 | ⭕️   55708768 | 🕗 24.179146   |
| 🇳  347 | ⭕️   56191792 | 🕗 24.347809   |
| 🇳  348 | ⭕️   56677600 | 🕗 25.039564   |
| 🇳  349 | ⭕️   57166200 | 🕗 25.176973   |
| 🇳  350 | ⭕️   57657600 | 🕗 25.57811    |
| 🇳  351 | ⭕️   58151808 | 🕗 25.53219    |
| 🇳  352 | ⭕️   58648832 | 🕗 26.307041   |
| 🇳  353 | ⭕️   59148680 | 🕗 26.357397   |
| 🇳  354 | ⭕️   59651360 | 🕗 26.2418     |
| 🇳  355 | ⭕️   60156880 | 🕗 27.201225   |
| 🇳  356 | ⭕️   60665248 | 🕗 27.106916   |
| 🇳  357 | ⭕️   61176472 | 🕗 27.04518    |
| 🇳  358 | ⭕️   61690560 | 🕗 27.696089   |
| 🇳  359 | ⭕️   62207520 | 🕗 28.637325   |
| 🇳  360 | ⭕️   62727360 | 🕗 28.951366   |
| 🇳  361 | ⭕️   63250088 | 🕗 28.253342   |
| 🇳  362 | ⭕️   63775712 | 🕗 29.16707    |
| 🇳  363 | ⭕️   64304240 | 🕗 28.9471     |
| 🇳  364 | ⭕️   64835680 | 🕗 29.58885    |
| 🇳  365 | ⭕️   65370040 | 🕗 30.277336   |
| 🇳  366 | ⭕️   65907328 | 🕗 30.043657   |
| 🇳  367 | ⭕️   66447552 | 🕗 30.222782   |
| 🇳  368 | ⭕️   66990720 | 🕗 31.22274    |
| 🇳  369 | ⭕️   67536840 | 🕗 30.609814   |
| 🇳  370 | ⭕️   68085920 | 🕗 31.520184   |
| 🇳  371 | ⭕️   68637968 | 🕗 31.897694   |
| 🇳  372 | ⭕️   69192992 | 🕗 32.284607   |
| 🇳  373 | ⭕️   69751000 | 🕗 32.355698   |
| 🇳  374 | ⭕️   70312000 | 🕗 33.37469    |
| 🇳  375 | ⭕️   70876000 | 🕗 33.464195   |
| 🇳  376 | ⭕️   71443008 | 🕗 33.66532    |
| 🇳  377 | ⭕️   72013032 | 🕗 34.122574   |
| 🇳  378 | ⭕️   72586080 | 🕗 34.4813     |
| 🇳  379 | ⭕️   73162160 | 🕗 34.336918   |
| 🇳  380 | ⭕️   73741280 | 🕗 35.77628    |
| 🇳  381 | ⭕️   74323448 | 🕗 35.234943   |
| 🇳  382 | ⭕️   74908672 | 🕗 36.02649    |
| 🇳  383 | ⭕️   75496960 | 🕗 36.24739    |
| 🇳  384 | ⭕️   76088320 | 🕗 36.68922    |
| 🇳  385 | ⭕️   76682760 | 🕗 37.538635   |
| 🇳  386 | ⭕️   77280288 | 🕗 36.894955   |
| 🇳  387 | ⭕️   77880912 | 🕗 36.483852   |
| 🇳  388 | ⭕️   78484640 | 🕗 38.60994    |
| 🇳  389 | ⭕️   79091480 | 🕗 38.232834   |
| 🇳  390 | ⭕️   79701440 | 🕗 40.01598    |
| 🇳  391 | ⭕️   80314528 | 🕗 39.32995    |
| 🇳  392 | ⭕️   80930752 | 🕗 39.674416   |
| 🇳  393 | ⭕️   81550120 | 🕗 40.853016   |
| 🇳  394 | ⭕️   82172640 | 🕗 42.55716    |
| 🇳  395 | ⭕️   82798320 | 🕗 42.104794   |
| 🇳  396 | ⭕️   83427168 | 🕗 42.74134    |
| 🇳  397 | ⭕️   84059192 | 🕗 41.433784   |
| 🇳  398 | ⭕️   84694400 | 🕗 42.718132   |
| 🇳  399 | ⭕️   85332800 | 🕗 43.25699    |
| 🇳  400 | ⭕️   85974400 | 🕗 43.4453     |
| 🇳  401 | ⭕️   86619208 | 🕗 43.56622    |
| 🇳  402 | ⭕️   87267232 | 🕗 44.402424   |
| 🇳  403 | ⭕️   87918480 | 🕗 43.810234   |
| 🇳  404 | ⭕️   88572960 | 🕗 44.8134     |
| 🇳  405 | ⭕️   89230680 | 🕗 45.785404   |
| 🇳  406 | ⭕️   89891648 | 🕗 45.568077   |
| 🇳  407 | ⭕️   90555872 | 🕗 46.21579    |
| 🇳  408 | ⭕️   91223360 | 🕗 46.33387    |
| 🇳  409 | ⭕️   91894120 | 🕗 47.211132   |
| 🇳  410 | ⭕️   92568160 | 🕗 47.72589    |
| 🇳  411 | ⭕️   93245488 | 🕗 48.311825   |
| 🇳  412 | ⭕️   93926112 | 🕗 48.28511    |
| 🇳  413 | ⭕️   94610040 | 🕗 48.761448   |
| 🇳  414 | ⭕️   95297280 | 🕗 48.40843    |
| 🇳  415 | ⭕️   95987840 | 🕗 50.058735   |
| 🇳  416 | ⭕️   96681728 | 🕗 49.6022     |
| 🇳  417 | ⭕️   97378952 | 🕗 50.980072   |
| 🇳  418 | ⭕️   98079520 | 🕗 51.562206   |
| 🇳  419 | ⭕️   98783440 | 🕗 51.98804    |
| 🇳  420 | ⭕️   99490720 | 🕗 52.15408    |
| 🇳  421 | ⭕️  100201368 | 🕗 52.846794   |
| 🇳  422 | ⭕️  100915392 | 🕗 52.69293    |
| 🇳  423 | ⭕️  101632800 | 🕗 52.90274    |
| 🇳  424 | ⭕️  102353600 | 🕗 55.14972    |
| 🇳  425 | ⭕️  103077800 | 🕗 55.05817    |
| 🇳  426 | ⭕️  103805408 | 🕗 55.527298   |
| 🇳  427 | ⭕️  104536432 | 🕗 55.779793   |
| 🇳  428 | ⭕️  105270880 | 🕗 55.419388   |
| 🇳  429 | ⭕️  106008760 | 🕗 55.916412   |
| 🇳  430 | ⭕️  106750080 | 🕗 57.81927    |
| 🇳  431 | ⭕️  107494848 | 🕗 58.167828   |
| 🇳  432 | ⭕️  108243072 | 🕗 58.28139    |
| 🇳  433 | ⭕️  108994760 | 🕗 59.325485   |
| 🇳  434 | ⭕️  109749920 | 🕗 59.921696   |
| 🇳  435 | ⭕️  110508560 | 🕗 59.61137    |
| 🇳  436 | ⭕️  111270688 | 🕗 61.44322    |
| 🇳  437 | ⭕️  112036312 | 🕗 60.483303   |
| 🇳  438 | ⭕️  112805440 | 🕗 61.493465   |
| 🇳  439 | ⭕️  113578080 | 🕗 61.8513     |
| 🇳  440 | ⭕️  114354240 | 🕗 63.502224   |
| 🇳  441 | ⭕️  115133928 | 🕗 62.737644   |
| 🇳  442 | ⭕️  115917152 | 🕗 64.58094    |
| 🇳  443 | ⭕️  116703920 | 🕗 65.304146   |
| 🇳  444 | ⭕️  117494240 | 🕗 65.37117    |
| 🇳  445 | ⭕️  118288120 | 🕗 65.63902    |
| 🇳  446 | ⭕️  119085568 | 🕗 66.89195    |
| 🇳  447 | ⭕️  119886592 | 🕗 66.99914    |
| 🇳  448 | ⭕️  120691200 | 🕗 67.54571    |
| 🇳  449 | ⭕️  121499400 | 🕗 68.99263    |
| 🇳  450 | ⭕️  122311200 | 🕗 68.316055   |
| 🇳  451 | ⭕️  123126608 | 🕗 70.68565    |
| 🇳  452 | ⭕️  123945632 | 🕗 71.80562    |
| 🇳  453 | ⭕️  124768280 | 🕗 71.40056    |
| 🇳  454 | ⭕️  125594560 | 🕗 71.88125    |
| 🇳  455 | ⭕️  126424480 | 🕗 71.9981     |
| 🇳  456 | ⭕️  127258048 | 🕗 72.11591    |
| 🇳  457 | ⭕️  128095272 | 🕗 74.21757    |
| 🇳  458 | ⭕️  128936160 | 🕗 74.63009    |
| 🇳  459 | ⭕️  129780720 | 🕗 73.02252    |
| 🇳  460 | ⭕️  130628960 | 🕗 73.89332    |
| 🇳  461 | ⭕️  131480888 | 🕗 75.19101    |
| 🇳  462 | ⭕️  132336512 | 🕗 76.97239    |
| 🇳  463 | ⭕️  133195840 | 🕗 77.20666    |
| 🇳  464 | ⭕️  134058880 | 🕗 77.44523    |
| 🇳  465 | ⭕️  134925640 | 🕗 78.39817    |
| 🇳  466 | ⭕️  135796128 | 🕗 79.58925    |
| 🇳  467 | ⭕️  136670352 | 🕗 79.29805    |
| 🇳  468 | ⭕️  137548320 | 🕗 80.30439    |
| 🇳  469 | ⭕️  138430040 | 🕗 81.391716   |
| 🇳  470 | ⭕️  139315520 | 🕗 85.98531    |
| 🇳  471 | ⭕️  140204768 | 🕗 82.52525    |
| 🇳  472 | ⭕️  141097792 | 🕗 84.19845    |
| 🇳  473 | ⭕️  141994600 | 🕗 84.259796   |
| 🇳  474 | ⭕️  142895200 | 🕗 84.673645   |
| 🇳  475 | ⭕️  143799600 | 🕗 85.17486    |
| 🇳  476 | ⭕️  144707808 | 🕗 85.61029    |
| 🇳  477 | ⭕️  145619832 | 🕗 86.62455    |
| 🇳  478 | ⭕️  146535680 | 🕗 88.19117    |
| 🇳  479 | ⭕️  147455360 | 🕗 90.929886   |
| 🇳  480 | ⭕️  148378880 | 🕗 90.09865    |
| 🇳  481 | ⭕️  149306248 | 🕗 91.11378    |
| 🇳  482 | ⭕️  150237472 | 🕗 92.444305   |
| 🇳  483 | ⭕️  151172560 | 🕗 95.09655    |
| 🇳  484 | ⭕️  152111520 | 🕗 95.48401    |
| 🇳  485 | ⭕️  153054360 | 🕗 94.144684   |
| 🇳  486 | ⭕️  154001088 | 🕗 94.22827    |
| 🇳  487 | ⭕️  154951712 | 🕗 94.48442    |
| 🇳  488 | ⭕️  155906240 | 🕗 97.66606    |
| 🇳  489 | ⭕️  156864680 | 🕗 97.10132    |
| 🇳  490 | ⭕️  157827040 | 🕗 97.924      |
| 🇳  491 | ⭕️  158793328 | 🕗 98.52145    |
| 🇳  492 | ⭕️  159763552 | 🕗 98.9019     |
| 🇳  493 | ⭕️  160737720 | 🕗 99.43956    |
| 🇳  494 | ⭕️  161715840 | 🕗 101.24073   |
| 🇳  495 | ⭕️  162697920 | 🕗 99.230034   |
| 🇳  496 | ⭕️  163683968 | 🕗 101.946434  |
| 🇳  497 | ⭕️  164673992 | 🕗 101.60298   |
| 🇳  498 | ⭕️  165668000 | 🕗 103.14232   |
| 🇳  499 | ⭕️  166666000 | 🕗 103.860695  |
| 🇳  500 | ⭕️  167668000 | 🕗 105.15138   |
| 🇳  501 | ⭕️  168674008 | 🕗 104.618515  |
| 🇳  502 | ⭕️  169684032 | 🕗 105.916595  |
| 🇳  503 | ⭕️  170698080 | 🕗 108.375114  |
| 🇳  504 | ⭕️  171716160 | 🕗 110.62443   |
| 🇳  505 | ⭕️  172738280 | 🕗 110.553566  |
| 🇳  506 | ⭕️  173764448 | 🕗 112.83786   |
| 🇳  507 | ⭕️  174794672 | 🕗 112.60872   |
| 🇳  508 | ⭕️  175828960 | 🕗 113.127365  |
| 🇳  509 | ⭕️  176867320 | 🕗 112.71225   |
| 🇳  510 | ⭕️  177909760 | 🕗 115.11601   |
| 🇳  511 | ⭕️  178956288 | 🕗 116.07203   |
| 🇳  512 | ⭕️  180006912 | 🕗 116.35614   |
| 🇳  513 | ⭕️  181061640 | 🕗 116.87842   |
| 🇳  514 | ⭕️  182120480 | 🕗 120.89874   |
| 🇳  515 | ⭕️  183183440 | 🕗 120.38289   |
| 🇳  516 | ⭕️  184250528 | 🕗 119.761566  |
| 🇳  517 | ⭕️  185321752 | 🕗 121.59659   |
| 🇳  518 | ⭕️  186397120 | 🕗 121.59104   |
| 🇳  519 | ⭕️  187476640 | 🕗 119.88061   |
| 🇳  520 | ⭕️  188560320 | 🕗 127.76067   |
| 🇳  521 | ⭕️  189648168 | 🕗 126.533554  |
| 🇳  522 | ⭕️  190740192 | 🕗 125.83278   |
| 🇳  523 | ⭕️  191836400 | 🕗 127.92514   |
| 🇳  524 | ⭕️  192936800 | 🕗 129.85686   |
| 🇳  525 | ⭕️  194041400 | 🕗 129.12285   |
| 🇳  526 | ⭕️  195150208 | 🕗 132.04358   |
| 🇳  527 | ⭕️  196263232 | 🕗 131.39987   |
| 🇳  528 | ⭕️  197380480 | 🕗 131.87721   |
| 🇳  529 | ⭕️  198501960 | 🕗 131.69095   |
| 🇳  530 | ⭕️  199627680 | 🕗 132.4668    |
| 🇳  531 | ⭕️  200757648 | 🕗 133.51082   |
| 🇳  532 | ⭕️  201891872 | 🕗 133.71538   |
| 🇳  533 | ⭕️  203030360 | 🕗 133.5442    |
| 🇳  534 | ⭕️  204173120 | 🕗 137.30698   |
| 🇳  535 | ⭕️  205320160 | 🕗 136.4574    |
| 🇳  536 | ⭕️  206471488 | 🕗 139.20169   |
| 🇳  537 | ⭕️  207627112 | 🕗 140.56607   |
| 🇳  538 | ⭕️  208787040 | 🕗 139.45547   |
| 🇳  539 | ⭕️  209951280 | 🕗 141.76472   |
| 🇳  540 | ⭕️  211119840 | 🕗 141.54082   |
| 🇳  541 | ⭕️  212292728 | 🕗 143.1617    |
| 🇳  542 | ⭕️  213469952 | 🕗 144.90912   |
| 🇳  543 | ⭕️  214651520 | 🕗 145.20244   |
| 🇳  544 | ⭕️  215837440 | 🕗 147.75807   |
| 🇳  545 | ⭕️  217027720 | 🕗 146.36388   |
| 🇳  546 | ⭕️  218222368 | 🕗 149.63226   |
| 🇳  547 | ⭕️  219421392 | 🕗 149.99312   |
| 🇳  548 | ⭕️  220624800 | 🕗 150.93156   |
| 🇳  549 | ⭕️  221832600 | 🕗 152.1299    |
| 🇳  550 | ⭕️  223044800 | 🕗 155.88113   |
| 🇳  551 | ⭕️  224261408 | 🕗 155.07405   |
| 🇳  552 | ⭕️  225482432 | 🕗 156.76454   |
| 🇳  553 | ⭕️  226707880 | 🕗 156.90509   |
| 🇳  554 | ⭕️  227937760 | 🕗 158.35698   |
| 🇳  555 | ⭕️  229172080 | 🕗 159.16417   |
| 🇳  556 | ⭕️  230410848 | 🕗 158.41768   |
| 🇳  557 | ⭕️  231654072 | 🕗 160.00195   |
| 🇳  558 | ⭕️  232901760 | 🕗 165.00262   |
| 🇳  559 | ⭕️  234153920 | 🕗 163.1451    |
| 🇳  560 | ⭕️  235410560 | 🕗 165.53459   |
| 🇳  561 | ⭕️  236671688 | 🕗 166.07326   |
| 🇳  562 | ⭕️  237937312 | 🕗 167.98341   |
| 🇳  563 | ⭕️  239207440 | 🕗 168.04437   |
| 🇳  564 | ⭕️  240482080 | 🕗 171.848     |
| 🇳  565 | ⭕️  241761240 | 🕗 171.06314   |
| 🇳  566 | ⭕️  243044928 | 🕗 174.6324    |
| 🇳  567 | ⭕️  244333152 | 🕗 172.78879   |
| 🇳  568 | ⭕️  245625920 | 🕗 175.58975   |
| 🇳  569 | ⭕️  246923240 | 🕗 179.42183   |
| 🇳  570 | ⭕️  248225120 | 🕗 179.78815   |
| 🇳  570 | ⭕️  248225120 | 🕗 179.78815   |
| 🇳  571 | ⭕️  249531568 | 🕗 180.31573   |
| 🇳  572 | ⭕️  250842592 | 🕗 182.19945   |
| 🇳  573 | ⭕️  252158200 | 🕗 182.62872   |
| 🇳  574 | ⭕️  253478400 | 🕗 184.66798   |
| 🇳  575 | ⭕️  254803200 | 🕗 187.61467   |
| 🇳  576 | ⭕️  256132608 | 🕗 186.98619   |
| 🇳  577 | ⭕️  257466632 | 🕗 184.87135   |
| 🇳  578 | ⭕️  258805280 | 🕗 189.56717   |
| 🇳  579 | ⭕️  260148560 | 🕗 187.35043   |
| 🇳  580 | ⭕️  261496480 | 🕗 192.12587   |
| 🇳  581 | ⭕️  262849048 | 🕗 193.16795   |
| 🇳  582 | ⭕️  264206272 | 🕗 194.6902    |
| 🇳  583 | ⭕️  265568160 | 🕗 197.04663   |
| 🇳  584 | ⭕️  266934720 | 🕗 194.37743   |
| 🇳  585 | ⭕️  268305960 | 🕗 193.86542   |
| 🇳  586 | ⭕️  269681888 | 🕗 195.97664   |
| 🇳  587 | ⭕️  271062512 | 🕗 197.89943   |
| 🇳  588 | ⭕️  272447840 | 🕗 200.29701   |
| 🇳  589 | ⭕️  273837880 | 🕗 202.6525    |
| 🇳  590 | ⭕️  275232640 | 🕗 200.66794   |
| 🇳  591 | ⭕️  276632128 | 🕗 202.40009   |
| 🇳  592 | ⭕️  278036352 | 🕗 203.74187   |
| 🇳  593 | ⭕️  279445320 | 🕗 207.82635   |
| 🇳  594 | ⭕️  280859040 | 🕗 208.36325   |
| 🇳  595 | ⭕️  282277520 | 🕗 210.08517   |
| 🇳  596 | ⭕️  283700768 | 🕗 212.49861   |
| 🇳  597 | ⭕️  285128792 | 🕗 215.53134   |
| 🇳  598 | ⭕️  286561600 | 🕗 216.51059   |
| 🇳  599 | ⭕️  287999200 | 🕗 220.99907   |
| 🇳  600 | ⭕️  289441600 | 🕗 219.10083   |
| 🇳  601 | ⭕️  290888808 | 🕗 217.03249   |
| 🇳  602 | ⭕️  292340832 | 🕗 221.26782   |
| 🇳  603 | ⭕️  293797680 | 🕗 220.79      |
| 🇳  604 | ⭕️  295259360 | 🕗 227.8595    |
| 🇳  605 | ⭕️  296725880 | 🕗 227.5769    |
| 🇳  606 | ⭕️  298197248 | 🕗 227.61673   |
| 🇳  607 | ⭕️  299673472 | 🕗 230.5792    |
| 🇳  608 | ⭕️  301154560 | 🕗 237.21375   |
| 🇳  609 | ⭕️  302640520 | 🕗 228.3568    |
| 🇳  610 | ⭕️  304131360 | 🕗 233.98697   |
| 🇳  611 | ⭕️  305627088 | 🕗 235.43521   |
| 🇳  612 | ⭕️  307127712 | 🕗 234.63312   |
| 🇳  613 | ⭕️  308633240 | 🕗 235.42027   |
| 🇳  614 | ⭕️  310143680 | 🕗 238.36655   |
| 🇳  615 | ⭕️  311659040 | 🕗 236.21582   |
| 🇳  616 | ⭕️  313179328 | 🕗 244.35      |
| 🇳  617 | ⭕️  314704552 | 🕗 245.1835    |
| 🇳  618 | ⭕️  316234720 | 🕗 248.50839   |
| 🇳  619 | ⭕️  317769840 | 🕗 244.38019   |
| 🇳  620 | ⭕️  319309920 | 🕗 249.75476   |
| 🇳  621 | ⭕️  320854968 | 🕗 251.6605    |
| 🇳  622 | ⭕️  322404992 | 🕗 252.70122   |
| 🇳  623 | ⭕️  323960000 | 🕗 255.81602   |
| 🇳  624 | ⭕️  325520000 | 🕗 254.25148   |
| 🇳  625 | ⭕️  327085000 | 🕗 256.72485   |
| 🇳  626 | ⭕️  328655008 | 🕗 259.71835   |
| 🇳  627 | ⭕️  330230032 | 🕗 259.02118   |
| 🇳  628 | ⭕️  331810080 | 🕗 264.54694   |
| 🇳  629 | ⭕️  333395160 | 🕗 263.34445   |
| 🇳  630 | ⭕️  334985280 | 🕗 266.01642   |
| 🇳  631 | ⭕️  336580448 | 🕗 265.517     |
| 🇳  632 | ⭕️  338180672 | 🕗 270.39685   |
| 🇳  633 | ⭕️  339785960 | 🕗 269.1739    |
| 🇳  634 | ⭕️  341396320 | 🕗 270.71725   |
| 🇳  635 | ⭕️  343011760 | 🕗 275.8484    |
| 🇳  636 | ⭕️  344632288 | 🕗 275.3204    |
| 🇳  637 | ⭕️  346257912 | 🕗 276.99954   |
| 🇳  638 | ⭕️  347888640 | 🕗 281.7454    |
| 🇳  639 | ⭕️  349524480 | 🕗 281.16776   |
| 🇳  640 | ⭕️  351165440 | 🕗 282.38452   |
| 🇳  641 | ⭕️  352811528 | 🕗 281.90735   |
| 🇳  642 | ⭕️  354462752 | 🕗 291.44666   |
| 🇳  643 | ⭕️  356119120 | 🕗 283.36206   |
| 🇳  644 | ⭕️  357780640 | 🕗 294.34402   |
| 🇳  645 | ⭕️  359447320 | 🕗 290.5282    |
| 🇳  646 | ⭕️  361119168 | 🕗 292.79742   |
| 🇳  647 | ⭕️  362796192 | 🕗 294.5445    |
| 🇳  648 | ⭕️  364478400 | 🕗 301.7287    |
| 🇳  649 | ⭕️  366165800 | 🕗 298.8732    |
| 🇳  650 | ⭕️  367858400 | 🕗 296.63193   |
| 🇳  651 | ⭕️  369556208 | 🕗 299.59885   |
| 🇳  652 | ⭕️  371259232 | 🕗 304.9361    |
| 🇳  653 | ⭕️  372967480 | 🕗 305.51074   |
| 🇳  654 | ⭕️  374680960 | 🕗 310.15192   |
| 🇳  655 | ⭕️  376399680 | 🕗 306.63828   |
| 🇳  656 | ⭕️  378123648 | 🕗 316.4287    |
| 🇳  657 | ⭕️  379852872 | 🕗 316.29248   |
| 🇳  658 | ⭕️  381587360 | 🕗 318.9253    |
| 🇳  659 | ⭕️  383327120 | 🕗 317.72104   |
| 🇳  660 | ⭕️  385072160 | 🕗 324.91394   |
| 🇳  661 | ⭕️  386822488 | 🕗 320.90378   |
| 🇳  662 | ⭕️  388578112 | 🕗 332.783     |
| 🇳  663 | ⭕️  390339040 | 🕗 326.09805   |
| 🇳  664 | ⭕️  392105280 | 🕗 325.52686   |
| 🇳  665 | ⭕️  393876840 | 🕗 332.3045    |
| 🇳  666 | ⭕️  395653728 | 🕗 332.49396   |
| 🇳  667 | ⭕️  397435952 | 🕗 333.56726   |
| 🇳  668 | ⭕️  399223520 | 🕗 333.41656   |
| 🇳  669 | ⭕️  401016440 | 🕗 335.96536   |
| 🇳  670 | ⭕️  402814720 | 🕗 352.47244   |
| 🇳  671 | ⭕️  404618368 | 🕗 342.31882   |
| 🇳  672 | ⭕️  406427392 | 🕗 349.36813   |
| 🇳  673 | ⭕️  408241800 | 🕗 343.68176   |
| 🇳  674 | ⭕️  410061600 | 🕗 347.03033   |
| 🇳  675 | ⭕️  411886800 | 🕗 343.7539    |
| 🇳  676 | ⭕️  413717408 | 🕗 351.1874    |
| 🇳  677 | ⭕️  415553432 | 🕗 350.2762    |
| 🇳  678 | ⭕️  417394880 | 🕗 352.42383   |
| 🇳  679 | ⭕️  419241760 | 🕗 376.27448   |
| 🇳  680 | ⭕️  421094080 | 🕗 375.88297   |
| 🇳  681 | ⭕️  422951848 | 🕗 376.91907   |
| 🇳  682 | ⭕️  424815072 | 🕗 381.3379    |
| 🇳  683 | ⭕️  426683760 | 🕗 373.8281    |
| 🇳  684 | ⭕️  428557920 | 🕗 382.82214   |
| 🇳  685 | ⭕️  430437560 | 🕗 382.72775   |
| 🇳  686 | ⭕️  432322688 | 🕗 374.317     |
| 🇳  687 | ⭕️  434213312 | 🕗 384.2565    |
| 🇳  688 | ⭕️  436109440 | 🕗 381.04147   |
| 🇳  689 | ⭕️  438011080 | 🕗 388.53992   |
| 🇳  690 | ⭕️  439918240 | 🕗 389.09537   |
| 🇳  691 | ⭕️  441830928 | 🕗 389.1716    |
| 🇳  692 | ⭕️  443749152 | 🕗 400.79697   |
| 🇳  693 | ⭕️  445672920 | 🕗 405.38696   |
| 🇳  694 | ⭕️  447602240 | 🕗 408.5462    |
| 🇳  695 | ⭕️  449537120 | 🕗 404.8601    |
| 🇳  696 | ⭕️  451477568 | 🕗 410.6329    |
| 🇳  697 | ⭕️  453423592 | 🕗 414.41913   |
| 🇳  698 | ⭕️  455375200 | 🕗 419.08527   |
| 🇳  699 | ⭕️  457332400 | 🕗 415.31454   |
| 🇳  700 | ⭕️  459295200 | 🕗 422.31543   |
| 🇳  701 | ⭕️  461263608 | 🕗 409.09766   |
| 🇳  702 | ⭕️  463237632 | 🕗 420.29843   |
| 🇳  703 | ⭕️  465217280 | 🕗 426.2619    |
| 🇳  704 | ⭕️  467202560 | 🕗 439.5127    |
| 🇳  705 | ⭕️  469193480 | 🕗 442.2176    |
| 🇳  706 | ⭕️  471190048 | 🕗 448.0588    |
| 🇳  707 | ⭕️  473192272 | 🕗 445.55316   |
| 🇳  708 | ⭕️  475200160 | 🕗 440.87      |
| 🇳  709 | ⭕️  477213720 | 🕗 440.41907   |
| 🇳  710 | ⭕️  479232960 | 🕗 443.05048   |
| 🇳  711 | ⭕️  481257888 | 🕗 455.3008    |
| 🇳  712 | ⭕️  483288512 | 🕗 442.98337   |
| 🇳  713 | ⭕️  485324840 | 🕗 452.03366   |
| 🇳  714 | ⭕️  487366880 | 🕗 448.5766    |
| 🇳  715 | ⭕️  489414640 | 🕗 447.6833    |
| 🇳  716 | ⭕️  491468128 | 🕗 460.47015   |
| 🇳  717 | ⭕️  493527352 | 🕗 454.91162   |
| 🇳  718 | ⭕️  495592320 | 🕗 468.20422   |
| 🇳  719 | ⭕️  497663040 | 🕗 467.21396   |
| 🇳  720 | ⭕️  499739520 | 🕗 463.67163   |
| 🇳  721 | ⭕️  501821768 | 🕗 469.57587   |
| 🇳  722 | ⭕️  503909792 | 🕗 474.38907   |
| 🇳  723 | ⭕️  506003600 | 🕗 482.1608    |
| 🇳  724 | ⭕️  508103200 | 🕗 478.16135   |
| 🇳  725 | ⭕️  510208600 | 🕗 477.60892   |
| 🇳  726 | ⭕️  512319808 | 🕗 487.3817    |
| 🇳  727 | ⭕️  514436832 | 🕗 494.51654   |
| 🇳  728 | ⭕️  516559680 | 🕗 487.7568    |
| 🇳  729 | ⭕️  518688360 | 🕗 485.13315   |
| 🇳  730 | ⭕️  520822880 | 🕗 498.55057   |
| 🇳  731 | ⭕️  522963248 | 🕗 498.00424   |
| 🇳  732 | ⭕️  525109472 | 🕗 502.94525   |
| 🇳  733 | ⭕️  527261560 | 🕗 502.40942   |
| 🇳  734 | ⭕️  529419520 | 🕗 511.55048   |
| 🇳  735 | ⭕️  531583360 | 🕗 512.0618    |
| 🇳  736 | ⭕️  533753088 | 🕗 519.5763    |
| 🇳  737 | ⭕️  535928712 | 🕗 513.1648    |
| 🇳  738 | ⭕️  538110240 | 🕗 522.27026   |
| 🇳  739 | ⭕️  540297680 | 🕗 525.7643    |
| 🇳  740 | ⭕️  542491040 | 🕗 522.33984   |
| 🇳  741 | ⭕️  544690328 | 🕗 530.2288    |
| 🇳  742 | ⭕️  546895552 | 🕗 534.44507   |
| 🇳  743 | ⭕️  549106720 | 🕗 537.5375    |
| 🇳  744 | ⭕️  551323840 | 🕗 542.68414   |
| 🇳  745 | ⭕️  553546920 | 🕗 549.7611    |
| 🇳  746 | ⭕️  555775968 | 🕗 559.8681    |
| 🇳  747 | ⭕️  558010992 | 🕗 557.90625   |
| 🇳  748 | ⭕️  560252000 | 🕗 563.9526    |
| 🇳  749 | ⭕️  562499000 | 🕗 567.5977    |
| 🇳  750 | ⭕️  564752000 | 🕗 570.25946   |
| 🇳  751 | ⭕️  567011008 | 🕗 580.4319    |
| 🇳  752 | ⭕️  569276032 | 🕗 564.4526    |
| 🇳  753 | ⭕️  571547080 | 🕗 584.68445   |
| 🇳  754 | ⭕️  573824160 | 🕗 580.8851    |
| 🇳  755 | ⭕️  576107280 | 🕗 582.40955   |
| 🇳  756 | ⭕️  578396448 | 🕗 593.1553    |
| 🇳  757 | ⭕️  580691672 | 🕗 596.1205    |
| 🇳  758 | ⭕️  582992960 | 🕗 592.6622    |
| 🇳  759 | ⭕️  585300320 | 🕗 607.0393    |
| 🇳  760 | ⭕️  587613760 | 🕗 609.11786   |
| 🇳  761 | ⭕️  589933288 | 🕗 613.04626   |
| 🇳  762 | ⭕️  592258912 | 🕗 621.4857    |
| 🇳  763 | ⭕️  594590640 | 🕗 615.5292    |
| 🇳  764 | ⭕️  596928480 | 🕗 626.5498    |
| 🇳  765 | ⭕️  599272440 | 🕗 627.74664   |
| 🇳  766 | ⭕️  601622528 | 🕗 635.35016   |
| 🇳  767 | ⭕️  603978752 | 🕗 632.1883    |
| 🇳  768 | ⭕️  606341120 | 🕗 638.151     |
| 🇳  769 | ⭕️  608709640 | 🕗 639.95465   |
| 🇳  770 | ⭕️  611084320 | 🕗 639.476     |
| 🇳  771 | ⭕️  613465168 | 🕗 640.6278    |
| 🇳  772 | ⭕️  615852192 | 🕗 654.24304   |
| 🇳  773 | ⭕️  618245400 | 🕗 653.71533   |
| 🇳  774 | ⭕️  620644800 | 🕗 672.1014    |
| 🇳  775 | ⭕️  623050400 | 🕗 665.65643   |
| 🇳  776 | ⭕️  625462208 | 🕗 669.41693   |
| 🇳  777 | ⭕️  627880232 | 🕗 676.18365   |
| 🇳  778 | ⭕️  630304480 | 🕗 677.1852    |
| 🇳  779 | ⭕️  632734960 | 🕗 691.56335   |
| 🇳  780 | ⭕️  635171680 | 🕗 697.5961    |
| 🇳  781 | ⭕️  637614648 | 🕗 694.1226    |
| 🇳  782 | ⭕️  640063872 | 🕗 698.0464    |
| 🇳  783 | ⭕️  642519360 | 🕗 692.53595   |
| 🇳  784 | ⭕️  644981120 | 🕗 713.0903    |
| 🇳  785 | ⭕️  647449160 | 🕗 707.44183   |
| 🇳  786 | ⭕️  649923488 | 🕗 711.2089    |
| 🇳  787 | ⭕️  652404112 | 🕗 707.148     |
| 🇳  788 | ⭕️  654891040 | 🕗 716.5655    |
| 🇳  789 | ⭕️  657384280 | 🕗 717.7836    |
| 🇳  790 | ⭕️  659883840 | 🕗 727.2548    |
| 🇳  791 | ⭕️  662389728 | 🕗 711.4368    |
| 🇳  792 | ⭕️  664901952 | 🕗 749.35803   |
| 🇳  793 | ⭕️  667420520 | 🕗 740.723     |
| 🇳  794 | ⭕️  669945440 | 🕗 739.453     |
| 🇳  795 | ⭕️  672476720 | 🕗 739.6702    |
| 🇳  796 | ⭕️  675014368 | 🕗 754.49194   |
| 🇳  797 | ⭕️  677558392 | 🕗 757.54785   |
| 🇳  798 | ⭕️  680108800 | 🕗 753.9655    |
| 🇳  799 | ⭕️  682665600 | 🕗 767.94226   |
| 🇳  800 | ⭕️  685228800 | 🕗 776.0574    |
| 🇳  801 | ⭕️  687798408 | 🕗 767.04785   |
| 🇳  802 | ⭕️  690374432 | 🕗 795.5465    |
| 🇳  803 | ⭕️  692956880 | 🕗 776.9618    |
| 🇳  804 | ⭕️  695545760 | 🕗 800.3405    |
| 🇳  805 | ⭕️  698141080 | 🕗 795.2684    |
| 🇳  806 | ⭕️  700742848 | 🕗 806.8855    |
| 🇳  807 | ⭕️  703351072 | 🕗 797.0185    |
| 🇳  808 | ⭕️  705965760 | 🕗 815.8328    |
| 🇳  809 | ⭕️  708586920 | 🕗 821.943     |
| 🇳  810 | ⭕️  711214560 | 🕗 817.94806   |
| 🇳  811 | ⭕️  713848688 | 🕗 831.2155    |
| 🇳  812 | ⭕️  716489312 | 🕗 832.433     |
| 🇳  813 | ⭕️  719136440 | 🕗 836.33704   |
| 🇳  814 | ⭕️  721790080 | 🕗 837.7355    |
| 🇳  815 | ⭕️  724450240 | 🕗 854.2903    |
| 🇳  816 | ⭕️  727116928 | 🕗 832.3699    |
| 🇳  817 | ⭕️  729790152 | 🕗 862.3605    |
| 🇳  818 | ⭕️  732469920 | 🕗 864.9721    |
| 🇳  819 | ⭕️  735156240 | 🕗 887.0709    |
| 🇳  820 | ⭕️  737849120 | 🕗 896.02313   |
| 🇳  821 | ⭕️  740548568 | 🕗 903.262     |
| 🇳  822 | ⭕️  743254592 | 🕗 918.7063    |
| 🇳  823 | ⭕️  745967200 | 🕗 905.7513    |
| 🇳  824 | ⭕️  748686400 | 🕗 878.9788    |
| 🇳  825 | ⭕️  751412200 | 🕗 901.5448    |
| 🇳  826 | ⭕️  754144608 | 🕗 916.91876   |
| 🇳  827 | ⭕️  756883632 | 🕗 932.1164    |
| 🇳  828 | ⭕️  759629280 | 🕗 933.8062    |
| 🇳  829 | ⭕️  762381560 | 🕗 927.74115   |
| 🇳  830 | ⭕️  765140480 | 🕗 947.50653   |
| 🇳  831 | ⭕️  767906048 | 🕗 959.41345   |
| 🇳  832 | ⭕️  770678272 | 🕗 933.34033   |
| 🇳  833 | ⭕️  773457160 | 🕗 956.98254   |
| 🇳  834 | ⭕️  776242720 | 🕗 949.05835   |
| 🇳  835 | ⭕️  779034960 | 🕗 943.38086   |
| 🇳  836 | ⭕️  781833888 | 🕗 973.9386    |
| 🇳  837 | ⭕️  784639512 | 🕗 989.7338    |
| 🇳  838 | ⭕️  787451840 | 🕗 961.6285    |
| 🇳  839 | ⭕️  790270880 | 🕗 988.29944   |
| 🇳  840 | ⭕️  793096640 | 🕗 990.651     |
| 🇳  841 | ⭕️  795929128 | 🕗 996.884     |
| 🇳  842 | ⭕️  798768352 | 🕗 1009.86005  |
| 🇳  843 | ⭕️  801614320 | 🕗 1008.44684  |
| 🇳  844 | ⭕️  804467040 | 🕗 1023.4822   |
| 🇳  845 | ⭕️  807326520 | 🕗 1036.7554   |
| 🇳  846 | ⭕️  810192768 | 🕗 1048.3129   |
| 🇳  847 | ⭕️  813065792 | 🕗 1039.1687   |
| 🇳  848 | ⭕️  815945600 | 🕗 1032.5049   |
| 🇳  849 | ⭕️  818832200 | 🕗 1034.432    |
| 🇳  850 | ⭕️  821725600 | 🕗 1070.8883   |
| 🇳  851 | ⭕️  824625808 | 🕗 1044.2975   |
| 🇳  852 | ⭕️  827532832 | 🕗 1081.0212   |
| 🇳  853 | ⭕️  830446680 | 🕗 1062.7942   |
| 🇳  854 | ⭕️  833367360 | 🕗 1081.022    |
| 🇳  855 | ⭕️  836294880 | 🕗 1103.6678   |
| 🇳  856 | ⭕️  839229248 | 🕗 1094.1357   |
| 🇳  857 | ⭕️  842170472 | 🕗 1088.5359   |
| 🇳  858 | ⭕️  845118560 | 🕗 1102.7394   |
| 🇳  859 | ⭕️  848073520 | 🕗 1086.4933   |
| 🇳  860 | ⭕️  851035360 | 🕗 1098.6343   |
| 🇳  861 | ⭕️  854004088 | 🕗 1099.377    |
| 🇳  862 | ⭕️  856979712 | 🕗 1126.6124   |
| 🇳  863 | ⭕️  859962240 | 🕗 1141.824    |
| 🇳  864 | ⭕️  862951680 | 🕗 1125.0447   |
| 🇳  865 | ⭕️  865948040 | 🕗 1123.4713   |
| 🇳  866 | ⭕️  868951328 | 🕗 1124.2178   |
| 🇳  867 | ⭕️  871961552 | 🕗 1150.2515   |
| 🇳  868 | ⭕️  874978720 | 🕗 1186.849    |
| 🇳  869 | ⭕️  878002840 | 🕗 1215.1089   |
| 🇳  870 | ⭕️  881033920 | 🕗 1184.2728   |
| 🇳  871 | ⭕️  884071968 | 🕗 1176.9257   |
| 🇳  872 | ⭕️  887116992 | 🕗 1191.0714   |
| 🇳  873 | ⭕️  890169000 | 🕗 1151.8418   |
| 🇳  874 | ⭕️  893228000 | 🕗 1219.4382   |
| 🇳  875 | ⭕️  896294000 | 🕗 1230.8821   |
| 🇳  876 | ⭕️  899367008 | 🕗 1198.1079   |
| 🇳  877 | ⭕️  902447032 | 🕗 1204.568    |
| 🇳  878 | ⭕️  905534080 | 🕗 1226.0151   |
| 🇳  879 | ⭕️  908628160 | 🕗 1235.4539   |
| 🇳  880 | ⭕️  911729280 | 🕗 1247.8569   |
| 🇳  881 | ⭕️  914837448 | 🕗 1249.0696   |
| 🇳  882 | ⭕️  917952672 | 🕗 1253.1187   |
| 🇳  883 | ⭕️  921074960 | 🕗 1241.7137   |
| 🇳  884 | ⭕️  924204320 | 🕗 1281.2852   |
| 🇳  885 | ⭕️  927340760 | 🕗 1242.4163   |
| 🇳  886 | ⭕️  930484288 | 🕗 1336.2025   |
| 🇳  887 | ⭕️  933634912 | 🕗 1272.7885   |
| 🇳  888 | ⭕️  936792640 | 🕗 1312.6239   |
| 🇳  889 | ⭕️  939957480 | 🕗 1268.2065   |
| 🇳  890 | ⭕️  943129440 | 🕗 1286.0504   |
| 🇳  891 | ⭕️  946308528 | 🕗 1374.1068   |
| 🇳  892 | ⭕️  949494752 | 🕗 1384.2745   |
| 🇳  893 | ⭕️  952688120 | 🕗 1309.6842   |
| 🇳  894 | ⭕️  955888640 | 🕗 1349.9381   |
| 🇳  895 | ⭕️  959096320 | 🕗 1364.9525   |
| 🇳  896 | ⭕️  962311168 | 🕗 1400.4164   |
| 🇳  897 | ⭕️  965533192 | 🕗 1407.1372   |
| 🇳  898 | ⭕️  968762400 | 🕗 1401.4249   |
| 🇳  899 | ⭕️  971998800 | 🕗 1258.9851   |
| 🇳  900 | ⭕️  975242400 | 🕗 1134.092    |
| 🇳  950 | ⭕️ 1146779200 | 🕗 1341.12323  |
| 🇳 1000 | ⭕️ 1337336000 | 🕗 1400.8303   |
| 🇳 1050 | ⭕️ 1547912800 | 🕗 1655.05322  |
| 🇳 1100 | ⭕️ 1779509600 | 🕗 1981.9500   |
| 🇳 1150 | ⭕️ 2033126400 | 🕗 2340.1879   |
| 🇳 1200 | ⭕️ 2309763200 | 🕗 2820.4538   |
| 🇳 1250 | ⭕️ 2610420000 | 🕗 3187.06396  |
| 🇳 1300 | ⭕️ 2936096800 | 🕗 4148.67041  |
| 🇳 1350 | ⭕️ 3287793600 | 🕗 4824.60888  |
| 🇳 1400 | ⭕️ 3666510400 | 🕗 5747.187983 |
| 🇳 1450 | ⭕️ 4073247200 | 🕗 6748.131340 |
| 🇳 1500 | ⭕️ 4509004000 | 🕗 7278.642052 |
| 🇳 1550 | ⭕️ 4974780800 | 🕗 7262.342778 |
| 🇳 1600 | ⭕️ 5471577600 | 🕗 8762.348632 |
| 🇳 1650 | ⭕️ 6000394400 | 🕗 9493.481445 |
| 🇳 1700 | ⭕️ 6562231200 | 🕗 10870.05661 |
| 🇳 1750 | ⭕️ 7158088000 | 🕗 12514.00583 |
| 🇳 1800 | ⭕️ 7788964800 | 🕗 15092.51664 |
| 🇳 1825 | ⭕️ 8117848200 | 🕗 15321.19042 |
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
