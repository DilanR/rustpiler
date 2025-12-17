# Reflections

<!--toc:start-->

- [Reflections](#reflections)
  - [Lab 1](#lab-1)
  - [Lab 2](#lab-2)
  - [Lab 3](#lab-3)
  - [Lab 4](#lab-4)
  - [Lab 5](#lab-5)
  - [Lab 6](#lab-6)
- [Conclusion](#conclusion)
  <!--toc:end-->

## Lab 1

Because of my previous experience with rust mostly coming from D7020E and personal project this lab was straight forward and easy to complete. I did however appreciate and need a refresher on the borrow checker and Copy and Clone traits

## Lab 2

This lab was interesting and somewhat difficult, reading through the regex docs helped a lot and a refresher on some aspects of binary trees made a solution viable. Also learning more about parsing through syn was very useful and I will take that with me in the future.

## Lab 3

I found this lab the most difficult particularly because of the theory regarding behind the ebnf and sos, reading about sequent calculus made sos much easier to grasp. Also because of the essence of the lab, being creating an VM I was forced to better understand the underlying theories of computations. This was very useful.

## Lab 4

I found this lab the easiest to complete, because I could following structure of the of lab 3. To be honest I almost attempted to implement parts of the type checker in lab 3 before remembering "Keep it simple, stupid". As with the lab 3 learning about type inference is something I found very useful.

## Lab 5

In this lab I struggled a lot, I found that my understanding of the mips architecture and microcomputer technology has been neglected. I got stuck with building the frame for functions and keeping my own stack machine in sync. I found the logger in the mips crate very useful. There are a lot of optimizations and extensions I want to implement, particularly implementing multiplication and division. Also I believe forking the mips crate and extending it would be a great exercise.

## Lab 6

Clap is a crate I am familiar with, making the beginning of this lab simple. Next steps was making sure that all parts of the compiler is in scope for the cli, which was easy to facilitate partially because of common trait Eval.

"Keep it simple, Stupid" is really easy to neglect here. But for good reason, there is a lot I want to implement but not only for my cli. mostly to make sure that the vm and codegen is in sync when it comes to features. However this would be difficult mostly because of strings.

Either way I am happy with the work done and I would like to extend my RnR for the January presentation.

# Conclusion

Overall this course was challenging and fun. I have gained experience in parsing, type inference, evaluation and code generation. Some concepts have been intimidating such as the implementing the vm but learning how to make a sos has made it a lot more manageable. In general this course has made more confident as a programmer.
