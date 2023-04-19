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
| 🇳    1 | ⭕️          8 | 🕗 SOLVE: 0.0000121670 | 📌 HamCycle | 🕗 CERTIFY: 0.0000019580
| 🇳    2 | ⭕️         32 | 🕗 SOLVE: 0.0000336670 | 📌 HamCycle | 🕗 CERTIFY: 0.0000043330
| 🇳    3 | ⭕️         80 | 🕗 SOLVE: 0.0000556250 | 📌 HamCycle | 🕗 CERTIFY: 0.0000057910
| 🇳    4 | ⭕️        160 | 🕗 SOLVE: 0.0000860830 | 📌 HamCycle | 🕗 CERTIFY: 0.0000108340
| 🇳    5 | ⭕️        280 | 🕗 SOLVE: 0.0001728340 | 📌 HamCycle | 🕗 CERTIFY: 0.0000222500
| 🇳    6 | ⭕️        448 | 🕗 SOLVE: 0.0002152500 | 📌 HamCycle | 🕗 CERTIFY: 0.0000235840
| 🇳    7 | ⭕️        672 | 🕗 SOLVE: 0.0002383330 | 📌 HamCycle | 🕗 CERTIFY: 0.0000411250
| 🇳    8 | ⭕️        960 | 🕗 SOLVE: 0.0002807920 | 📌 HamCycle | 🕗 CERTIFY: 0.0000819580
| 🇳    9 | ⭕️       1320 | 🕗 SOLVE: 0.0003124580 | 📌 HamCycle | 🕗 CERTIFY: 0.0000835000
| 🇳   10 | ⭕️       1760 | 🕗 SOLVE: 0.0003534590 | 📌 HamCycle | 🕗 CERTIFY: 0.0000758330
| 🇳   11 | ⭕️       2288 | 🕗 SOLVE: 0.0003796250 | 📌 HamCycle | 🕗 CERTIFY: 0.0001184160
| 🇳   12 | ⭕️       2912 | 🕗 SOLVE: 0.0004421670 | 📌 HamCycle | 🕗 CERTIFY: 0.0001262080
| 🇳   13 | ⭕️       3640 | 🕗 SOLVE: 0.0004606250 | 📌 HamCycle | 🕗 CERTIFY: 0.0001967920
| 🇳   14 | ⭕️       4480 | 🕗 SOLVE: 0.0005024590 | 📌 HamCycle | 🕗 CERTIFY: 0.0002020000
| 🇳   15 | ⭕️       5440 | 🕗 SOLVE: 0.0005727920 | 📌 HamCycle | 🕗 CERTIFY: 0.0002014170
| 🇳   16 | ⭕️       6528 | 🕗 SOLVE: 0.0006797920 | 📌 HamCycle | 🕗 CERTIFY: 0.0002303750
| 🇳   17 | ⭕️       7752 | 🕗 SOLVE: 0.0006532500 | 📌 HamCycle | 🕗 CERTIFY: 0.0003662910
| 🇳   18 | ⭕️       9120 | 🕗 SOLVE: 0.0006986670 | 📌 HamCycle | 🕗 CERTIFY: 0.0003873340
| 🇳   19 | ⭕️      10640 | 🕗 SOLVE: 0.0008152920 | 📌 HamCycle | 🕗 CERTIFY: 0.0003732080
| 🇳   20 | ⭕️      12320 | 🕗 SOLVE: 0.0008502500 | 📌 HamCycle | 🕗 CERTIFY: 0.0004233750
| 🇳   21 | ⭕️      14168 | 🕗 SOLVE: 0.0009908330 | 📌 HamCycle | 🕗 CERTIFY: 0.0005137920
| 🇳   22 | ⭕️      16192 | 🕗 SOLVE: 0.0009650000 | 📌 HamCycle | 🕗 CERTIFY: 0.0006795420
| 🇳   23 | ⭕️      18400 | 🕗 SOLVE: 0.0011169170 | 📌 HamCycle | 🕗 CERTIFY: 0.0007106660
| 🇳   24 | ⭕️      20800 | 🕗 SOLVE: 0.0011144590 | 📌 HamCycle | 🕗 CERTIFY: 0.0007952920
| 🇳   25 | ⭕️      23400 | 🕗 SOLVE: 0.0011674589 | 📌 HamCycle | 🕗 CERTIFY: 0.0007642920
| 🇳   26 | ⭕️      26208 | 🕗 SOLVE: 0.0013020000 | 📌 HamCycle | 🕗 CERTIFY: 0.0008409580
| 🇳   27 | ⭕️      29232 | 🕗 SOLVE: 0.0013477500 | 📌 HamCycle | 🕗 CERTIFY: 0.0013523750
| 🇳   28 | ⭕️      32480 | 🕗 SOLVE: 0.0014275840 | 📌 HamCycle | 🕗 CERTIFY: 0.0014503750
| 🇳   29 | ⭕️      35960 | 🕗 SOLVE: 0.0014958750 | 📌 HamCycle | 🕗 CERTIFY: 0.0014186660
| 🇳   30 | ⭕️      39680 | 🕗 SOLVE: 0.0014919169 | 📌 HamCycle | 🕗 CERTIFY: 0.0015128331
| 🇳   31 | ⭕️      43648 | 🕗 SOLVE: 0.0015422500 | 📌 HamCycle | 🕗 CERTIFY: 0.0015926660
| 🇳   32 | ⭕️      47872 | 🕗 SOLVE: 0.0016930420 | 📌 HamCycle | 🕗 CERTIFY: 0.0016594169
| 🇳   33 | ⭕️      52360 | 🕗 SOLVE: 0.0017866250 | 📌 HamCycle | 🕗 CERTIFY: 0.0018488750
| 🇳   34 | ⭕️      57120 | 🕗 SOLVE: 0.0018867080 | 📌 HamCycle | 🕗 CERTIFY: 0.0019238750
| 🇳   35 | ⭕️      62160 | 🕗 SOLVE: 0.0020575831 | 📌 HamCycle | 🕗 CERTIFY: 0.0027387501
| 🇳   36 | ⭕️      67488 | 🕗 SOLVE: 0.0020389170 | 📌 HamCycle | 🕗 CERTIFY: 0.0028811251
| 🇳   37 | ⭕️      73112 | 🕗 SOLVE: 0.0022446250 | 📌 HamCycle | 🕗 CERTIFY: 0.0029874579
| 🇳   38 | ⭕️      79040 | 🕗 SOLVE: 0.0022678750 | 📌 HamCycle | 🕗 CERTIFY: 0.0032792501
| 🇳   39 | ⭕️      85280 | 🕗 SOLVE: 0.0024716251 | 📌 HamCycle | 🕗 CERTIFY: 0.0034453331
| 🇳   40 | ⭕️      91840 | 🕗 SOLVE: 0.0025920840 | 📌 HamCycle | 🕗 CERTIFY: 0.0032706249
| 🇳   41 | ⭕️      98728 | 🕗 SOLVE: 0.0026535830 | 📌 HamCycle | 🕗 CERTIFY: 0.0037181671
| 🇳   42 | ⭕️     105952 | 🕗 SOLVE: 0.0027707920 | 📌 HamCycle | 🕗 CERTIFY: 0.0041115419
| 🇳   43 | ⭕️     113520 | 🕗 SOLVE: 0.0029962501 | 📌 HamCycle | 🕗 CERTIFY: 0.0041267499
| 🇳   44 | ⭕️     121440 | 🕗 SOLVE: 0.0029682911 | 📌 HamCycle | 🕗 CERTIFY: 0.0056015421
| 🇳   45 | ⭕️     129720 | 🕗 SOLVE: 0.0033047090 | 📌 HamCycle | 🕗 CERTIFY: 0.0055404170
| 🇳   46 | ⭕️     138368 | 🕗 SOLVE: 0.0034581251 | 📌 HamCycle | 🕗 CERTIFY: 0.0058304998
| 🇳   47 | ⭕️     147392 | 🕗 SOLVE: 0.0036917501 | 📌 HamCycle | 🕗 CERTIFY: 0.0059546251
| 🇳   48 | ⭕️     156800 | 🕗 SOLVE: 0.0038126251 | 📌 HamCycle | 🕗 CERTIFY: 0.0066784588
| 🇳   49 | ⭕️     166600 | 🕗 SOLVE: 0.0039234171 | 📌 HamCycle | 🕗 CERTIFY: 0.0062501249
| 🇳   50 | ⭕️     176800 | 🕗 SOLVE: 0.0039170841 | 📌 HamCycle | 🕗 CERTIFY: 0.0071218330
| 🇳   51 | ⭕️     187408 | 🕗 SOLVE: 0.0042002499 | 📌 HamCycle | 🕗 CERTIFY: 0.0066629169
| 🇳   52 | ⭕️     198432 | 🕗 SOLVE: 0.0043289168 | 📌 HamCycle | 🕗 CERTIFY: 0.0070405412
| 🇳   53 | ⭕️     209880 | 🕗 SOLVE: 0.0043575000 | 📌 HamCycle | 🕗 CERTIFY: 0.0074788751
| 🇳   54 | ⭕️     221760 | 🕗 SOLVE: 0.0045866668 | 📌 HamCycle | 🕗 CERTIFY: 0.0077070422
| 🇳   55 | ⭕️     234080 | 🕗 SOLVE: 0.0048543750 | 📌 HamCycle | 🕗 CERTIFY: 0.0112062497
| 🇳   56 | ⭕️     246848 | 🕗 SOLVE: 0.0049461662 | 📌 HamCycle | 🕗 CERTIFY: 0.0111648748
| 🇳   57 | ⭕️     260072 | 🕗 SOLVE: 0.0054035839 | 📌 HamCycle | 🕗 CERTIFY: 0.0112190843
| 🇳   58 | ⭕️     273760 | 🕗 SOLVE: 0.0055786660 | 📌 HamCycle | 🕗 CERTIFY: 0.0122139174
| 🇳   59 | ⭕️     287920 | 🕗 SOLVE: 0.0055797920 | 📌 HamCycle | 🕗 CERTIFY: 0.0124150002
| 🇳   60 | ⭕️     302560 | 🕗 SOLVE: 0.0060125832 | 📌 HamCycle | 🕗 CERTIFY: 0.0122132087
| 🇳   61 | ⭕️     317688 | 🕗 SOLVE: 0.0062666670 | 📌 HamCycle | 🕗 CERTIFY: 0.0130162919
| 🇳   62 | ⭕️     333312 | 🕗 SOLVE: 0.0064859171 | 📌 HamCycle | 🕗 CERTIFY: 0.0128933750
| 🇳   63 | ⭕️     349440 | 🕗 SOLVE: 0.0067297500 | 📌 HamCycle | 🕗 CERTIFY: 0.0138282916
| 🇳   64 | ⭕️     366080 | 🕗 SOLVE: 0.0070810001 | 📌 HamCycle | 🕗 CERTIFY: 0.0137839159
| 🇳   65 | ⭕️     383240 | 🕗 SOLVE: 0.0073842080 | 📌 HamCycle | 🕗 CERTIFY: 0.0140083749
| 🇳   66 | ⭕️     400928 | 🕗 SOLVE: 0.0075188749 | 📌 HamCycle | 🕗 CERTIFY: 0.0144224167
| 🇳   67 | ⭕️     419152 | 🕗 SOLVE: 0.0080193747 | 📌 HamCycle | 🕗 CERTIFY: 0.0148717090
| 🇳   68 | ⭕️     437920 | 🕗 SOLVE: 0.0091587501 | 📌 HamCycle | 🕗 CERTIFY: 0.0164495837
| 🇳   69 | ⭕️     457240 | 🕗 SOLVE: 0.0094685405 | 📌 HamCycle | 🕗 CERTIFY: 0.0165821668
| 🇳   70 | ⭕️     477120 | 🕗 SOLVE: 0.0086249169 | 📌 HamCycle | 🕗 CERTIFY: 0.0228634588
| 🇳   71 | ⭕️     497568 | 🕗 SOLVE: 0.0091080833 | 📌 HamCycle | 🕗 CERTIFY: 0.0228017922
| 🇳   72 | ⭕️     518592 | 🕗 SOLVE: 0.0094844159 | 📌 HamCycle | 🕗 CERTIFY: 0.0231352076
| 🇳   73 | ⭕️     540200 | 🕗 SOLVE: 0.0098849582 | 📌 HamCycle | 🕗 CERTIFY: 0.0241808761
| 🇳   74 | ⭕️     562400 | 🕗 SOLVE: 0.0103974165 | 📌 HamCycle | 🕗 CERTIFY: 0.0242182072
| 🇳   75 | ⭕️     585200 | 🕗 SOLVE: 0.0107163750 | 📌 HamCycle | 🕗 CERTIFY: 0.0248430427
| 🇳   76 | ⭕️     608608 | 🕗 SOLVE: 0.0108824996 | 📌 HamCycle | 🕗 CERTIFY: 0.0250478312
| 🇳   77 | ⭕️     632632 | 🕗 SOLVE: 0.0113353329 | 📌 HamCycle | 🕗 CERTIFY: 0.0256342497
| 🇳   78 | ⭕️     657280 | 🕗 SOLVE: 0.0114379581 | 📌 HamCycle | 🕗 CERTIFY: 0.0263907928
| 🇳   79 | ⭕️     682560 | 🕗 SOLVE: 0.0119424583 | 📌 HamCycle | 🕗 CERTIFY: 0.0263701677
| 🇳   80 | ⭕️     708480 | 🕗 SOLVE: 0.0127124172 | 📌 HamCycle | 🕗 CERTIFY: 0.0273367502
| 🇳   81 | ⭕️     735048 | 🕗 SOLVE: 0.0130524170 | 📌 HamCycle | 🕗 CERTIFY: 0.0275577921
| 🇳   82 | ⭕️     762272 | 🕗 SOLVE: 0.0137428343 | 📌 HamCycle | 🕗 CERTIFY: 0.0284026675
| 🇳   83 | ⭕️     790160 | 🕗 SOLVE: 0.0139735406 | 📌 HamCycle | 🕗 CERTIFY: 0.0290805846
| 🇳   84 | ⭕️     818720 | 🕗 SOLVE: 0.0146117499 | 📌 HamCycle | 🕗 CERTIFY: 0.0326527506
| 🇳   85 | ⭕️     847960 | 🕗 SOLVE: 0.0145710828 | 📌 HamCycle | 🕗 CERTIFY: 0.0307015423
| 🇳   86 | ⭕️     877888 | 🕗 SOLVE: 0.0154153341 | 📌 HamCycle | 🕗 CERTIFY: 0.0318818763
| 🇳   87 | ⭕️     908512 | 🕗 SOLVE: 0.0159500837 | 📌 HamCycle | 🕗 CERTIFY: 0.0328654572
| 🇳   88 | ⭕️     939840 | 🕗 SOLVE: 0.0162739586 | 📌 HamCycle | 🕗 CERTIFY: 0.0468763746
| 🇳   89 | ⭕️     971880 | 🕗 SOLVE: 0.0171253756 | 📌 HamCycle | 🕗 CERTIFY: 0.0467696674
| 🇳   90 | ⭕️    1004640 | 🕗 SOLVE: 0.0177017916 | 📌 HamCycle | 🕗 CERTIFY: 0.0480717085
| 🇳   91 | ⭕️    1038128 | 🕗 SOLVE: 0.0178309605 | 📌 HamCycle | 🕗 CERTIFY: 0.0475981683
| 🇳   92 | ⭕️    1072352 | 🕗 SOLVE: 0.0184610840 | 📌 HamCycle | 🕗 CERTIFY: 0.0485990010
| 🇳   93 | ⭕️    1107320 | 🕗 SOLVE: 0.0192627925 | 📌 HamCycle | 🕗 CERTIFY: 0.0489931256
| 🇳   94 | ⭕️    1143040 | 🕗 SOLVE: 0.0196390003 | 📌 HamCycle | 🕗 CERTIFY: 0.0505070388
| 🇳   95 | ⭕️    1179520 | 🕗 SOLVE: 0.0203737915 | 📌 HamCycle | 🕗 CERTIFY: 0.0506451689
| 🇳   96 | ⭕️    1216768 | 🕗 SOLVE: 0.0208999999 | 📌 HamCycle | 🕗 CERTIFY: 0.0515392087
| 🇳   97 | ⭕️    1254792 | 🕗 SOLVE: 0.0215984602 | 📌 HamCycle | 🕗 CERTIFY: 0.0527715832
| 🇳   98 | ⭕️    1293600 | 🕗 SOLVE: 0.0222403761 | 📌 HamCycle | 🕗 CERTIFY: 0.0535826646
| 🇳   99 | ⭕️    1333200 | 🕗 SOLVE: 0.0230612494 | 📌 HamCycle | 🕗 CERTIFY: 0.0540756248
| 🇳  100 | ⭕️    1373600 | 🕗 SOLVE: 0.0237264577 | 📌 HamCycle | 🕗 CERTIFY: 0.0555070415
| 🇳  101 | ⭕️    1414808 | 🕗 SOLVE: 0.0241850000 | 📌 HamCycle | 🕗 CERTIFY: 0.0563396625
| 🇳  102 | ⭕️    1456832 | 🕗 SOLVE: 0.0250747073 | 📌 HamCycle | 🕗 CERTIFY: 0.0576850846
| 🇳  103 | ⭕️    1499680 | 🕗 SOLVE: 0.0259533338 | 📌 HamCycle | 🕗 CERTIFY: 0.0588395856
| 🇳  104 | ⭕️    1543360 | 🕗 SOLVE: 0.0269267503 | 📌 HamCycle | 🕗 CERTIFY: 0.0664894581
| 🇳  105 | ⭕️    1587880 | 🕗 SOLVE: 0.0275232922 | 📌 HamCycle | 🕗 CERTIFY: 0.0612831675
| 🇳  106 | ⭕️    1633248 | 🕗 SOLVE: 0.0281232502 | 📌 HamCycle | 🕗 CERTIFY: 0.0626725033
| 🇳  107 | ⭕️    1679472 | 🕗 SOLVE: 0.0291454587 | 📌 HamCycle | 🕗 CERTIFY: 0.0660361648
| 🇳  108 | ⭕️    1726560 | 🕗 SOLVE: 0.0296714995 | 📌 HamCycle | 🕗 CERTIFY: 0.0665046275
| 🇳  109 | ⭕️    1774520 | 🕗 SOLVE: 0.0304882079 | 📌 HamCycle | 🕗 CERTIFY: 0.0687586218
| 🇳  110 | ⭕️    1823360 | 🕗 SOLVE: 0.0312354155 | 📌 HamCycle | 🕗 CERTIFY: 0.0711183771
| 🇳  111 | ⭕️    1873088 | 🕗 SOLVE: 0.0320182070 | 📌 HamCycle | 🕗 CERTIFY: 0.1005262882
| 🇳  112 | ⭕️    1923712 | 🕗 SOLVE: 0.0330021679 | 📌 HamCycle | 🕗 CERTIFY: 0.1009249613
| 🇳  113 | ⭕️    1975240 | 🕗 SOLVE: 0.0342445411 | 📌 HamCycle | 🕗 CERTIFY: 0.1015338749
| 🇳  114 | ⭕️    2027680 | 🕗 SOLVE: 0.0350919999 | 📌 HamCycle | 🕗 CERTIFY: 0.1026133373
| 🇳  115 | ⭕️    2081040 | 🕗 SOLVE: 0.0360104553 | 📌 HamCycle | 🕗 CERTIFY: 0.1042178720
| 🇳  116 | ⭕️    2135328 | 🕗 SOLVE: 0.0369302928 | 📌 HamCycle | 🕗 CERTIFY: 0.1053217128
| 🇳  117 | ⭕️    2190552 | 🕗 SOLVE: 0.0381915830 | 📌 HamCycle | 🕗 CERTIFY: 0.1082956269
| 🇳  118 | ⭕️    2246720 | 🕗 SOLVE: 0.0385576226 | 📌 HamCycle | 🕗 CERTIFY: 0.1080350876
| 🇳  119 | ⭕️    2303840 | 🕗 SOLVE: 0.0401081257 | 📌 HamCycle | 🕗 CERTIFY: 0.1091707945
| 🇳  120 | ⭕️    2361920 | 🕗 SOLVE: 0.0406793319 | 📌 HamCycle | 🕗 CERTIFY: 0.1106343344
| 🇳  121 | ⭕️    2420968 | 🕗 SOLVE: 0.0421322919 | 📌 HamCycle | 🕗 CERTIFY: 0.1123057529
| 🇳  122 | ⭕️    2480992 | 🕗 SOLVE: 0.0432262495 | 📌 HamCycle | 🕗 CERTIFY: 0.1130926237
| 🇳  123 | ⭕️    2542000 | 🕗 SOLVE: 0.0448335856 | 📌 HamCycle | 🕗 CERTIFY: 0.1150822043
| 🇳  124 | ⭕️    2604000 | 🕗 SOLVE: 0.0458601229 | 📌 HamCycle | 🕗 CERTIFY: 0.1177988723
| 🇳  125 | ⭕️    2667000 | 🕗 SOLVE: 0.0467119589 | 📌 HamCycle | 🕗 CERTIFY: 0.1186887920
| 🇳  126 | ⭕️    2731008 | 🕗 SOLVE: 0.0481842496 | 📌 HamCycle | 🕗 CERTIFY: 0.1199836656
| 🇳  127 | ⭕️    2796032 | 🕗 SOLVE: 0.0490408763 | 📌 HamCycle | 🕗 CERTIFY: 0.1218082905
| 🇳  128 | ⭕️    2862080 | 🕗 SOLVE: 0.0506184176 | 📌 HamCycle | 🕗 CERTIFY: 0.1251665801
| 🇳  129 | ⭕️    2929160 | 🕗 SOLVE: 0.0517759174 | 📌 HamCycle | 🕗 CERTIFY: 0.1267738342
| 🇳  130 | ⭕️    2997280 | 🕗 SOLVE: 0.0529624149 | 📌 HamCycle | 🕗 CERTIFY: 0.1296220869
| 🇳  131 | ⭕️    3066448 | 🕗 SOLVE: 0.0550655387 | 📌 HamCycle | 🕗 CERTIFY: 0.1309237480
| 🇳  132 | ⭕️    3136672 | 🕗 SOLVE: 0.0554944165 | 📌 HamCycle | 🕗 CERTIFY: 0.1328032464
| 🇳  133 | ⭕️    3207960 | 🕗 SOLVE: 0.0574050397 | 📌 HamCycle | 🕗 CERTIFY: 0.1369607449
| 🇳  134 | ⭕️    3280320 | 🕗 SOLVE: 0.0590833314 | 📌 HamCycle | 🕗 CERTIFY: 0.1388695091
| 🇳  135 | ⭕️    3353760 | 🕗 SOLVE: 0.0595314987 | 📌 HamCycle | 🕗 CERTIFY: 0.1400721222
| 🇳  136 | ⭕️    3428288 | 🕗 SOLVE: 0.0616207533 | 📌 HamCycle | 🕗 CERTIFY: 0.1448406726
| 🇳  137 | ⭕️    3503912 | 🕗 SOLVE: 0.0627114177 | 📌 HamCycle | 🕗 CERTIFY: 0.1477504671
| 🇳  138 | ⭕️    3580640 | 🕗 SOLVE: 0.0649086237 | 📌 HamCycle | 🕗 CERTIFY: 0.1520039588
| 🇳  139 | ⭕️    3658480 | 🕗 SOLVE: 0.0664635450 | 📌 HamCycle | 🕗 CERTIFY: 0.1561306268
| 🇳  140 | ⭕️    3737440 | 🕗 SOLVE: 0.0669075847 | 📌 HamCycle | 🕗 CERTIFY: 0.2280444652
| 🇳  141 | ⭕️    3817528 | 🕗 SOLVE: 0.0695139617 | 📌 HamCycle | 🕗 CERTIFY: 0.2208655328
| 🇳  142 | ⭕️    3898752 | 🕗 SOLVE: 0.0711619630 | 📌 HamCycle | 🕗 CERTIFY: 0.2267002016
| 🇳  143 | ⭕️    3981120 | 🕗 SOLVE: 0.0727650821 | 📌 HamCycle | 🕗 CERTIFY: 0.2261465788
| 🇳  144 | ⭕️    4064640 | 🕗 SOLVE: 0.0747104138 | 📌 HamCycle | 🕗 CERTIFY: 0.2283535302
| 🇳  145 | ⭕️    4149320 | 🕗 SOLVE: 0.0756006241 | 📌 HamCycle | 🕗 CERTIFY: 0.2292028368
| 🇳  146 | ⭕️    4235168 | 🕗 SOLVE: 0.0783675835 | 📌 HamCycle | 🕗 CERTIFY: 0.2332046777
| 🇳  147 | ⭕️    4322192 | 🕗 SOLVE: 0.0801399201 | 📌 HamCycle | 🕗 CERTIFY: 0.2371101528
| 🇳  148 | ⭕️    4410400 | 🕗 SOLVE: 0.0825362056 | 📌 HamCycle | 🕗 CERTIFY: 0.2389174998
| 🇳  149 | ⭕️    4499800 | 🕗 SOLVE: 0.0837670788 | 📌 HamCycle | 🕗 CERTIFY: 0.2405595332
| 🇳  150 | ⭕️    4590400 | 🕗 SOLVE: 0.0853706226 | 📌 HamCycle | 🕗 CERTIFY: 0.2534982860
| 🇳  151 | ⭕️    4682208 | 🕗 SOLVE: 0.0879387930 | 📌 HamCycle | 🕗 CERTIFY: 0.2461744100
| 🇳  152 | ⭕️    4775232 | 🕗 SOLVE: 0.0900958702 | 📌 HamCycle | 🕗 CERTIFY: 0.2561092377
| 🇳  153 | ⭕️    4869480 | 🕗 SOLVE: 0.0924538299 | 📌 HamCycle | 🕗 CERTIFY: 0.2511181235
| 🇳  154 | ⭕️    4964960 | 🕗 SOLVE: 0.0951462910 | 📌 HamCycle | 🕗 CERTIFY: 0.2548812628
| 🇳  155 | ⭕️    5061680 | 🕗 SOLVE: 0.0967189595 | 📌 HamCycle | 🕗 CERTIFY: 0.2591587603
| 🇳  156 | ⭕️    5159648 | 🕗 SOLVE: 0.0988397151 | 📌 HamCycle | 🕗 CERTIFY: 0.2675176263
| 🇳  157 | ⭕️    5258872 | 🕗 SOLVE: 0.1019583791 | 📌 HamCycle | 🕗 CERTIFY: 0.2649063766
| 🇳  158 | ⭕️    5359360 | 🕗 SOLVE: 0.1027188748 | 📌 HamCycle | 🕗 CERTIFY: 0.2696419060
| 🇳  159 | ⭕️    5461120 | 🕗 SOLVE: 0.1059616208 | 📌 HamCycle | 🕗 CERTIFY: 0.2729706466
| 🇳  160 | ⭕️    5564160 | 🕗 SOLVE: 0.1081095040 | 📌 HamCycle | 🕗 CERTIFY: 0.2737050056
| 🇳  161 | ⭕️    5668488 | 🕗 SOLVE: 0.1102593392 | 📌 HamCycle | 🕗 CERTIFY: 0.2801372707
| 🇳  162 | ⭕️    5774112 | 🕗 SOLVE: 0.1126611307 | 📌 HamCycle | 🕗 CERTIFY: 0.2912693322
| 🇳  163 | ⭕️    5881040 | 🕗 SOLVE: 0.1150461286 | 📌 HamCycle | 🕗 CERTIFY: 0.2849387228
| 🇳  164 | ⭕️    5989280 | 🕗 SOLVE: 0.1181682497 | 📌 HamCycle | 🕗 CERTIFY: 0.2900526822
| 🇳  165 | ⭕️    6098840 | 🕗 SOLVE: 0.1205536276 | 📌 HamCycle | 🕗 CERTIFY: 0.2927924097
| 🇳  166 | ⭕️    6209728 | 🕗 SOLVE: 0.1237108335 | 📌 HamCycle | 🕗 CERTIFY: 0.2990247309
| 🇳  167 | ⭕️    6321952 | 🕗 SOLVE: 0.1253125370 | 📌 HamCycle | 🕗 CERTIFY: 0.2991106212
| 🇳  168 | ⭕️    6435520 | 🕗 SOLVE: 0.1278068274 | 📌 HamCycle | 🕗 CERTIFY: 0.3034209311
| 🇳  169 | ⭕️    6550440 | 🕗 SOLVE: 0.1313734204 | 📌 HamCycle | 🕗 CERTIFY: 0.3124341965
| 🇳  170 | ⭕️    6666720 | 🕗 SOLVE: 0.1344302148 | 📌 HamCycle | 🕗 CERTIFY: 0.3156662285
| 🇳  171 | ⭕️    6784368 | 🕗 SOLVE: 0.1370944977 | 📌 HamCycle | 🕗 CERTIFY: 0.3181820214
| 🇳  172 | ⭕️    6903392 | 🕗 SOLVE: 0.1395644099 | 📌 HamCycle | 🕗 CERTIFY: 0.3229197562
| 🇳  173 | ⭕️    7023800 | 🕗 SOLVE: 0.1428508759 | 📌 HamCycle | 🕗 CERTIFY: 0.3317772448
| 🇳  174 | ⭕️    7145600 | 🕗 SOLVE: 0.1446470916 | 📌 HamCycle | 🕗 CERTIFY: 0.3363453448
| 🇳  175 | ⭕️    7268800 | 🕗 SOLVE: 0.1485457122 | 📌 HamCycle | 🕗 CERTIFY: 0.3444035947
| 🇳  176 | ⭕️    7393408 | 🕗 SOLVE: 0.1515042037 | 📌 HamCycle | 🕗 CERTIFY: 0.6218777895
| 🇳  177 | ⭕️    7519432 | 🕗 SOLVE: 0.1547876298 | 📌 HamCycle | 🕗 CERTIFY: 0.5140995383
| 🇳  178 | ⭕️    7646880 | 🕗 SOLVE: 0.1581035405 | 📌 HamCycle | 🕗 CERTIFY: 0.6330441833
| 🇳  179 | ⭕️    7775760 | 🕗 SOLVE: 0.1609414220 | 📌 HamCycle | 🕗 CERTIFY: 0.6453571916
| 🇳  180 | ⭕️    7906080 | 🕗 SOLVE: 0.1643581241 | 📌 HamCycle | 🕗 CERTIFY: 0.6517589092
| 🇳  181 | ⭕️    8037848 | 🕗 SOLVE: 0.1674534231 | 📌 HamCycle | 🕗 CERTIFY: 0.6477447748
| 🇳  182 | ⭕️    8171072 | 🕗 SOLVE: 0.1702348739 | 📌 HamCycle | 🕗 CERTIFY: 0.5413854718
| 🇳  183 | ⭕️    8305760 | 🕗 SOLVE: 0.1744350940 | 📌 HamCycle | 🕗 CERTIFY: 0.6639050245
| 🇳  184 | ⭕️    8441920 | 🕗 SOLVE: 0.1764109582 | 📌 HamCycle | 🕗 CERTIFY: 0.6651903987
| 🇳  185 | ⭕️    8579560 | 🕗 SOLVE: 0.1822682619 | 📌 HamCycle | 🕗 CERTIFY: 0.6702162027
| 🇳  186 | ⭕️    8718688 | 🕗 SOLVE: 0.1858128011 | 📌 HamCycle | 🕗 CERTIFY: 0.5836877227
| 🇳  187 | ⭕️    8859312 | 🕗 SOLVE: 0.1893977076 | 📌 HamCycle | 🕗 CERTIFY: 0.6828818321
| 🇳  188 | ⭕️    9001440 | 🕗 SOLVE: 0.1925365478 | 📌 HamCycle | 🕗 CERTIFY: 0.5865250826
| 🇳  189 | ⭕️    9145080 | 🕗 SOLVE: 0.1966986209 | 📌 HamCycle | 🕗 CERTIFY: 0.5793348551
| 🇳  190 | ⭕️    9290240 | 🕗 SOLVE: 0.2020906210 | 📌 HamCycle | 🕗 CERTIFY: 0.7180958986
| 🇳  191 | ⭕️    9436928 | 🕗 SOLVE: 0.2058481574 | 📌 HamCycle | 🕗 CERTIFY: 0.7069823146
| 🇳  192 | ⭕️    9585152 | 🕗 SOLVE: 0.2081409991 | 📌 HamCycle | 🕗 CERTIFY: 0.5968327522
| 🇳  193 | ⭕️    9734920 | 🕗 SOLVE: 0.2135929614 | 📌 HamCycle | 🕗 CERTIFY: 0.6150780916
| 🇳  194 | ⭕️    9886240 | 🕗 SOLVE: 0.2167537063 | 📌 HamCycle | 🕗 CERTIFY: 0.7222892642
| 🇳  195 | ⭕️   10039120 | 🕗 SOLVE: 0.2204166204 | 📌 HamCycle | 🕗 CERTIFY: 0.6161663532
| 🇳  196 | ⭕️   10193568 | 🕗 SOLVE: 0.2253544927 | 📌 HamCycle | 🕗 CERTIFY: 0.6075631380
| 🇳  197 | ⭕️   10349592 | 🕗 SOLVE: 0.2297920436 | 📌 HamCycle | 🕗 CERTIFY: 0.6147156954
| 🇳  198 | ⭕️   10507200 | 🕗 SOLVE: 0.2340660542 | 📌 HamCycle | 🕗 CERTIFY: 0.7407029867
| 🇳  199 | ⭕️   10666400 | 🕗 SOLVE: 0.2374149561 | 📌 HamCycle | 🕗 CERTIFY: 0.6380874515
| 🇳  200 | ⭕️   10827200 | 🕗 SOLVE: 0.2429190129 | 📌 HamCycle | 🕗 CERTIFY: 0.6543717980
| 🇳  201 | ⭕️   10989608 | 🕗 SOLVE: 0.2486662567 | 📌 HamCycle | 🕗 CERTIFY: 0.6485744119
| 🇳  202 | ⭕️   11153632 | 🕗 SOLVE: 0.2522604764 | 📌 HamCycle | 🕗 CERTIFY: 0.8093630672
| 🇳  203 | ⭕️   11319280 | 🕗 SOLVE: 0.2565264702 | 📌 HamCycle | 🕗 CERTIFY: 0.7735610008
| 🇳  204 | ⭕️   11486560 | 🕗 SOLVE: 0.2610530257 | 📌 HamCycle | 🕗 CERTIFY: 0.6568829417
| 🇳  205 | ⭕️   11655480 | 🕗 SOLVE: 0.2666850388 | 📌 HamCycle | 🕗 CERTIFY: 0.7796666026
| 🇳  206 | ⭕️   11826048 | 🕗 SOLVE: 0.2696547210 | 📌 HamCycle | 🕗 CERTIFY: 0.7933348417
| 🇳  207 | ⭕️   11998272 | 🕗 SOLVE: 0.2757692039 | 📌 HamCycle | 🕗 CERTIFY: 0.6838290095
| 🇳  208 | ⭕️   12172160 | 🕗 SOLVE: 0.2815547585 | 📌 HamCycle | 🕗 CERTIFY: 0.7066042423
| 🇳  209 | ⭕️   12347720 | 🕗 SOLVE: 0.2862318456 | 📌 HamCycle | 🕗 CERTIFY: 0.8083460927
| 🇳  210 | ⭕️   12524960 | 🕗 SOLVE: 0.2915490866 | 📌 HamCycle | 🕗 CERTIFY: 0.8528863788
| 🇳  211 | ⭕️   12703888 | 🕗 SOLVE: 0.2969746590 | 📌 HamCycle | 🕗 CERTIFY: 0.8296341300
| 🇳  212 | ⭕️   12884512 | 🕗 SOLVE: 0.3020814955 | 📌 HamCycle | 🕗 CERTIFY: 0.7254588008
| 🇳  213 | ⭕️   13066840 | 🕗 SOLVE: 0.3068147600 | 📌 HamCycle | 🕗 CERTIFY: 0.8842924237
| 🇳  214 | ⭕️   13250880 | 🕗 SOLVE: 0.3128221631 | 📌 HamCycle | 🕗 CERTIFY: 0.8618339896
| 🇳  215 | ⭕️   13436640 | 🕗 SOLVE: 0.3169069588 | 📌 HamCycle | 🕗 CERTIFY: 0.9013076425
| 🇳  216 | ⭕️   13624128 | 🕗 SOLVE: 0.3231490850 | 📌 HamCycle | 🕗 CERTIFY: 0.8855215907
| 🇳  217 | ⭕️   13813352 | 🕗 SOLVE: 0.3292649388 | 📌 HamCycle | 🕗 CERTIFY: 0.8035225868
| 🇳  218 | ⭕️   14004320 | 🕗 SOLVE: 0.3355272114 | 📌 HamCycle | 🕗 CERTIFY: 0.8085771799
| 🇳  219 | ⭕️   14197040 | 🕗 SOLVE: 0.3442309499 | 📌 HamCycle | 🕗 CERTIFY: 0.8052686453
| 🇳  220 | ⭕️   14391520 | 🕗 SOLVE: 0.3481214046 | 📌 HamCycle | 🕗 CERTIFY: 0.8206392527
| 🇳  221 | ⭕️   14587768 | 🕗 SOLVE: 0.3518816829 | 📌 HamCycle | 🕗 CERTIFY: 0.9438311458
| 🇳  222 | ⭕️   14785792 | 🕗 SOLVE: 0.3584341705 | 📌 HamCycle | 🕗 CERTIFY: 1.1485706568
| 🇳  223 | ⭕️   14985600 | 🕗 SOLVE: 0.3655003309 | 📌 HamCycle | 🕗 CERTIFY: 1.1210992336
| 🇳  224 | ⭕️   15187200 | 🕗 SOLVE: 0.3700094819 | 📌 HamCycle | 🕗 CERTIFY: 1.1401512623
| 🇳  225 | ⭕️   15390600 | 🕗 SOLVE: 0.3760042489 | 📌 HamCycle | 🕗 CERTIFY: 1.1593096256
| 🇳  226 | ⭕️   15595808 | 🕗 SOLVE: 0.3842642903 | 📌 HamCycle | 🕗 CERTIFY: 1.6334825754
| 🇳  227 | ⭕️   15802832 | 🕗 SOLVE: 0.3881231546 | 📌 HamCycle | 🕗 CERTIFY: 1.3223994970
| 🇳  228 | ⭕️   16011680 | 🕗 SOLVE: 0.3951343596 | 📌 HamCycle | 🕗 CERTIFY: 1.1764081717
| 🇳  229 | ⭕️   16222360 | 🕗 SOLVE: 0.4000105858 | 📌 HamCycle | 🕗 CERTIFY: 1.1961133480
| 🇳  230 | ⭕️   16434880 | 🕗 SOLVE: 0.4087437987 | 📌 HamCycle | 🕗 CERTIFY: 1.1918767691
| 🇳  231 | ⭕️   16649248 | 🕗 SOLVE: 0.4144673347 | 📌 HamCycle | 🕗 CERTIFY: 1.6792132854
| 🇳  232 | ⭕️   16865472 | 🕗 SOLVE: 0.4193789363 | 📌 HamCycle | 🕗 CERTIFY: 1.3638885021
| 🇳  233 | ⭕️   17083560 | 🕗 SOLVE: 0.4283965230 | 📌 HamCycle | 🕗 CERTIFY: 1.7037545443
| 🇳  234 | ⭕️   17303520 | 🕗 SOLVE: 0.4355364442 | 📌 HamCycle | 🕗 CERTIFY: 1.2392940521
| 🇳  235 | ⭕️   17525360 | 🕗 SOLVE: 0.4420328736 | 📌 HamCycle | 🕗 CERTIFY: 1.2464878559
| 🇳  236 | ⭕️   17749088 | 🕗 SOLVE: 0.4497081339 | 📌 HamCycle | 🕗 CERTIFY: 1.7527086735
| 🇳  237 | ⭕️   17974712 | 🕗 SOLVE: 0.4586576819 | 📌 HamCycle | 🕗 CERTIFY: 1.2826663256
| 🇳  238 | ⭕️   18202240 | 🕗 SOLVE: 0.4644054174 | 📌 HamCycle | 🕗 CERTIFY: 1.7741241455
| 🇳  239 | ⭕️   18431680 | 🕗 SOLVE: 0.4720064402 | 📌 HamCycle | 🕗 CERTIFY: 1.3082902431
| 🇳  240 | ⭕️   18663040 | 🕗 SOLVE: 0.4793971479 | 📌 HamCycle | 🕗 CERTIFY: 1.3244841099
| 🇳  241 | ⭕️   18896328 | 🕗 SOLVE: 0.4870021641 | 📌 HamCycle | 🕗 CERTIFY: 1.8328464031
| 🇳  242 | ⭕️   19131552 | 🕗 SOLVE: 0.4953063130 | 📌 HamCycle | 🕗 CERTIFY: 1.3361477852
| 🇳  243 | ⭕️   19368720 | 🕗 SOLVE: 0.5006814003 | 📌 HamCycle | 🕗 CERTIFY: 1.8307106495
| 🇳  244 | ⭕️   19607840 | 🕗 SOLVE: 0.5105180740 | 📌 HamCycle | 🕗 CERTIFY: 1.3737242222
| 🇳  245 | ⭕️   19848920 | 🕗 SOLVE: 0.5178053379 | 📌 HamCycle | 🕗 CERTIFY: 1.3815498352
| 🇳  246 | ⭕️   20091968 | 🕗 SOLVE: 0.5275897980 | 📌 HamCycle | 🕗 CERTIFY: 1.8823841810
| 🇳  247 | ⭕️   20336992 | 🕗 SOLVE: 0.5355270505 | 📌 HamCycle | 🕗 CERTIFY: 1.4196995497
| 🇳  248 | ⭕️   20584000 | 🕗 SOLVE: 0.5422776937 | 📌 HamCycle | 🕗 CERTIFY: 1.9163603783
| 🇳  249 | ⭕️   20833000 | 🕗 SOLVE: 0.5512285233 | 📌 HamCycle | 🕗 CERTIFY: 1.4578850269
| 🇳  250 | ⭕️   21084000 | 🕗 SOLVE: 0.5612413883 | 📌 HamCycle | 🕗 CERTIFY: 1.4890100956
| 🇳  251 | ⭕️   21337008 | 🕗 SOLVE: 0.5668765903 | 📌 HamCycle | 🕗 CERTIFY: 1.5651103258
| 🇳  252 | ⭕️   21592032 | 🕗 SOLVE: 0.5786451101 | 📌 HamCycle | 🕗 CERTIFY: 1.5819171667
| 🇳  253 | ⭕️   21849080 | 🕗 SOLVE: 0.5857163668 | 📌 HamCycle | 🕗 CERTIFY: 1.9950194359
| 🇳  254 | ⭕️   22108160 | 🕗 SOLVE: 0.5941022635 | 📌 HamCycle | 🕗 CERTIFY: 1.6146483421
| 🇳  255 | ⭕️   22369280 | 🕗 SOLVE: 0.6033035517 | 📌 HamCycle | 🕗 CERTIFY: 1.7903946638
| 🇳  256 | ⭕️   22632448 | 🕗 SOLVE: 0.6252706647 | 📌 HamCycle | 🕗 CERTIFY: 2.0070071220
| 🇳  257 | ⭕️   22897672 | 🕗 SOLVE: 0.6327763200 | 📌 HamCycle | 🕗 CERTIFY: 1.9975817204
| 🇳  258 | ⭕️   23164960 | 🕗 SOLVE: 0.6434557438 | 📌 HamCycle | 🕗 CERTIFY: 2.0128102303
| 🇳  259 | ⭕️   23434320 | 🕗 SOLVE: 0.6519679427 | 📌 HamCycle | 🕗 CERTIFY: 2.0547678471
| 🇳  260 | ⭕️   23705760 | 🕗 SOLVE: 0.6610835791 | 📌 HamCycle | 🕗 CERTIFY: 2.0537357330
| 🇳  261 | ⭕️   23979288 | 🕗 SOLVE: 0.6737011075 | 📌 HamCycle | 🕗 CERTIFY: 2.0801942348
| 🇳  262 | ⭕️   24254912 | 🕗 SOLVE: 0.6827772856 | 📌 HamCycle | 🕗 CERTIFY: 2.1148202419
| 🇳  263 | ⭕️   24532640 | 🕗 SOLVE: 0.6937137246 | 📌 HamCycle | 🕗 CERTIFY: 2.1302053928
| 🇳  264 | ⭕️   24812480 | 🕗 SOLVE: 0.7061291337 | 📌 HamCycle | 🕗 CERTIFY: 2.1472744942
| 🇳  265 | ⭕️   25094440 | 🕗 SOLVE: 0.7181665301 | 📌 HamCycle | 🕗 CERTIFY: 2.1513209343
| 🇳  266 | ⭕️   25378528 | 🕗 SOLVE: 0.7214239240 | 📌 HamCycle | 🕗 CERTIFY: 2.1812005043
| 🇳  267 | ⭕️   25664752 | 🕗 SOLVE: 0.7408060431 | 📌 HamCycle | 🕗 CERTIFY: 2.2002990246
| 🇳  268 | ⭕️   25953120 | 🕗 SOLVE: 0.7548851967 | 📌 HamCycle | 🕗 CERTIFY: 2.2349390984
| 🇳  269 | ⭕️   26243640 | 🕗 SOLVE: 0.7708546519 | 📌 HamCycle | 🕗 CERTIFY: 2.2654895782
| 🇳  270 | ⭕️   26536320 | 🕗 SOLVE: 0.7776065469 | 📌 HamCycle | 🕗 CERTIFY: 2.2905611992
| 🇳  271 | ⭕️   26831168 | 🕗 SOLVE: 0.7914184332 | 📌 HamCycle | 🕗 CERTIFY: 2.3182470798
| 🇳  272 | ⭕️   27128192 | 🕗 SOLVE: 0.7949767709 | 📌 HamCycle | 🕗 CERTIFY: 2.3420846462
| 🇳  273 | ⭕️   27427400 | 🕗 SOLVE: 0.8101597428 | 📌 HamCycle | 🕗 CERTIFY: 2.3544745445
| 🇳  274 | ⭕️   27728800 | 🕗 SOLVE: 0.8232811093 | 📌 HamCycle | 🕗 CERTIFY: 2.3872075081
| 🇳  275 | ⭕️   28032400 | 🕗 SOLVE: 0.8323560357 | 📌 HamCycle | 🕗 CERTIFY: 2.4105651379
| 🇳  276 | ⭕️   28338208 | 🕗 SOLVE: 0.8405999541 | 📌 HamCycle | 🕗 CERTIFY: 2.4286603928
| 🇳  277 | ⭕️   28646232 | 🕗 SOLVE: 0.8595604300 | 📌 HamCycle | 🕗 CERTIFY: 2.4654700756
| 🇳  278 | ⭕️   28956480 | 🕗 SOLVE: 0.8712547421 | 📌 HamCycle | 🕗 CERTIFY: 2.5027334690
| 🇳  279 | ⭕️   29268960 | 🕗 SOLVE: 0.8897573352 | 📌 HamCycle | 🕗 CERTIFY: 2.5250985622
| 🇳  280 | ⭕️   29583680 | 🕗 SOLVE: 0.9022457004 | 📌 HamCycle | 🕗 CERTIFY: 3.9595308304
| 🇳  281 | ⭕️   29900648 | 🕗 SOLVE: 0.9105177522 | 📌 HamCycle | 🕗 CERTIFY: 3.9156217575
| 🇳  282 | ⭕️   30219872 | 🕗 SOLVE: 0.9232621193 | 📌 HamCycle | 🕗 CERTIFY: 3.9201886654
| 🇳  283 | ⭕️   30541360 | 🕗 SOLVE: 0.9378010631 | 📌 HamCycle | 🕗 CERTIFY: 3.9439935684
| 🇳  284 | ⭕️   30865120 | 🕗 SOLVE: 0.9549767971 | 📌 HamCycle | 🕗 CERTIFY: 3.9746809006
| 🇳  285 | ⭕️   31191160 | 🕗 SOLVE: 0.9584531784 | 📌 HamCycle | 🕗 CERTIFY: 4.0020337105
| 🇳  286 | ⭕️   31519488 | 🕗 SOLVE: 0.9730947018 | 📌 HamCycle | 🕗 CERTIFY: 4.0223727226
| 🇳  287 | ⭕️   31850112 | 🕗 SOLVE: 0.9911147952 | 📌 HamCycle | 🕗 CERTIFY: 4.0365262032
| 🇳  288 | ⭕️   32183040 | 🕗 SOLVE: 1.0046166182 | 📌 HamCycle | 🕗 CERTIFY: 4.0572113991
| 🇳  289 | ⭕️   32518280 | 🕗 SOLVE: 1.0104249716 | 📌 HamCycle | 🕗 CERTIFY: 4.1083045006
| 🇳  290 | ⭕️   32855840 | 🕗 SOLVE: 1.0377593040 | 📌 HamCycle | 🕗 CERTIFY: 4.1628155708
| 🇳  291 | ⭕️   33195728 | 🕗 SOLVE: 1.0476689339 | 📌 HamCycle | 🕗 CERTIFY: 4.1424889565
| 🇳  292 | ⭕️   33537952 | 🕗 SOLVE: 1.0634326935 | 📌 HamCycle | 🕗 CERTIFY: 4.1548690796
| 🇳  293 | ⭕️   33882520 | 🕗 SOLVE: 1.0750583410 | 📌 HamCycle | 🕗 CERTIFY: 4.2024521828
| 🇳  294 | ⭕️   34229440 | 🕗 SOLVE: 1.0891497135 | 📌 HamCycle | 🕗 CERTIFY: 4.2095537186
| 🇳  295 | ⭕️   34578720 | 🕗 SOLVE: 1.1051938534 | 📌 HamCycle | 🕗 CERTIFY: 4.2318449020
| 🇳  296 | ⭕️   34930368 | 🕗 SOLVE: 1.1190025806 | 📌 HamCycle | 🕗 CERTIFY: 4.2491240501
| 🇳  297 | ⭕️   35284392 | 🕗 SOLVE: 1.1377320290 | 📌 HamCycle | 🕗 CERTIFY: 4.2802762985
| 🇳  298 | ⭕️   35640800 | 🕗 SOLVE: 1.1448394060 | 📌 HamCycle | 🕗 CERTIFY: 3.1491918564
| 🇳  299 | ⭕️   35999600 | 🕗 SOLVE: 1.1700984240 | 📌 HamCycle | 🕗 CERTIFY: 3.1761534214
| 🇳  300 | ⭕️   36360800 | 🕗 SOLVE: 1.1781280041 | 📌 HamCycle | 🕗 CERTIFY: 3.2107572556
| 🇳  301 | ⭕️   36724408 | 🕗 SOLVE: 1.1975694895 | 📌 HamCycle | 🕗 CERTIFY: 3.2214367390
| 🇳  302 | ⭕️   37090432 | 🕗 SOLVE: 1.2077808380 | 📌 HamCycle | 🕗 CERTIFY: 3.2424554825
| 🇳  303 | ⭕️   37458880 | 🕗 SOLVE: 1.2310798168 | 📌 HamCycle | 🕗 CERTIFY: 3.3044102192
| 🇳  304 | ⭕️   37829760 | 🕗 SOLVE: 1.2437727451 | 📌 HamCycle | 🕗 CERTIFY: 3.3183033466
| 🇳  305 | ⭕️   38203080 | 🕗 SOLVE: 1.2562892437 | 📌 HamCycle | 🕗 CERTIFY: 3.3292438984
| 🇳  306 | ⭕️   38578848 | 🕗 SOLVE: 1.2775118351 | 📌 HamCycle | 🕗 CERTIFY: 3.3634119034
| 🇳  307 | ⭕️   38957072 | 🕗 SOLVE: 1.2918379307 | 📌 HamCycle | 🕗 CERTIFY: 3.3813583851
| 🇳  308 | ⭕️   39337760 | 🕗 SOLVE: 1.3126298189 | 📌 HamCycle | 🕗 CERTIFY: 3.4100592136
| 🇳  309 | ⭕️   39720920 | 🕗 SOLVE: 1.3313826323 | 📌 HamCycle | 🕗 CERTIFY: 3.4335229397
| 🇳  310 | ⭕️   40106560 | 🕗 SOLVE: 1.3432358503 | 📌 HamCycle | 🕗 CERTIFY: 3.4604668617
| 🇳  311 | ⭕️   40494688 | 🕗 SOLVE: 1.3551286459 | 📌 HamCycle | 🕗 CERTIFY: 3.5140140057
| 🇳  312 | ⭕️   40885312 | 🕗 SOLVE: 1.3776762486 | 📌 HamCycle | 🕗 CERTIFY: 3.5081214905
| 🇳  313 | ⭕️   41278440 | 🕗 SOLVE: 1.3859341145 | 📌 HamCycle | 🕗 CERTIFY: 3.8133878708
| 🇳  314 | ⭕️   41674080 | 🕗 SOLVE: 1.4143201113 | 📌 HamCycle | 🕗 CERTIFY: 3.8368594646
| 🇳  315 | ⭕️   42072240 | 🕗 SOLVE: 1.4270088673 | 📌 HamCycle | 🕗 CERTIFY: 3.8830280304
| 🇳  316 | ⭕️   42472928 | 🕗 SOLVE: 1.4452447891 | 📌 HamCycle | 🕗 CERTIFY: 3.8944842815
| 🇳  317 | ⭕️   42876152 | 🕗 SOLVE: 1.4563241005 | 📌 HamCycle | 🕗 CERTIFY: 3.9231369495
| 🇳  318 | ⭕️   43281920 | 🕗 SOLVE: 1.4832894802 | 📌 HamCycle | 🕗 CERTIFY: 3.9639568329
| 🇳  319 | ⭕️   43690240 | 🕗 SOLVE: 1.5032505989 | 📌 HamCycle | 🕗 CERTIFY: 3.7917933464
| 🇳  320 | ⭕️   44101120 | 🕗 SOLVE: 1.4987275600 | 📌 HamCycle | 🕗 CERTIFY: 3.8237326145
| 🇳  321 | ⭕️   44514568 | 🕗 SOLVE: 1.5415856838 | 📌 HamCycle | 🕗 CERTIFY: 3.8604440689
| 🇳  322 | ⭕️   44930592 | 🕗 SOLVE: 1.5574114323 | 📌 HamCycle | 🕗 CERTIFY: 3.8763654232
| 🇳  323 | ⭕️   45349200 | 🕗 SOLVE: 1.5809527636 | 📌 HamCycle | 🕗 CERTIFY: 3.9077606201
| 🇳  324 | ⭕️   45770400 | 🕗 SOLVE: 1.5889067650 | 📌 HamCycle | 🕗 CERTIFY: 3.9349989891
| 🇳  325 | ⭕️   46194200 | 🕗 SOLVE: 1.6091566086 | 📌 HamCycle | 🕗 CERTIFY: 3.7402067184
| 🇳  326 | ⭕️   46620608 | 🕗 SOLVE: 1.6281342506 | 📌 HamCycle | 🕗 CERTIFY: 3.7903118134
| 🇳  327 | ⭕️   47049632 | 🕗 SOLVE: 1.6434062719 | 📌 HamCycle | 🕗 CERTIFY: 3.8102800846
| 🇳  328 | ⭕️   47481280 | 🕗 SOLVE: 1.6657049656 | 📌 HamCycle | 🕗 CERTIFY: 3.8388199806
| 🇳  329 | ⭕️   47915560 | 🕗 SOLVE: 1.6852111816 | 📌 HamCycle | 🕗 CERTIFY: 3.8824086189
| 🇳  330 | ⭕️   48352480 | 🕗 SOLVE: 1.6988587379 | 📌 HamCycle | 🕗 CERTIFY: 3.9159724712
| 🇳  331 | ⭕️   48792048 | 🕗 SOLVE: 1.7205665112 | 📌 HamCycle | 🕗 CERTIFY: 3.9270329475
| 🇳  332 | ⭕️   49234272 | 🕗 SOLVE: 1.7402656078 | 📌 HamCycle | 🕗 CERTIFY: 3.9635765553
| 🇳  333 | ⭕️   49679160 | 🕗 SOLVE: 1.7440986633 | 📌 HamCycle | 🕗 CERTIFY: 4.0000925064
| 🇳  334 | ⭕️   50126720 | 🕗 SOLVE: 1.7606859207 | 📌 HamCycle | 🕗 CERTIFY: 4.0455656052
| 🇳  335 | ⭕️   50576960 | 🕗 SOLVE: 1.7603950500 | 📌 HamCycle | 🕗 CERTIFY: 4.0871315002
| 🇳  336 | ⭕️   51029888 | 🕗 SOLVE: 1.7924468517 | 📌 HamCycle | 🕗 CERTIFY: 4.1059250832
| 🇳  337 | ⭕️   51485512 | 🕗 SOLVE: 1.8042736053 | 📌 HamCycle | 🕗 CERTIFY: 4.1417927742
| 🇳  338 | ⭕️   51943840 | 🕗 SOLVE: 1.8152449131 | 📌 HamCycle | 🕗 CERTIFY: 4.2028632164
| 🇳  339 | ⭕️   52404880 | 🕗 SOLVE: 1.8430088758 | 📌 HamCycle | 🕗 CERTIFY: 4.2210140228
| 🇳  340 | ⭕️   52868640 | 🕗 SOLVE: 1.8608152866 | 📌 HamCycle | 🕗 CERTIFY: 4.2588133812
| 🇳  341 | ⭕️   53335128 | 🕗 SOLVE: 1.8804984093 | 📌 HamCycle | 🕗 CERTIFY: 4.2893118858
| 🇳  342 | ⭕️   53804352 | 🕗 SOLVE: 1.8971538544 | 📌 HamCycle | 🕗 CERTIFY: 4.3358302116
| 🇳  343 | ⭕️   54276320 | 🕗 SOLVE: 1.9238594770 | 📌 HamCycle | 🕗 CERTIFY: 4.3955092430
| 🇳  344 | ⭕️   54751040 | 🕗 SOLVE: 1.9492731094 | 📌 HamCycle | 🕗 CERTIFY: 4.4461650848
| 🇳  345 | ⭕️   55228520 | 🕗 SOLVE: 1.9652727842 | 📌 HamCycle | 🕗 CERTIFY: 4.4781270027
| 🇳  346 | ⭕️   55708768 | 🕗 SOLVE: 1.9872024059 | 📌 HamCycle | 🕗 CERTIFY: 4.5174918175
| 🇳  347 | ⭕️   56191792 | 🕗 SOLVE: 2.0084447861 | 📌 HamCycle | 🕗 CERTIFY: 4.5431728363
| 🇳  348 | ⭕️   56677600 | 🕗 SOLVE: 2.0305397511 | 📌 HamCycle | 🕗 CERTIFY: 4.6186332703
| 🇳  349 | ⭕️   57166200 | 🕗 SOLVE: 2.0602569580 | 📌 HamCycle | 🕗 CERTIFY: 4.6728415489
| 🇳  350 | ⭕️   57657600 | 🕗 SOLVE: 2.0797111988 | 📌 HamCycle | 🕗 CERTIFY: 4.7146635056
| 🇳  351 | ⭕️   58151808 | 🕗 SOLVE: 2.0990145206 | 📌 HamCycle | 🕗 CERTIFY: 4.7682824135
| 🇳  352 | ⭕️   58648832 | 🕗 SOLVE: 2.1228535175 | 📌 HamCycle | 🕗 CERTIFY: 4.8369207382
| 🇳  353 | ⭕️   59148680 | 🕗 SOLVE: 2.1440532207 | 📌 HamCycle | 🕗 CERTIFY: 6.5556139946
| 🇳  354 | ⭕️   59651360 | 🕗 SOLVE: 2.1542353630 | 📌 HamCycle | 🕗 CERTIFY: 6.5602059364
| 🇳  355 | ⭕️   60156880 | 🕗 SOLVE: 2.1813373566 | 📌 HamCycle | 🕗 CERTIFY: 6.3392696381
| 🇳  356 | ⭕️   60665248 | 🕗 SOLVE: 2.2163894176 | 📌 HamCycle | 🕗 CERTIFY: 6.6011295319
| 🇳  357 | ⭕️   61176472 | 🕗 SOLVE: 2.2290546894 | 📌 HamCycle | 🕗 CERTIFY: 6.4759545326
| 🇳  358 | ⭕️   61690560 | 🕗 SOLVE: 2.2577862740 | 📌 HamCycle | 🕗 CERTIFY: 6.6506481171
| 🇳  359 | ⭕️   62207520 | 🕗 SOLVE: 2.2764096260 | 📌 HamCycle | 🕗 CERTIFY: 6.4798011780
| 🇳  360 | ⭕️   62727360 | 🕗 SOLVE: 2.3054440022 | 📌 HamCycle | 🕗 CERTIFY: 6.8128614426
| 🇳  361 | ⭕️   63250088 | 🕗 SOLVE: 2.3209156990 | 📌 HamCycle | 🕗 CERTIFY: 6.7695083618
| 🇳  362 | ⭕️   63775712 | 🕗 SOLVE: 2.3474292755 | 📌 HamCycle | 🕗 CERTIFY: 6.9389247894
| 🇳  363 | ⭕️   64304240 | 🕗 SOLVE: 2.3677115440 | 📌 HamCycle | 🕗 CERTIFY: 6.8739924431
| 🇳  364 | ⭕️   64835680 | 🕗 SOLVE: 2.4003891945 | 📌 HamCycle | 🕗 CERTIFY: 7.0439419746
| 🇳  365 | ⭕️   65370040 | 🕗 SOLVE: 2.4228780270 | 📌 HamCycle | 🕗 CERTIFY: 7.0110702515
| 🇳  366 | ⭕️   65907328 | 🕗 SOLVE: 2.4527673721 | 📌 HamCycle | 🕗 CERTIFY: 7.0808081627
| 🇳  367 | ⭕️   66447552 | 🕗 SOLVE: 2.4714524746 | 📌 HamCycle | 🕗 CERTIFY: 7.1432290077
| 🇳  368 | ⭕️   66990720 | 🕗 SOLVE: 2.5030930042 | 📌 HamCycle | 🕗 CERTIFY: 7.1406183243
| 🇳  369 | ⭕️   67536840 | 🕗 SOLVE: 2.5241811275 | 📌 HamCycle | 🕗 CERTIFY: 7.2628836632
| 🇳  370 | ⭕️   68085920 | 🕗 SOLVE: 2.5526947975 | 📌 HamCycle | 🕗 CERTIFY: 7.2036051750
| 🇳  371 | ⭕️   68637968 | 🕗 SOLVE: 2.5826325417 | 📌 HamCycle | 🕗 CERTIFY: 7.2121291161
| 🇳  372 | ⭕️   69192992 | 🕗 SOLVE: 2.6038544178 | 📌 HamCycle | 🕗 CERTIFY: 7.3454842567
| 🇳  373 | ⭕️   69751000 | 🕗 SOLVE: 2.6332132816 | 📌 HamCycle | 🕗 CERTIFY: 7.4556746483
| 🇳  374 | ⭕️   70312000 | 🕗 SOLVE: 2.6593961716 | 📌 HamCycle | 🕗 CERTIFY: 7.5913524628
| 🇳  375 | ⭕️   70876000 | 🕗 SOLVE: 2.6879386902 | 📌 HamCycle | 🕗 CERTIFY: 7.5597200394
| 🇳  376 | ⭕️   71443008 | 🕗 SOLVE: 2.7146666050 | 📌 HamCycle | 🕗 CERTIFY: 7.6940355301
| 🇳  377 | ⭕️   72013032 | 🕗 SOLVE: 2.7563600540 | 📌 HamCycle | 🕗 CERTIFY: 7.6967515945
| 🇳  378 | ⭕️   72586080 | 🕗 SOLVE: 2.7757461071 | 📌 HamCycle | 🕗 CERTIFY: 7.6325531006
| 🇳  379 | ⭕️   73162160 | 🕗 SOLVE: 2.7977271080 | 📌 HamCycle | 🕗 CERTIFY: 7.8037438393
| 🇳  380 | ⭕️   73741280 | 🕗 SOLVE: 2.8362705708 | 📌 HamCycle | 🕗 CERTIFY: 7.7094879150
| 🇳  381 | ⭕️   74323448 | 🕗 SOLVE: 2.8553814888 | 📌 HamCycle | 🕗 CERTIFY: 7.7820067406
| 🇳  382 | ⭕️   74908672 | 🕗 SOLVE: 2.8915631771 | 📌 HamCycle | 🕗 CERTIFY: 7.8524198532
| 🇳  383 | ⭕️   75496960 | 🕗 SOLVE: 2.9218170643 | 📌 HamCycle | 🕗 CERTIFY: 7.8899736404
| 🇳  384 | ⭕️   76088320 | 🕗 SOLVE: 2.9435520172 | 📌 HamCycle | 🕗 CERTIFY: 8.0496578217
| 🇳  385 | ⭕️   76682760 | 🕗 SOLVE: 2.9808762074 | 📌 HamCycle | 🕗 CERTIFY: 7.9643325806
| 🇳  386 | ⭕️   77280288 | 🕗 SOLVE: 3.0130045414 | 📌 HamCycle | 🕗 CERTIFY: 8.0851058960
| 🇳  387 | ⭕️   77880912 | 🕗 SOLVE: 3.0382304192 | 📌 HamCycle | 🕗 CERTIFY: 8.1256341934
| 🇳  388 | ⭕️   78484640 | 🕗 SOLVE: 3.0813434124 | 📌 HamCycle | 🕗 CERTIFY: 8.1759147644
| 🇳  389 | ⭕️   79091480 | 🕗 SOLVE: 3.0979421139 | 📌 HamCycle | 🕗 CERTIFY: 8.2115936279
| 🇳  390 | ⭕️   79701440 | 🕗 SOLVE: 3.1379735470 | 📌 HamCycle | 🕗 CERTIFY: 8.2420444489
| 🇳  391 | ⭕️   80314528 | 🕗 SOLVE: 3.1679635048 | 📌 HamCycle | 🕗 CERTIFY: 8.2778511047
| 🇳  392 | ⭕️   80930752 | 🕗 SOLVE: 3.1992635727 | 📌 HamCycle | 🕗 CERTIFY: 8.3268127441
| 🇳  393 | ⭕️   81550120 | 🕗 SOLVE: 3.2300934792 | 📌 HamCycle | 🕗 CERTIFY: 8.3866329193
| 🇳  394 | ⭕️   82172640 | 🕗 SOLVE: 3.2542500496 | 📌 HamCycle | 🕗 CERTIFY: 8.4211807251
| 🇳  395 | ⭕️   82798320 | 🕗 SOLVE: 3.2980310917 | 📌 HamCycle | 🕗 CERTIFY: 8.4633646011
| 🇳  396 | ⭕️   83427168 | 🕗 SOLVE: 3.3268754482 | 📌 HamCycle | 🕗 CERTIFY: 8.5459022522
| 🇳  397 | ⭕️   84059192 | 🕗 SOLVE: 3.3620347977 | 📌 HamCycle | 🕗 CERTIFY: 11.4395685196
| 🇳  398 | ⭕️   84694400 | 🕗 SOLVE: 3.3948915005 | 📌 HamCycle | 🕗 CERTIFY: 9.0779886246
| 🇳  399 | ⭕️   85332800 | 🕗 SOLVE: 3.4242134094 | 📌 HamCycle | 🕗 CERTIFY: 9.1040792465
| 🇳  400 | ⭕️   85974400 | 🕗 SOLVE: 3.4592323303 | 📌 HamCycle | 🕗 CERTIFY: 9.1683006287
| 🇳  401 | ⭕️   86619208 | 🕗 SOLVE: 3.4899778366 | 📌 HamCycle | 🕗 CERTIFY: 9.1612701416
| 🇳  402 | ⭕️   87267232 | 🕗 SOLVE: 3.5227410793 | 📌 HamCycle | 🕗 CERTIFY: 9.3894939423
| 🇳  403 | ⭕️   87918480 | 🕗 SOLVE: 3.5635776520 | 📌 HamCycle | 🕗 CERTIFY: 9.2686271667
| 🇳  404 | ⭕️   88572960 | 🕗 SOLVE: 3.5935065746 | 📌 HamCycle | 🕗 CERTIFY: 9.5218153000
| 🇳  405 | ⭕️   89230680 | 🕗 SOLVE: 3.6327414513 | 📌 HamCycle | 🕗 CERTIFY: 9.4661979675
| 🇳  406 | ⭕️   89891648 | 🕗 SOLVE: 3.6607322693 | 📌 HamCycle | 🕗 CERTIFY: 10.0523681641
| 🇳  407 | ⭕️   90555872 | 🕗 SOLVE: 3.6933164597 | 📌 HamCycle | 🕗 CERTIFY: 9.9899377823
| 🇳  408 | ⭕️   91223360 | 🕗 SOLVE: 3.7418606281 | 📌 HamCycle | 🕗 CERTIFY: 15.6869812012
| 🇳  409 | ⭕️   91894120 | 🕗 SOLVE: 3.7638649940 | 📌 HamCycle | 🕗 CERTIFY: 12.5705986023
| 🇳  410 | ⭕️   92568160 | 🕗 SOLVE: 3.8051760197 | 📌 HamCycle | 🕗 CERTIFY: 20.3284778595
| 🇳  411 | ⭕️   93245488 | 🕗 SOLVE: 3.8393759727 | 📌 HamCycle | 🕗 CERTIFY: 19.9098358154
| 🇳  412 | ⭕️   93926112 | 🕗 SOLVE: 3.8652687073 | 📌 HamCycle | 🕗 CERTIFY: 12.0320129395
| 🇳  413 | ⭕️   94610040 | 🕗 SOLVE: 3.9112775326 | 📌 HamCycle | 🕗 CERTIFY: 14.1938037872
| 🇳  414 | ⭕️   95297280 | 🕗 SOLVE: 3.9450783730 | 📌 HamCycle | 🕗 CERTIFY: 22.5493774414
| 🇳  415 | ⭕️   95987840 | 🕗 SOLVE: 3.9859733582 | 📌 HamCycle | 🕗 CERTIFY: 26.1627197266
| 🇳  416 | ⭕️   96681728 | 🕗 SOLVE: 4.0158762932 | 📌 HamCycle | 🕗 CERTIFY: 26.4545860291
| 🇳  417 | ⭕️   97378952 | 🕗 SOLVE: 4.0553226471 | 📌 HamCycle | 🕗 CERTIFY: 13.4279499054
| 🇳  418 | ⭕️   98079520 | 🕗 SOLVE: 4.0899710655 | 📌 HamCycle | 🕗 CERTIFY: 20.5381488800
| 🇳  419 | ⭕️   98783440 | 🕗 SOLVE: 4.1247377396 | 📌 HamCycle | 🕗 CERTIFY: 29.2129211426
| 🇳  420 | ⭕️   99490720 | 🕗 SOLVE: 4.1694235802 | 📌 HamCycle | 🕗 CERTIFY: 12.3191041946
| 🇳  421 | ⭕️  100201368 | 🕗 4.2037181854 |
| 🇳  422 | ⭕️  100915392 | 🕗 4.2537355423 |
| 🇳  423 | ⭕️  101632800 | 🕗 4.2907476425 |
| 🇳  424 | ⭕️  102353600 | 🕗 4.3410701752 |
| 🇳  425 | ⭕️  103077800 | 🕗 4.4085803032 |
| 🇳  426 | ⭕️  103805408 | 🕗 4.4271612167 |
| 🇳  427 | ⭕️  104536432 | 🕗 4.4607510567 |
| 🇳  428 | ⭕️  105270880 | 🕗 4.5109295845 |
| 🇳  429 | ⭕️  106008760 | 🕗 4.5306105614 |
| 🇳  430 | ⭕️  106750080 | 🕗 4.5749878883 |
| 🇳  431 | ⭕️  107494848 | 🕗 4.6500787735 |
| 🇳  432 | ⭕️  108243072 | 🕗 4.6717743874 |
| 🇳  433 | ⭕️  108994760 | 🕗 4.7238435745 |
| 🇳  434 | ⭕️  109749920 | 🕗 4.7592077255 |
| 🇳  435 | ⭕️  110508560 | 🕗 4.7970252037 |
| 🇳  436 | ⭕️  111270688 | 🕗 4.8226242065 |
| 🇳  437 | ⭕️  112036312 | 🕗 4.8673853874 |
| 🇳  438 | ⭕️  112805440 | 🕗 4.9161777496 |
| 🇳  439 | ⭕️  113578080 | 🕗 4.9609446526 |
| 🇳  440 | ⭕️  114354240 | 🕗 5.0242443085 |
| 🇳  441 | ⭕️  115133928 | 🕗 5.0692362785 |
| 🇳  442 | ⭕️  115917152 | 🕗 5.1170597076 |
| 🇳  443 | ⭕️  116703920 | 🕗 5.1668162346 |
| 🇳  444 | ⭕️  117494240 | 🕗 5.1868915558 |
| 🇳  445 | ⭕️  118288120 | 🕗 5.2363333702 |
| 🇳  446 | ⭕️  119085568 | 🕗 5.2728896141 |
| 🇳  447 | ⭕️  119886592 | 🕗 5.3395667076 |
| 🇳  448 | ⭕️  120691200 | 🕗 5.3689684868 |
| 🇳  449 | ⭕️  121499400 | 🕗 5.4454574585 |
| 🇳  450 | ⭕️  122311200 | 🕗 5.4567236900 |
| 🇳  451 | ⭕️  123126608 | 🕗 5.5140542984 |
| 🇳  452 | ⭕️  123945632 | 🕗 5.5848412514 |
| 🇳  453 | ⭕️  124768280 | 🕗 5.6180467606 |
| 🇳  454 | ⭕️  125594560 | 🕗 5.7027635574 |
| 🇳  455 | ⭕️  126424480 | 🕗 5.7007894516 |
| 🇳  456 | ⭕️  127258048 | 🕗 5.7667279243 |
| 🇳  457 | ⭕️  128095272 | 🕗 5.8025741577 |
| 🇳  458 | ⭕️  128936160 | 🕗 5.8558149338 |
| 🇳  459 | ⭕️  129780720 | 🕗 5.9396958351 |
| 🇳  460 | ⭕️  130628960 | 🕗 5.9576253891 |
| 🇳  461 | ⭕️  131480888 | 🕗 5.9992723465 |
| 🇳  462 | ⭕️  132336512 | 🕗 6.0377497673 |
| 🇳  463 | ⭕️  133195840 | 🕗 6.0959258080 |
| 🇳  464 | ⭕️  134058880 | 🕗 6.1883983612 |
| 🇳  465 | ⭕️  134925640 | 🕗 6.2206907272 |
| 🇳  466 | ⭕️  135796128 | 🕗 6.2725119591 |
| 🇳  467 | ⭕️  136670352 | 🕗 6.2992658615 |
| 🇳  468 | ⭕️  137548320 | 🕗 6.3347177505 |
| 🇳  469 | ⭕️  138430040 | 🕗 6.4146580696 |
| 🇳  470 | ⭕️  139315520 | 🕗 6.4513297081 |
| 🇳  471 | ⭕️  140204768 | 🕗 6.5426158905 |
| 🇳  472 | ⭕️  141097792 | 🕗 6.5523233414 |
| 🇳  473 | ⭕️  141994600 | 🕗 6.6255145073 |
| 🇳  474 | ⭕️  142895200 | 🕗 6.6785917282 |
| 🇳  475 | ⭕️  143799600 | 🕗 6.7110738754 |
| 🇳  476 | ⭕️  144707808 | 🕗 6.8051223755 |
| 🇳  477 | ⭕️  145619832 | 🕗 6.8102507591 |
| 🇳  478 | ⭕️  146535680 | 🕗 6.8606562614 |
| 🇳  479 | ⭕️  147455360 | 🕗 6.9208254814 |
| 🇳  480 | ⭕️  148378880 | 🕗 6.9970793724 |
| 🇳  481 | ⭕️  149306248 | 🕗 7.0476222038 |
| 🇳  482 | ⭕️  150237472 | 🕗 7.1175599098 |
| 🇳  483 | ⭕️  151172560 | 🕗 7.1777501106 |
| 🇳  484 | ⭕️  152111520 | 🕗 7.2448754311 |
| 🇳  485 | ⭕️  153054360 | 🕗 7.2952923775 |
| 🇳  486 | ⭕️  154001088 | 🕗 7.3523969650 |
| 🇳  487 | ⭕️  154951712 | 🕗 7.4165315628 |
| 🇳  488 | ⭕️  155906240 | 🕗 7.4655795097 |
| 🇳  489 | ⭕️  156864680 | 🕗 7.5236325264 |
| 🇳  490 | ⭕️  157827040 | 🕗 7.5819687843 |
| 🇳  491 | ⭕️  158793328 | 🕗 7.6839604378 |
| 🇳  492 | ⭕️  159763552 | 🕗 7.7059221268 |
| 🇳  493 | ⭕️  160737720 | 🕗 7.7312030792 |
| 🇳  494 | ⭕️  161715840 | 🕗 7.8410520554 |
| 🇳  495 | ⭕️  162697920 | 🕗 7.8767709732 |
| 🇳  496 | ⭕️  163683968 | 🕗 7.9313440323 |
| 🇳  497 | ⭕️  164673992 | 🕗 8.0396604538 |
| 🇳  498 | ⭕️  165668000 | 🕗 8.0553207397 |
| 🇳  499 | ⭕️  166666000 | 🕗 8.1096744537 |
| 🇳  500 | ⭕️  167668000 | 🕗 8.1465568542 |

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
