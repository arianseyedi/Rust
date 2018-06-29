# Rust
This repo is dedicated to my Rust mini projects.

### Intro:
[Rust](https://www.rust-lang.org/en-US/ "Rust Programming Language") is an amazing programming language officially announced in 2010 and is gaining ground pretty fast. Here are some fun quick facts about Rust:
- Grew out of a personal project by Mozilla employee Graydon Hoare in 2006!
- Mozilla began sponsoring the project in 2009.    
- Rust 1.0, the first stable release, arrived on May 15, 2015.  
- Used by Mozilla in collaboration with Samsung to create a parallel web browsing engine.  
One of the Rust's most unique features is that it is memory safe. No more dangling pointers, null pointers, or memory leaks! This unique awesome feature makes Rust a great system programming language. Although imparative, Rust does support pattern matching which makes it almost as cool as Haskell! Actually many feautures of Rust are inspired by it. Last but not least, be **super** aware that Rust won't make you get away with implicit type conversions as it is **strictly** statically typed. 
      
I would like to thank professor [Alex Ufkes](https://ca.linkedin.com/in/alex-ufkes-37154844 "His Linked In Page")
 for teaching an awesome course in Summer 2018 on Comparative Programming Languages where I have got the above information from directly. I learned a great deal from him, needless to say. The projects 1 and 2 are from this course. 

### 1. Rust starter mini-project:

* Here is what I have implemented:   
    - Write a Rust function that accepts two Strings (or string slices) as input, and returns the concatenation of the two.
    - Write a Rust function that accepts an array of floating point values, and a tuple containing an upper and lower bound. Return the average of the values in the slice defined by the upper and lower bounds. Assume the tuple contains valid bounds. No need to error check.
    - Write a Rust signum function. It should accept an array of integers, and change the elements in the array values to -1, 0, or 1 depending on the sign of the elements in the argument array. Hint: You’re not returning a new array, you’re changing the original.


### 2. Poker Hand Evaluator: 
 * This was part of my end-of-term project. I implemented a method that evaluates two poker hands of 5. It uses basic poker ranks to distinguish different ranks, and in an event of a tie (i.e. two hands of the same rank, e.g. Straight) it looks at the high cards or high pairs until one hands is a winner. Identical hands result in *hand1* as the winner.
