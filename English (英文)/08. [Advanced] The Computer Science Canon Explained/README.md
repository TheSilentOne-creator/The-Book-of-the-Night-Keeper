# "The Computer Science Canon" Explained · General Introduction

---

## Opening: A Viral Short Video

You may have come across a video like this:

A pure blue background. A person who looks undeniably impressive stands in the center. A deep voice intones:

**"Young one, do you yearn for power?"**  

Then, one after another, thick books with black marble-textured covers rain down from the sky—*Introduction to Algorithms*, *Computer Systems: A Programmer's Perspective*, *Compilers: Principles, Techniques, and Tools*, *TCP/IP Illustrated*... Each one lands with a heavier sense of pressure than the last.

The reason this video went viral is that it perfectly captures a specific emotion: **You want to become stronger, but looking at these books fills you with dread.**

This collection of books has a unified name: *The Computer Science Canon*. Because of its iconic black covers, readers simply call them the **"Black Books."**

The website you are looking at right now is one person's live record of trying to chew through these books, one by one. Before we officially begin, this general introduction aims to answer a few of the most fundamental questions: What are these books actually about? Why should you read them? And—how do you read them without being scared away?

---

## Chapter 1: Before You Open Your First Book—What is "Code"?

Before we talk about computer science, let's discuss a more fundamental question: **What exactly is a computer?**

Many people think a computer is a "very complex appliance." But in reality, the essence of a computer is incredibly simple—**it is a machine that processes information.**

To understand this, there's a masterpiece worth mentioning first: Charles Petzold's ***Code: The Hidden Language of Computer Hardware and Software***. Although this book is not part of the "Black Book" series, it is the perfect introductory read. It asks a single question and spends the entire book answering it:

> **Two kids want to chat across their rooms at night using only flashlights. The flashlights can only be on or off. How can they transmit complex information?**

The answer is: they need a **coding system**. For example, "flash once" means A, "flash twice" means B... and so on. This is the principle behind Morse code.

Following this line of thought, *Code* takes you by the hand, starting with a flashlight, and guides you step-by-step through inventing the telegraph, relays, logic gates, adders, and finally—you discover, to your astonishment, that you have completely "built" a computer. And the only components you used were switches that could represent "on" and "off."

***Code* reveals a core truth: A computer is not magic; it is built up layer by layer of "code." The lowest layer is nothing but 0s and 1s. Each layer establishes new rules based on the layer beneath it, ultimately allowing you to see text, images, and video on your screen.**

Understanding the idea that "coding is just translating information layer by layer" gives you the first key to entering the world of computer science.

**Why mention *Code* in the introduction to the Black Books?**

Because many books in the Black Book series, such as *Computer Systems: A Programmer's Perspective* and *Computer Organization and Design*, are essentially explaining exactly how this system of "layered coding" is built. *Code* is the map; the Black Books are the detailed construction blueprints for each floor. If you are an absolute beginner, reading *Code* first will make the Black Books feel much smoother.

---

## Chapter 2: Then, What is "Computer Science"?

Alright, so now you know a computer is built up layer by layer of codes. So, is the study of these things "computer science"?

Yes, but not entirely.

There is a widespread misunderstanding: *"Computer science is all about studying computers."* This is as absurd as saying *"Astronomy is all about studying telescopes."* Turing Award winner Edsger W. Dijkstra specifically corrected this:

> **"Computer science is no more about computers than astronomy is about telescopes."**

The computer is just a tool. What computer science truly studies is **information itself**—how to represent it, store it, process it, and transmit it. And more importantly: **What are the things a computer can never do? What things are theoretically possible but would take until the end of the universe to compute?**

It pursues two ultimate questions:

1. **Computability**: For a given problem, can it be solved by an algorithm in principle? Are there questions that a computer simply cannot answer, no matter what? In 1936, before the electronic computer even existed, Alan Turing proved mathematically: Yes, there are. This is the "Halting Problem"—you cannot write a universal program to determine whether another program will run forever or eventually stop.
2. **Computational Complexity**: If a problem is solvable, how much time and memory does it require? Can we find a faster way? This is the mathematical foundation behind the phrase you often hear: "This algorithm is O(n²), which is too slow. Can we optimize it to O(n log n)?"

**Computer science is the study of "which problems can be computed, how to compute them faster, and which ones can never be computed at all."**  

---

## Chapter 3: CS and CE Are Two Different Species

Here, we must introduce a concept that beginners often confuse: Computer Science (CS) versus Computer Engineering (CE).

In the simplest terms:

