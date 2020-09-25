
<img src="./README_images/hibou_banner_v2.svg" alt="hibou banner" width="750">

# HIBOU LABEL

HIBOU (for Holistic Interaction Behavioral Oracle Utility) provides utilities for the analysis of traces and 
multi-traces collected from the execution of Distributed Systems against interaction models.

This present version "hibou_label" treats labelled interaction models.

This piece of software has been developed as part of my PhD thesis in 2018-2020 at the 
[CentraleSupelec](https://www.centralesupelec.fr/)
engineering school
(part of Université Paris-Saclay) 
in collaboration with the 
[CEA](http://www.cea.fr/) (Commissariat à l'énergie atomique et aux énergies alternatives).

We described our approach in the following paper: 
"[Revisiting Semantics of Interactions for Trace Validity Analysis](https://link.springer.com/chapter/10.1007%2F978-3-030-45234-6_24)"
which was published in the 23rd International Conference on Fundamental Approaches to Software Engineering
i.e. the 2020 edition of the FASE conference, which is part of ETAPS (European joint conferences on Theory And Practice of Software).

The theoretical background of this version of HIBOU is described in [this paper](https://arxiv.org/abs/2009.01777)
(currently available on Arxiv).

If you are interested in the Coq proof associated with [this paper](https://arxiv.org/abs/2009.01777), please click on this 
[link](https://erwanm974.github.io/coq_ictss_2020/) or visit the following
[repository](https://github.com/erwanM974/coq_ictss_2020).

# Entry language : Interactions (.hsf files)

Interaction models are specified with .hsf (Hibou Specification File) files.
The figure below illustrates:
- on the left the model of the interaction as a binary tree (mathematical model)
- in the middle the encoding using the entry langage of HIBOU (PEG grammar)
- on the right the resulting sequence diagram as drawn by HIBOU  

<img src="./README_images/entry_schema.png" alt="schema interactions" width="750">

## Signature Declaration

The signature of the interaction model is declared in the "@message" and "@lifeline" sections of the .hsf file.
It suffices then to list the different message names and lifeline names that will be used in the model.

## Interaction Term

Interactions are terms of a formal language, that can be specified using a simple and intuitive inductive language.
Those terms are build inductively from the composition of basic buildings blocks using specific operators.

### Basic building blocks

<img src="./README_images/building_blocks.png" alt="building blocks" width="600">

The most basic interactions can either be:
- the empty interaction, that specify an absence of observable behavior and which is encoded with "o" or "∅"
- the reception of a message "m" on a lifeline "a" from the environment, which is encoded with "m -> a"
- the emission of a message "m" from lifeline "a" to the environment, which is encoded using "a -- m ->|"
- the passing of a message "m" from lifeline "a" to lifeline "b", which is encoded with "a -- m -> b" (here message "m" is passed asynchronously from lifeline "a" to "b")
- an asynchronous broadcast, which can be encoded with "a -- m -> (b,c,...)"

### Operators

Interaction terms can be composed inductively using some operators so as to build more complex interaction terms.
We define the following operators:
- "strict", "seq" and "par", which are the "scheduling operators"
- "alt" which is the "alternative operator"
- "loop_strict", "loop_seq" and "loop_par" which are "repetition operators"

#### Scheduling operators
Scheduling operators specify how the execution of different sub-interactions can be scheduled w.r.t. one another.
Given sub-interactions "i1", "i2", ..., and "in", and given a scheduling operator "f",
we encode using "f(i1,i2,...,in)" the interaction which results from the scheduling with "f" of those sub-interactions.
We allow n-ary expressions in the entry language but the interaction term is constructed as a binary tree in hibou.

- the "strict" operator specifies strict sequencing i.e. preceding sub-interactions must be executed entirely before any following interaction can be
- the "seq" operator specifies weak sequencing i.e. a strict scheduling is only enforced between actions occurring on the same lifeline 
- the "par" operator allow any interleaving of the executions of sub-interactions

#### Alternative operator

The "alt" operator specified exclusive alternative choice between the execution of sub-interactions. As for the scheduling operators, we allow n-ary expressions.

#### Repetition operators 

Loop operators are unary operators that represent the repetition of a given sub-interaction,
each repeated behavior being sequenced w.r.t. the others using one of the 3 scheduling operators.

For instance "loopseq(m->a)" is equivalent to the infinite alternative "alt(∅,m->a,seq(m->a,m->a),...)".

## Process options

Additionally, one can specify in the header of a .hsf file 
a number of options that will then be used if this .hsf file is exploited 
in some algorithmic process
("explore" or "analyze" command).

In the example below, a Depth First Search exploration strategy 
is specified (by default it is Breadth First Search).

We can specify that we want algorithmic treatments of this .hsf file to be logged with the "loggers" attribute.
In this build only a "graphic" logger exists. 
It will create an image file (a .png file with the same name as the .hsf file) describing the treatment.

Finally, we can specify a number of filters that will limit the exploration 
of graphs in algoritmic treatments  in 
different ways.
- "max_depth" limits the depth of the explored graph
- "max_loop_depth" limits the cumulative number of loop instances that can be unfolded in a given execution
- "max_node_number" limits the number of nodes in the explored graph

<img src="./README_images/options.png" alt="hibou options" width="550">


# Entry language : traces & multi-traces (.htf files)

When a distributed system is executed, we can collect logs on the external interfaces of its sub-system so as to have a
trace of the events that were observed during the execution. We consider two kinds of events that may occur on the interface
of sub-systems: emissions and receptions of messages.

As the lifelines of an interaction model represent the sub-systems of the distributed system modelled by the interaction,
we can understand those collected logs - called traces - as sequences of words "a!m" or "a?m" - called actions - where "a" is a lifeline and "m" a message.

A centralized trace is a sequence of any such action unregarding of the lifelines on which they occur. 
Below is given, in the syntax accepted by HIBOU, such a centralized trace. 
This trace must be specified in a ".htf" file, which stands for Hibou Trace File.
Here, the "[#all]" signifies that the trace is defined over all lifelines.

In [this paper](https://link.springer.com/chapter/10.1007%2F978-3-030-45234-6_24), we describe our approach for the analysis of centralized traces.

<img src="./README_images/global_trace2_htf.png" alt="global trace" width="560">

However, it is not always possible to collect such centralized traces. 
More often that not, we have a local log/trace for each sub-system of the distributed system.
As such, we use the notion of multi-trace.

Multi-traces are sets of traces called its components. 
Each component is defined over a subset of lifelines called a co-localization that is disjoint to that of any other component.

In [this other paper](https://arxiv.org/abs/2009.01777), we describe our approach for the analysis of multi-traces.
In it, we restricted co-localizations to singletons in order to simplify the presentation.
However, our approach still works when considering the more general case of co-localizations.

On the example below is given an example of .htf file which defines a multi-trace composed of 2 components:
- on the co-localization of the 2 lifelines "a" and "b", the local trace "b!m.a!m" has been logged
- on the localization of the "c" lifeline, the local trace "c?m.c?m" has been logged

<img src="./README_images/htf.png" alt="co-localized multi-trace" width="400">

Let us note that we can also analyze global traces simply by defining a multi-trace with a single component as is done below.
Here we used the "#all" keyword to state that this component is defined over all the lifelines defined in "@lifeline".

<img src="./README_images/global_trace1_htf.png" alt="global co-localization" width="650">

We can also use the "#any" keyword to state that a given multi-trace component is defined over all the lifelines that appear in the subsequent trace definition.
For example below is defined a multi-trace that is the same than the one in our first example.
The first component is defined over lifelines "a" and "b", and the second over lifeline "c".

<img src="./README_images/htf_bis.png" alt="global trace" width="400">


# Command Line Interface

The functionnalities of HIBOU are accessed via a Command Line Interface (CLI).
For the version presented in this repository, you will have to launch the executable "hibou_label" (or "hibou_label.exe" on Windows OSs) from a terminal.

## Help

The HIBOU executable provides a small documentation about its interface. This can be accessed by tying "hibou_label help" or "hibou_label -h".
It explaines that there are 4 sub-commands in HIBOU:
- "draw" (to be used as "hibou_label draw <.hsf file>"), which draws as a sequence diagram a given interaction
- "analyze" (to be used as "hibou_label analyze <.hsf file> <.htf file>"), which analyze a multi-trace w.r.t. an interaction
- "explore" (to be used as "hibou_label explore <.hsf file>"), which computes (partially or totally if possible) the semantics of a given interaction
- "help", which is the present help

Each sub-command also has its dedicated documentation:
- "hibou_label draw -h" provides a small documentation for the drawing utility
- "hibou_label analyze -h" provides a small documentation for the "analyze" sub-command
- "hibou_label explore -h" provides a small documentation for the "explore" sub-command

In the following, we provide more details about those sub-commands.

## Draw

Diagrams, such as the one on the previous images can be drawn using the "hibou draw" command as exemplified below.

<img src="./README_images/draw_command_building_blocks.png" alt="draw command" width="450">

<img src="./README_images/building_blocks.png" alt="building blocks" width="450">


## Analyze

The "analyze" sub-command of HIBOU can analyse multi-traces w.r.t. interactions. 
For any multi-trace and any interaction, it returns a verdict about the conformity of the multi-trace w.r.t. the interaction. 
This verdict is either "Pass" or "Fail". 
We have proven in [this Coq proof](https://erwanm974.github.io/coq_ictss_2020/) that the verdict "Pass" is equivalent to the membership of the multi-trace to the semantics of the interaction.


### Analysing multi-traces  

Our approach to analysis consists in consuming one-by-one the head elements of the multi-trace 
i.e. the actions which are at the beginning of its trace components.

To do so, we compare them with the actions that are immediately executable within the initial interaction model.
If there is a matche between such an action - called a frontier action - and a trace action,
we can compute another interaction model which describes what can happen in the remainder of the execution.
We then repeat the process with this new interaction model and the new multi-trace on which we have removed the consumed action.

Consequently, any given analysis opens-up paths which are successions of couples (interaction*multi-trace).
Each such path terminates either with 
- a "Cov" local verdict, when the multi-trace has been entirely consumed and the interaction can express the empty execution (statically verified on the interaction term)
- an "UnCov" local verdict in the other cases (i.e. either when the consumption of the multi-trace is impossible, or when the multi-trace has been emptied but the interaction cannot express the empty execution)

If there exists a path terminating in "Cov", a global verdict "Pass" is returned. 
The global verdict is "Fail" otherwise.

#### Example 1

Below is given an example analysis, that you can reproduce by using the files from the "examples" folder.
The analysis of this multi-trace yields the "Pass" global verdict.
Let us note that, as we use a "DFS" (Depth-First-Search) heuristics, we have quickly found a path that consumed the entire
multi-trace and we did not need to explore further executions of the initial interaction model.
For instance, you can see that we did not explore the branch starting with the execution and consumption of "c!m4".

<img src="./README_images/analysis_command_1.png" alt="analysis command ex1" width="600">

<img src="./README_images/analysis_1.png" alt="analysis ex1" width="750">

#### Example 2

Below is another example, this time of the analysis of a global trace, and yielding the "Fail" verdict.
You can also reproduce it by using the files from the "examples" folder.

<img src="./README_images/analysis_command_2.png" alt="analysis command ex2" width="600">

<img src="./README_images/analysis_2.png" alt="analysis ex2" width="750">

## Explore

The "explore" command of HIBOU can generate execution trees which illustrate the semantics of a given interaction model.
The exploration of such execution trees can be defined up to certain limits (depth, number of nodes, loop 
instanciations) and up to certain search algorithms.

Below is given an example exploration that you can obtain by tying 
"hibou_label.exe explore example_for_exploration_1.hsf" 
with the files from "examples" folder.

![image info](./README_images/explo1.png)

And a second example that you can obtain by tying 
"hibou_label.exe explore example_for_exploration_2.hsf" 
with the files from "examples" folder.

![image info](./README_images/explo2.png)

## Build

You can build the Rust project with cargo using "cargo build --release".

This will generate an executable in "./target/release".

Or you could download the provided binary for windows.

## Requirements / Dependencies

So as to generate the images of the graphs, you will need to have graphviz installed on your system. 
Graphviz is available at ( https://www.graphviz.org/download/ ).
The "dot" command provided by Graphviz must be in your "PATH" environment variable.

## Examples

All the examples in this README are provided in the "examples" directory as well as the commands used to generate the images above.
