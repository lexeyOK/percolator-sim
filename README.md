# Percolation simulator
This is percolation [^1] simulation in rust.
Will create `output.png` file in folder it was executed

Basic percolation is achieved like this:
- take grid graph of sides NxM
- remove edges of graph with some probability p
- find connected-components of leftover graph [^2]

This code supposed to output image of colored connected-components similar to that found in the video[^3].

# Examples of different visualizations. 
first one is modulo times some primes in rgb and second one is grayscale in order of occurrence  

![colored](https://user-images.githubusercontent.com/35109763/188222336-da25039f-48f6-4ab6-9adb-424f9d7c829a.png)

![output](https://user-images.githubusercontent.com/35109763/188223548-bfe75954-ed34-42c0-8414-a01c9c16f2fa.png)

### Sources 
[^3]: Inspired by [Percolation a Mathematical Phase Transition](https://youtu.be/a-767WnbaCQ)
[^1]:Wikipedia article about percolation: [Percolation theory](https://en.wikipedia.org/wiki/Percolation_theory)
[^2]: Algorithm for labeling connected-components: [Connected-component labeling Two-pass](https://en.wikipedia.org/wiki/Connected-component_labeling#Two-pass)