- **CS cares about the "abstract"**: How do you write an algorithm? How is the syntax of a programming language designed? What functions should an operating system provide? These questions don't require knowing how electricity moves.
- **CE cares about the "implementation"**: How is a CPU circuit drawn? How is memory manufactured? How do you handle heat dissipation? These problems must confront the harsh laws of the physical world—electricity has delays, chips get hot, and hard drives can fail.

Think of it this way: **CS is drawing the architectural blueprint; CE is constructing the building on the site.** A wrong blueprint can cause a building to collapse, but the blueprint designer doesn't need to know the grade of the concrete. The construction worker must know the concrete grade, but isn't responsible for deciding if the building's design is beautiful.

What does the book *Code* cover? It perfectly bridges CS and CE together—it starts with the bricks of the building (switches, relays) and goes all the way to how to design the room layouts (logic gates, CPU instructions). It is the bridge connecting both worlds.

---

## Chapter 4: Regarding the Snobbery—What You See Online

You might see some strange "snobbery" in the tech community:

- Some look down on frontend development: "Isn't it just writing web pages? Does that even count as programming?"
- Some look down on hardware engineers: "Who touches registers anymore? No future."
- Yet others look down on application developers: "You call yourself a programmer without ever reading an operating system kernel?"

The essence behind these arguments is **the confusion between CS and CE.**

In fact, this snobbery can be drawn as two completely opposite lines:

- **In the Engineering domain (closer to the user = higher status)**:
    > App developer → Backend API developer → Database maintenance → Embedded systems developer
- **In the Science domain (closer to the bottom layer = higher status)**:
    > Computational theory researcher → Compiler developer → Operating systems developer → Application developer

Do you see it? The same person—for example, an App developer—is at the top of one chain and at the bottom of the other. What does this show? **The snobbery itself is absurd.** It is simply two circles measuring people with different yardsticks.

Truly exceptional people aren't even on these chains. *Code*'s author, Charles Petzold, is not a Turing Award laureate, but he enabled countless people to understand computers for the first time. Linus Torvalds, who created Linux and Git, can write a version control tool at the application layer and an operating system at the kernel layer. He is enshrined by both chains, not because he ranks high on either, but because **he has connected the two chains together.**

The same goes for this collection of Black Books. It contains both pure CS works like *Introduction to Algorithms* and *Introduction to the Theory of Computation*, as well as pure CE works like *Computer Organization and Design* and *Computer Architecture*. You don't need to pick a side. What you need to do is connect them.

---

## Chapter 5: What Can This Set of Black Books Actually Give You?

### Layer One: Knowledge

You will learn concrete technical principles. Why can a hash table instantly find data? Why does TCP absolutely require a three-way handshake? How does a compiler turn a single line of `printf` into instructions the CPU can understand? How does an operating system pretend it has infinite memory? This knowledge helps you pass interviews, perform your job, and know which direction to look for answers when solving problems.

### Layer Two: A Way of Thinking

Knowledge can become obsolete, but a way of thinking will not.

- **Layered Thinking**: After studying these books, you won't look at any computer problem as a "tangled mess." You will automatically judge: "This is a network layer issue, not an application layer issue." "This bug was introduced during the compiler optimization phase." This layered diagnostic ability is the dividing line between an expert and an ordinary person.

- **Trade-off Thinking**: There are no silver bullets in computer science. Hash tables are fast but consume more memory; linked lists save memory but are slow. TCP is reliable but has high latency; UDP is fast but might lose packets. After studying these books, you will no longer ask "Which is the best?", but rather "Which is more suitable in this specific scenario?"

- **Limit Thinking**: The theory of computation tells you that some problems are simply unsolvable, and some, while solvable, have a practical time cost so high it's infeasible. Knowing what is impossible prevents you from wasting your career on the impossible.

### Layer Three: Aesthetic Taste

This is the most hidden gift. When you have read enough classics, you will gradually develop a form of "taste"—the ability to distinguish an elegant design from a makeshift patch. This aesthetic sense will seep into every line of code you write.

---

## Chapter 6: So Many Books, Where to Start?

This collection is currently known to hold over six hundred volumes. Don't try to read them all—and no one needs to. They are not novels; they are reference books and textbooks. The correct way to use them is: **Pick up the one you need, when you need it.**

Below is a navigational map to help you find the first book to open based on your current position.

### Step 1: Determine Your Stage

- **Absolute Beginner**: Read *Code* first to build the intuition that "a computer is built up layer by layer of code." Then come back to this map.
- **Have a foundation, want systematic study**: Start with *Computer Science: An Overview* to build a global view, then dive deeper following the categorized routes below.

### Step 2: A CS Spectrum Diagram

I have arranged the core books along a spectrum from "abstract to concrete." The higher up, the more it leans towards mathematics and theory (CS); the lower down, the more it leans towards hardware and engineering (CE). You can choose where to jump in based on your interests.

