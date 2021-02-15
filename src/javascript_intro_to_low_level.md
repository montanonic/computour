# Intro to computing machines and programmming languages for javascript devs

## Assumptions of knowledge

Hello there! I hope this interactive guide will be enjoyable for you and as many people as possible. I wanted to let you know up front that my explanations will assume working knowledge of JavaScript, but definitely not intricate knowledge of it. I would definitely love to create more material for people who are just starting out in their programming journey, but this interactive guide is not targetted towards that level of accessibility.

## What this guide covers

In this guide, our main arc will cover the things we need to know about how modern processors so that we can understand how a programming language like JavaScript *really* works. To get there, we're going to cover a lot, and we're going to do it in a way that lets you program and invent alongside me every step of the way. In fact, this guide allows for two parallel codebases, one that is guided through the code I wrote and let's you complete exercises in it, and another where you are free to modify and extend it however you want, and follow along this guide with as much custom code as you're interested in implementing. Switching between both versions will make it easy to follow along, while letting you work out things for yourself whenever you want to. Here's an overview of the things we'll be learning and doing along this journey:;

(Note to self, this list is huge but I find it helpful to organize my current inspiration. I think working this out helps me by having something structured to reference guide along what I learn and teach.)

* What assembly language is, without getting into the any hard-to-read assembly.
* A custom, simple, elegant assembly language that is easy to read and understand, and helps us learn how a lot about how code running on CPUs works. We'll have several iterative versions of it that get more intricate.
* A javascript interpreter for our assembly language that lets us run it (and change it)
* Our own programming language, based on Lisp (because it's really easy to parse). I worked on a really nice UI (seen below: try and edit it!) to make working with our language really nice too! You can edit it as a text language, but also more graphically like that. By the end of this guide you'll know a lot about how I did that, and with my future guide we'll actually implement it so that you'll have all the knowledge you'd need to make something like this yourself!
* A compiler: we'll compile our programming language to our custom assembly language, learning a ton about how things like loops, recursion, functions, objects, methods, and all that jazz is truly implemented.
* Webassembly! We're going to learn webassembly for real, and learn it in the context of already understanding some solid fundamentals of how assembly language works thanks to our custom assembly language.
* Compiling our programming language to webassembly! We'll actually learn two different ways, first we'll learn how to transform our custom assembly language into webassembly, so that we automatically are able to run our programming language in webassembly! (Super cool!). Then we'll learn about optimization by trying to directly compile parts of our language into webassembly, and compare the results of that with the output of transforming our custom assembly.
* This is already a ton, and at this point, we'll begin our second major arc: learning Rust. We'll rewrite our interpreter in Rust and see how it can help us write our code. Secret: our programming language will already have taught us a lot about Rust without us even knowing it!
* We'll learn to compile to Rust itself next, so that we can leverage all of it's code analysis tools to help us solve problems that would be too difficult to do with our current tools. We'll create a toolchain that lets us use Rust to compile our code more safely and efficiently, and also create an interactive debugger/interpretter that lets us debug and walk through our program language, custom assembly, and webassembly all at once! This will be an incredible asset that will make it much more obvious what is really happening.
* Garbage collection: we'll write a couple different styles of garbage collector.
* JavaScript-ish. We'll implement more semantics of JavaScript into our language, prototypal inheritance, proxy objects, the object system, et cetera. But, we're going to make nicer versions of them by default.

## Motivation for this guide

As someone who's only worked web-development jobs, I've been professionally incentivized to focus on that area of knowledge, much of which resides at a fairly high level in the hierarchy of machine code. I feel that, for all I learned, I was always missing a deeper foundation into the lower levels of computing, and that there wasn't much in the way of accessible resources to help me dive deeper in a fun way. I really didn't want to learn C, I really didn't want to learn x86 assembly (the processor architecture a *lot* of modern consumer computers are based off of), I just generally didn't want to stumble in the dark with things that were unfamiliar and didn't seem user-friendly. In other words, I care a lot about ergonomics and ease of use. And I don't think I'm the only one who struggles to dive into code when the toolchains are hard, debugging is difficult, documentation isn't great, et cetera.

If parts of that resonates with your experience, I hope this guide will be a pleasant time. I have written this to be ergonomic and interactable. You can play around with a ton of the code without having to install anything: it will run right in your browser, and save to your localstorage automatically so that you can leave and come back whenever. There's also github integration to backup your save as a gist so you can keep your progress synched across different devices, and it also lets you export your code as gists too.

All of the code that you see will be available, and in the future I'll have a guide that interactively teaches the Rust programming language in ways that will help you to understand practically every detail of how this entire interactive guide was written. That said, we'll still be able to cover a *lot* about how this stuff works; it's just that if you want to fully peak under the hood, you'll need to read some Rust code :).

## How to reach me

You're welcome to email me at montanonic@gmail.com if you'd like to share your thoughts or experiences reading this, or to let me know what type of future guide might be helpful for you. You can also often find me in the two primary Rust discord communities (LINKS HERE).