**🌌 The Theory Layer (Pure CS Mathematics)**  

- ***Discrete Mathematics and Its Applications*** — The "mother tongue" of computer science. Logic, sets, graph theory, and proof methods. The theoretical foundation for everything.
- ***Concrete Mathematics*** — By the legendary Donald Knuth. The hardcore math needed to deal with algorithmic complexity is right here.
- ***Introduction to Algorithms*** — The "Bible" of the algorithms field. Data structures, sorting, graph algorithms, NP-completeness theory.
- ***Introduction to the Theory of Computation*** — The standard textbook on computability and computational complexity. The Halting Problem, P vs NP, it's all here.

**📜 The Software and Abstraction Layer**  

- ***Compilers: Principles, Techniques, and Tools*** — Commonly known as the "Dragon Book." Teaches you how to translate high-level languages into instructions a machine can understand.
- ***Structure and Interpretation of Computer Programs (SICP)*** — Uses the Scheme language to teach you "the essence of programming."
- ***Database System Concepts*** — Relational models, transactions, concurrency control.

**📡 The Network and Communication Layer**  

- ***Computer Networking: A Top-Down Approach*** — Starts from the application layer where you browse web pages, then peels back each layer downward until the physical layer.
- ***TCP/IP Illustrated*** — More hardcore than the previous book, delving deeply into every field of the protocol and kernel implementations.

**⚙️ The System and Architecture Layer (The Key Zone Connecting CS and CE)**  

- ***Computer Systems: A Programmer's Perspective (CS:APP)*** — The bridge connecting software and hardware. After studying it, the code you write no longer runs inside a "black box."
- ***Modern Operating Systems*** — Processes, threads, deadlocks, virtual memory, file systems.
- ***Operating System Concepts*** — A renowned OS textbook alongside the previous one, commonly known as the "Dinosaur Book."

**🔧 The Hardware and Engineering Layer (Pure CE Implementation)**  

- ***Computer Organization and Design*** — The structure of the CPU, pipelining, memory hierarchy. An introduction to RISC-V and MIPS architectures.
- ***Computer Architecture: A Quantitative Approach*** — Essential advanced reading in the computer architecture field, using data-driven performance optimization.
- ***Digital Design and Computer Architecture*** — From logic gates to processors, leaning towards circuit and logic design.

### Step 3: Choose Your First Book Based on Your Goal

| Your Identity / Goal | Recommended First Book | Reason |
| :--- | :--- | :--- |
| Absolute beginner, wants to build intuition first | *Code* | Requires zero background. Starts with a flashlight and leads you to "invent" a computer. |
| University freshman, needs a global view | *Computer Science: An Overview* | Introduces the complete computing system using a layered structure, preventing you from "missing the forest for the trees." |
| Wants to get hands-on with programming ASAP | *The C Programming Language* | A textbook written by the father of C. It's thin but incredibly precise. Makes reading OS and networking books much easier afterwards. |
| Wants to lay a foundation for algorithmic competitions | *Introduction to Algorithms* | No need to read it all. Deeply study the core chapters on data structures, sorting, and graph algorithms; use it as a reference book. |
| Wants to truly "understand" computer systems | *Computer Systems: A Programmer's Perspective* | The bridge book connecting software and hardware. After reading it, your view of programs will be refreshed. |
| Interested in hacking, attack/defense, and cybersecurity | *Computer Networking: A Top-Down Approach* + *Computer Systems: A Programmer's Perspective* | Security is the shadow cast by the architecture. First, understand how the network and system operate, then talk about offense and defense. |
| Wants to do AI and machine learning | *Introduction to Algorithms* + mathematical foundation (linear algebra, probability theory) | The bottleneck of AI ultimately lies in math and algorithms. Tools change, fundamental principles do not. |
| Wants to work on hardware and chip design | *Computer Organization and Design* | A classic by Patterson and Hennessy. From instruction sets to pipelining, step-by-step, teaches you how a CPU is designed. |
| Pure curiosity, wants to feel the beauty of computing | *Code* or *Structure and Interpretation of Computer Programs* | The former makes you exclaim, "So that's how it is!" The latter makes you rethink, "What exactly is programming?" |

---

## Epilogue: So, "Do You Yearn for Power?"

Let's return to that short video from the beginning.

The reason its line is so powerful is that it inadvertently reveals a truth: **The power is not in the video; it is within those books. And the action of opening them can only be done by you.**

This website is the record of one person opening these books. Every reading note is something an ordinary reader discovered within a particular Black Book. If you are also reading these books, I hope this place can help you a little. If you haven't started yet, now is the best time.

**You don't need to finish them all. You just need to pick up the one you need most, when you need it.**

The power is right there.
